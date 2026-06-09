use crate::auth::{extract_token, validate_token};
use crate::ecpay::{EcpayClient, EcpayConfig, EcpayReturnData};
use crate::error::AppError;
use crate::models::*;
use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{Html, IntoResponse},
    Json,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::collections::BTreeMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub category_id: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub category_id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddCartRequest {
    pub product_id: String,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCartRequest {
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub shipping_address: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub rating: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePaymentRequest {
    pub order_id: String,
    pub card_number: String,
    pub card_brand: String,
    pub expiry_month: String,
    pub expiry_year: String,
    pub cvv: String,
}

pub async fn get_categories(
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<ApiResponse<Vec<Category>>>, AppError> {
    let categories: Vec<Category> =
        sqlx::query_as("SELECT * FROM categories ORDER BY name")
            .fetch_all(&*pool)
            .await?;

    Ok(Json(ApiResponse::success(categories)))
}

pub async fn get_products(
    State(pool): State<Arc<SqlitePool>>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = (page - 1) * limit;

    let products: Vec<Product> = if let Some(ref search) = query.search {
        let search_pattern = format!("%{}%", search);
        sqlx::query_as(
            "SELECT * FROM products WHERE name LIKE ? OR description LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(&*pool)
        .await?
    } else if let Some(ref cat_id) = query.category_id {
        sqlx::query_as(
            "SELECT * FROM products WHERE category_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(cat_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&*pool)
        .await?
    } else {
        sqlx::query_as("SELECT * FROM products ORDER BY created_at DESC LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(&*pool)
            .await?
    };

    let response: Vec<ProductResponse> = products.into_iter().map(|p| p.into()).collect();

    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_product(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ProductResponse>>, AppError> {
    let product: Option<Product> =
        sqlx::query_as("SELECT * FROM products WHERE id = ?")
            .bind(&id)
            .fetch_optional(&*pool)
            .await?;

    match product {
        Some(p) => Ok(Json(ApiResponse::success(p.into()))),
        None => Err(AppError::NotFound("Product not found".to_string())),
    }
}

pub async fn create_product(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Json(payload): Json<CreateProductRequest>,
) -> Result<Json<ApiResponse<ProductResponse>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO products (id, seller_id, category_id, name, description, price, stock, image_url, rating, sold_count, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0.0, 0, ?, ?)",
    )
    .bind(&id)
    .bind(&user.id)
    .bind(&payload.category_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.price)
    .bind(&payload.stock)
    .bind(&payload.image_url)
    .bind(&now)
    .bind(&now)
    .execute(&*pool)
    .await?;

    let product = Product {
        id,
        seller_id: user.id,
        category_id: payload.category_id,
        name: payload.name,
        description: payload.description,
        price: payload.price,
        stock: payload.stock,
        image_url: payload.image_url,
        rating: 0.0,
        sold_count: 0,
        created_at: now.clone(),
        updated_at: now,
    };

    Ok(Json(ApiResponse::success(product.into())))
}

pub async fn update_product(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Json<ApiResponse<ProductResponse>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let product: Option<Product> =
        sqlx::query_as("SELECT * FROM products WHERE id = ?")
            .bind(&id)
            .fetch_optional(&*pool)
            .await?;

    let product = product.ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

    if product.seller_id != user.id {
        return Err(AppError::Unauthorized);
    }

    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query("UPDATE products SET name = COALESCE(?, name), description = COALESCE(?, description), price = COALESCE(?, price), stock = COALESCE(?, stock), image_url = COALESCE(?, image_url), updated_at = ? WHERE id = ?")
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(&payload.price)
        .bind(&payload.stock)
        .bind(&payload.image_url)
        .bind(&now)
        .bind(&id)
        .execute(&*pool)
        .await?;

    let updated: Product = sqlx::query_as("SELECT * FROM products WHERE id = ?")
        .bind(&id)
        .fetch_one(&*pool)
        .await?;

    Ok(Json(ApiResponse::success(updated.into())))
}

pub async fn delete_product(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let product: Option<Product> =
        sqlx::query_as("SELECT * FROM products WHERE id = ?")
            .bind(&id)
            .fetch_optional(&*pool)
            .await?;

    let product = product.ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

    if product.seller_id != user.id {
        return Err(AppError::Unauthorized);
    }

    sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await?;

    Ok(Json(ApiResponse::<()>::message("Product deleted")))
}

pub async fn get_cart(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<Vec<CartItemResponse>>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let cart_items: Vec<CartItem> = sqlx::query_as(
        "SELECT id, user_id, product_id, quantity, created_at, updated_at FROM cart_items WHERE user_id = ?",
    )
    .bind(&user.id)
    .fetch_all(&*pool)
    .await?;

    let mut response = Vec::new();
    for item in cart_items {
        let product: Product = sqlx::query_as("SELECT * FROM products WHERE id = ?")
            .bind(&item.product_id)
            .fetch_one(&*pool)
            .await?;

        let p: ProductResponse = product.into();
        response.push(CartItemResponse {
            id: item.id,
            product: p.clone(),
            quantity: item.quantity,
            subtotal: p.price * item.quantity as f64,
        });
    }

    Ok(Json(ApiResponse::success(response)))
}

pub async fn add_to_cart(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Json(payload): Json<AddCartRequest>,
) -> Result<Json<ApiResponse<CartItemResponse>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let existing: Option<CartItem> =
        sqlx::query_as("SELECT * FROM cart_items WHERE user_id = ? AND product_id = ?")
            .bind(&user.id)
            .bind(&payload.product_id)
            .fetch_optional(&*pool)
            .await?;

    let (id, quantity) = if let Some(item) = existing {
        let new_qty = item.quantity + payload.quantity;
        sqlx::query("UPDATE cart_items SET quantity = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(new_qty)
            .bind(&item.id)
            .execute(&*pool)
            .await?;
        (item.id, new_qty)
    } else {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO cart_items (id, user_id, product_id, quantity, created_at, updated_at) VALUES (?, ?, ?, ?, datetime('now'), datetime('now'))",
        )
        .bind(&id)
        .bind(&user.id)
        .bind(&payload.product_id)
        .bind(&payload.quantity)
        .execute(&*pool)
        .await?;
        (id, payload.quantity)
    };

    let product: Product = sqlx::query_as("SELECT * FROM products WHERE id = ?")
        .bind(&payload.product_id)
        .fetch_one(&*pool)
        .await?;

    let p: ProductResponse = product.into();
    let subtotal = p.price * quantity as f64;

    Ok(Json(ApiResponse::success(CartItemResponse {
        id,
        product: p,
        quantity,
        subtotal,
    })))
}

pub async fn update_cart_item(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<UpdateCartRequest>,
) -> Result<Json<ApiResponse<CartItemResponse>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let item: Option<CartItem> = sqlx::query_as("SELECT * FROM cart_items WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user.id)
        .fetch_optional(&*pool)
        .await?;

    let item = item.ok_or_else(|| AppError::NotFound("Cart item not found".to_string()))?;

    sqlx::query("UPDATE cart_items SET quantity = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(&payload.quantity)
        .bind(&id)
        .execute(&*pool)
        .await?;

    let product: Product = sqlx::query_as("SELECT * FROM products WHERE id = ?")
        .bind(&item.product_id)
        .fetch_one(&*pool)
        .await?;

    let p: ProductResponse = product.into();
    let subtotal = p.price * payload.quantity as f64;

    Ok(Json(ApiResponse::success(CartItemResponse {
        id: item.id,
        product: p,
        quantity: payload.quantity,
        subtotal,
    })))
}

pub async fn remove_from_cart(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let result = sqlx::query("DELETE FROM cart_items WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user.id)
        .execute(&*pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Cart item not found".to_string()));
    }

    Ok(Json(ApiResponse::<()>::message("Item removed from cart")))
}

pub async fn create_order(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<ApiResponse<OrderResponse>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let cart_items: Vec<(String, i32, f64)> = sqlx::query_as(
        "SELECT c.product_id, c.quantity, p.price
         FROM cart_items c
         JOIN products p ON c.product_id = p.id
         WHERE c.user_id = ?",
    )
    .bind(&user.id)
    .fetch_all(&*pool)
    .await?;

    if cart_items.is_empty() {
        return Err(AppError::BadRequest("Cart is empty".to_string()));
    }

    let total: f64 = cart_items.iter().map(|(_, q, p)| *p as f64 * *q as f64).sum();

    let order_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO orders (id, user_id, total_amount, status, shipping_address, created_at, updated_at) VALUES (?, ?, ?, 'pending', ?, ?, ?)",
    )
    .bind(&order_id)
    .bind(&user.id)
    .bind(total)
    .bind(&payload.shipping_address)
    .bind(&now)
    .bind(&now)
    .execute(&*pool)
    .await?;

    let mut order_items = Vec::new();

    for (product_id, quantity, price) in cart_items {
        let item_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO order_items (id, order_id, product_id, quantity, price) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&item_id)
        .bind(&order_id)
        .bind(&product_id)
        .bind(&quantity)
        .bind(&price)
        .execute(&*pool)
        .await?;

        sqlx::query("UPDATE products SET stock = stock - ?, sold_count = sold_count + ? WHERE id = ?")
            .bind(&quantity)
            .bind(&quantity)
            .bind(&product_id)
            .execute(&*pool)
            .await?;

        let product: Product = sqlx::query_as("SELECT * FROM products WHERE id = ?")
            .bind(&product_id)
            .fetch_one(&*pool)
            .await?;

        order_items.push(OrderItemResponse {
            product: product.into(),
            quantity,
            price,
        });
    }

    sqlx::query("DELETE FROM cart_items WHERE user_id = ?")
        .bind(&user.id)
        .execute(&*pool)
        .await?;

    Ok(Json(ApiResponse::success(OrderResponse {
        id: order_id,
        items: order_items,
        total_amount: total,
        status: "pending".to_string(),
        shipping_address: Some(payload.shipping_address),
        created_at: now,
    })))
}

pub async fn get_orders(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<Vec<OrderResponse>>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let orders: Vec<Order> = sqlx::query_as("SELECT * FROM orders WHERE user_id = ? ORDER BY created_at DESC")
        .bind(&user.id)
        .fetch_all(&*pool)
        .await?;

    let mut response = Vec::new();

    for order in orders {
        let items: Vec<OrderItem> = sqlx::query_as("SELECT * FROM order_items WHERE order_id = ?")
            .bind(&order.id)
            .fetch_all(&*pool)
            .await?;

        let mut order_items = Vec::new();
        for item in items {
            let product: Product = sqlx::query_as("SELECT * FROM products WHERE id = ?")
                .bind(&item.product_id)
                .fetch_one(&*pool)
                .await?;

            order_items.push(OrderItemResponse {
                product: product.into(),
                quantity: item.quantity,
                price: item.price,
            });
        }

        response.push(OrderResponse {
            id: order.id,
            items: order_items,
            total_amount: order.total_amount,
            status: order.status,
            shipping_address: order.shipping_address,
            created_at: order.created_at,
        });
    }

    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_order(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<OrderResponse>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let order: Option<Order> = sqlx::query_as("SELECT * FROM orders WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user.id)
        .fetch_optional(&*pool)
        .await?;

    let order = order.ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

    let items: Vec<OrderItem> = sqlx::query_as("SELECT * FROM order_items WHERE order_id = ?")
        .bind(&order.id)
        .fetch_all(&*pool)
        .await?;

    let mut order_items = Vec::new();
    for item in items {
        let product: Product = sqlx::query_as("SELECT * FROM products WHERE id = ?")
            .bind(&item.product_id)
            .fetch_one(&*pool)
            .await?;

        order_items.push(OrderItemResponse {
            product: product.into(),
            quantity: item.quantity,
            price: item.price,
        });
    }

    Ok(Json(ApiResponse::success(OrderResponse {
        id: order.id,
        items: order_items,
        total_amount: order.total_amount,
        status: order.status,
        shipping_address: order.shipping_address,
        created_at: order.created_at,
    })))
}

pub async fn get_user_profile(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = auth_user(&headers, &pool).await?;
    Ok(Json(ApiResponse::success(user.into())))
}

pub async fn create_review(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Path(product_id): Path<String>,
    Json(payload): Json<CreateReviewRequest>,
) -> Result<Json<ApiResponse<ReviewResponse>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    if payload.rating < 1 || payload.rating > 5 {
        return Err(AppError::BadRequest("Rating must be between 1 and 5".to_string()));
    }

    let existing: Option<(i64,)> =
        sqlx::query_as("SELECT COUNT(*) FROM reviews WHERE product_id = ? AND user_id = ?")
            .bind(&product_id)
            .bind(&user.id)
            .fetch_optional(&*pool)
            .await?;

    if let Some((count,)) = existing {
        if count > 0 {
            return Err(AppError::BadRequest("You already reviewed this product".to_string()));
        }
    }

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO reviews (id, product_id, user_id, rating, comment, created_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&product_id)
    .bind(&user.id)
    .bind(&payload.rating)
    .bind(&payload.comment)
    .bind(&now)
    .execute(&*pool)
    .await?;

    let (avg_rating,): (f64,) =
        sqlx::query_as("SELECT COALESCE(AVG(rating), 0) FROM reviews WHERE product_id = ?")
            .bind(&product_id)
            .fetch_one(&*pool)
            .await?;

    sqlx::query("UPDATE products SET rating = ? WHERE id = ?")
        .bind(avg_rating)
        .bind(&product_id)
        .execute(&*pool)
        .await?;

    Ok(Json(ApiResponse::success(ReviewResponse {
        id,
        user_id: user.id,
        username: user.username,
        rating: payload.rating,
        comment: payload.comment,
        created_at: now,
    })))
}

pub async fn get_product_reviews(
    State(pool): State<Arc<SqlitePool>>,
    Path(product_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<ReviewResponse>>>, AppError> {
    let reviews: Vec<(String, String, String, i32, Option<String>, String, String)> = sqlx::query_as(
        "SELECT r.id, r.product_id, r.user_id, r.rating, r.comment, r.created_at,
                u.username
         FROM reviews r
         JOIN users u ON r.user_id = u.id
         WHERE r.product_id = ?
         ORDER BY r.created_at DESC",
    )
    .bind(&product_id)
    .fetch_all(&*pool)
    .await?;

    let response: Vec<ReviewResponse> = reviews
        .into_iter()
        .map(|(id, _product_id, user_id, rating, comment, created_at, username)| ReviewResponse {
            id,
            user_id,
            username,
            rating,
            comment,
            created_at,
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

pub async fn create_payment(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Json(payload): Json<CreatePaymentRequest>,
) -> Result<Json<ApiResponse<PaymentResponse>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let order: Option<Order> =
        sqlx::query_as("SELECT * FROM orders WHERE id = ? AND user_id = ?")
            .bind(&payload.order_id)
            .bind(&user.id)
            .fetch_optional(&*pool)
            .await?;

    let order = order.ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

    if order.status != "pending" {
        return Err(AppError::BadRequest("Order is not in pending status".to_string()));
    }

    let existing_payment: Option<Payment> =
        sqlx::query_as("SELECT * FROM payments WHERE order_id = ? AND status = 'completed'")
            .bind(&payload.order_id)
            .fetch_optional(&*pool)
            .await?;

    if existing_payment.is_some() {
        return Err(AppError::BadRequest("Order already paid".to_string()));
    }

    let card_last_four = if payload.card_number.len() >= 4 {
        payload.card_number[payload.card_number.len() - 4..].to_string()
    } else {
        payload.card_number.clone()
    };

    let transaction_id = format!("TXN-{}", Uuid::new_v4().to_string().split('-').next().unwrap());

    let payment_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO payments (id, order_id, user_id, amount, card_last_four, card_brand, status, transaction_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, 'completed', ?, ?, ?)",
    )
    .bind(&payment_id)
    .bind(&payload.order_id)
    .bind(&user.id)
    .bind(order.total_amount)
    .bind(&card_last_four)
    .bind(&payload.card_brand)
    .bind(&transaction_id)
    .bind(&now)
    .bind(&now)
    .execute(&*pool)
    .await?;

    sqlx::query("UPDATE orders SET status = 'paid', updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(&payload.order_id)
        .execute(&*pool)
        .await?;

    Ok(Json(ApiResponse::success(PaymentResponse {
        id: payment_id,
        order_id: payload.order_id,
        amount: order.total_amount,
        card_last_four,
        card_brand: payload.card_brand,
        status: "completed".to_string(),
        transaction_id: Some(transaction_id),
        created_at: now,
    })))
}

pub async fn get_payment(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<PaymentResponse>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let payment: Option<Payment> =
        sqlx::query_as("SELECT * FROM payments WHERE id = ? AND user_id = ?")
            .bind(&id)
            .bind(&user.id)
            .fetch_optional(&*pool)
            .await?;

    match payment {
        Some(p) => Ok(Json(ApiResponse::success(p.into()))),
        None => Err(AppError::NotFound("Payment not found".to_string())),
    }
}

pub async fn get_payments(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<Vec<PaymentResponse>>>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let payments: Vec<Payment> =
        sqlx::query_as("SELECT * FROM payments WHERE user_id = ? ORDER BY created_at DESC")
            .bind(&user.id)
            .fetch_all(&*pool)
            .await?;

    let response: Vec<PaymentResponse> = payments.into_iter().map(|p| p.into()).collect();

    Ok(Json(ApiResponse::success(response)))
}

pub async fn create_ecpay_order(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Json(payload): Json<CreateEcpayOrderRequest>,
) -> Result<Html<String>, AppError> {
    let user = auth_user(&headers, &pool).await?;

    let order: Option<Order> =
        sqlx::query_as("SELECT * FROM orders WHERE id = ? AND user_id = ?")
            .bind(&payload.order_id)
            .bind(&user.id)
            .fetch_optional(&*pool)
            .await?;

    let order = order.ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

    if order.status != "pending" {
        return Err(AppError::BadRequest("Order is not in pending status".to_string()));
    }

    let existing_payment: Option<Payment> =
        sqlx::query_as("SELECT * FROM payments WHERE order_id = ? AND status = 'paid'")
            .bind(&payload.order_id)
            .fetch_optional(&*pool)
            .await?;

    if existing_payment.is_some() {
        return Err(AppError::BadRequest("Order already paid".to_string()));
    }

    let payment_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO payments (id, order_id, user_id, amount, card_last_four, card_brand, status, transaction_id, created_at, updated_at) VALUES (?, ?, ?, ?, '', 'ECPAY', 'pending', '', datetime('now'), datetime('now'))",
    )
    .bind(&payment_id)
    .bind(&payload.order_id)
    .bind(&user.id)
    .bind(order.total_amount)
    .execute(&*pool)
    .await?;

    let config = EcpayConfig::sandbox();
    let client = EcpayClient::new(config);

    let items: Vec<OrderItem> = sqlx::query_as("SELECT * FROM order_items WHERE order_id = ?")
        .bind(&payload.order_id)
        .fetch_all(&*pool)
        .await?;

    let mut item_names = Vec::new();
    for item in items.iter().take(5) {
        let product: Product = sqlx::query_as("SELECT name FROM products WHERE id = ?")
            .bind(&item.product_id)
            .fetch_one(&*pool)
            .await?;
        item_names.push(format!("{} x{}", product.name, item.quantity));
    }
    let item_name = if items.len() > 5 {
        format!("{} ... ({} items)", item_names.join("#"), items.len())
    } else {
        item_names.join("#")
    };

    let params = crate::ecpay::CreateEcpayOrderParams {
        merchant_trade_no: payment_id.clone(),
        total_amount: order.total_amount as i64,
        trade_desc: "Shop4 Order Payment".to_string(),
        item_name,
        return_url: payload.return_url.clone().unwrap_or_default(),
        client_back_url: payload.return_url.unwrap_or_default(),
    };

    let form_html = client.create_credit_card_order(params).await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok(Html(form_html))
}

#[derive(Debug, Deserialize)]
pub struct CreateEcpayOrderRequest {
    pub order_id: String,
    pub return_url: Option<String>,
}

pub async fn ecpay_return(
    State(pool): State<Arc<SqlitePool>>,
    params: Query<BTreeMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let rtn_code = params.get("RtnCode").and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
    let merchant_trade_no = params.get("MerchantTradeNo").cloned().unwrap_or_default();

    if rtn_code == 1 {
        let now = chrono::Utc::now().to_rfc3339();
        let trade_no = params.get("TradeNo").cloned().unwrap_or_default();
        let card4no = params.get("Card4No").cloned().unwrap_or_default();
        let card6no = params.get("Card6No").cloned().unwrap_or_default();

        let card_last_four = if !card4no.is_empty() {
            card4no
        } else if !card6no.is_empty() {
            card6no[card6no.len().saturating_sub(4)..].to_string()
        } else {
            "N/A".to_string()
        };

        sqlx::query(
            "UPDATE payments SET status = 'paid', card_last_four = ?, transaction_id = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&card_last_four)
        .bind(&trade_no)
        .bind(&now)
        .bind(&merchant_trade_no)
        .execute(&*pool)
        .await?;

        sqlx::query("UPDATE orders SET status = 'paid', updated_at = ? WHERE id = (SELECT order_id FROM payments WHERE id = ?)")
            .bind(&now)
            .bind(&merchant_trade_no)
            .execute(&*pool)
            .await?;
    }

    Ok("1|OK".into_response())
}

pub async fn ecpay_callback(
    State(pool): State<Arc<SqlitePool>>,
    params: Query<BTreeMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let rtn_code = params.get("RtnCode").and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
    let merchant_trade_no = params.get("MerchantTradeNo").cloned().unwrap_or_default();

    if rtn_code == 1 {
        let now = chrono::Utc::now().to_rfc3339();
        let trade_no = params.get("TradeNo").cloned().unwrap_or_default();
        let card4no = params.get("Card4No").cloned().unwrap_or_default();
        let card6no = params.get("Card6No").cloned().unwrap_or_default();

        let card_last_four = if !card4no.is_empty() {
            card4no
        } else if !card6no.is_empty() {
            card6no[card6no.len().saturating_sub(4)..].to_string()
        } else {
            "N/A".to_string()
        };

        sqlx::query(
            "UPDATE payments SET status = 'paid', card_last_four = ?, transaction_id = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&card_last_four)
        .bind(&trade_no)
        .bind(&now)
        .bind(&merchant_trade_no)
        .execute(&*pool)
        .await?;

        sqlx::query("UPDATE orders SET status = 'paid', updated_at = ? WHERE id = (SELECT order_id FROM payments WHERE id = ?)")
            .bind(&now)
            .bind(&merchant_trade_no)
            .execute(&*pool)
            .await?;

        tracing::info!("ECPay callback success: order_id={}, trade_no={}", merchant_trade_no, trade_no);
    }

    Ok("1|OK".into_response())
}

async fn auth_user(headers: &HeaderMap, pool: &SqlitePool) -> Result<User, AppError> {
    let token = extract_token(headers).ok_or(AppError::Unauthorized)?;

    let claims = validate_token(&token)?;

    let user: Option<User> =
        sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(&claims.sub)
            .fetch_optional(pool)
            .await?;

    user.ok_or(AppError::Unauthorized)
}