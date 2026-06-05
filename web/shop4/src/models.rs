use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: String,
    pub seller_id: String,
    pub category_id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub image_url: Option<String>,
    pub rating: f64,
    pub sold_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: String,
    pub seller_id: String,
    pub category_id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub image_url: Option<String>,
    pub rating: f64,
    pub sold_count: i32,
}

impl From<Product> for ProductResponse {
    fn from(p: Product) -> Self {
        Self {
            id: p.id,
            seller_id: p.seller_id,
            category_id: p.category_id,
            name: p.name,
            description: p.description,
            price: p.price,
            stock: p.stock,
            image_url: p.image_url,
            rating: p.rating,
            sold_count: p.sold_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CartItem {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItemResponse {
    pub id: String,
    pub product: ProductResponse,
    pub quantity: i32,
    pub subtotal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub total_amount: f64,
    pub status: String,
    pub shipping_address: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: String,
    pub items: Vec<OrderItemResponse>,
    pub total_amount: f64,
    pub status: String,
    pub shipping_address: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemResponse {
    pub product: ProductResponse,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Review {
    pub id: String,
    pub product_id: String,
    pub user_id: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResponse {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn message(msg: &str) -> ApiResponse<()> {
        ApiResponse {
            success: true,
            data: None,
            message: Some(msg.to_string()),
        }
    }
}