use crate::model::product;
use anyhow::{bail, Result};
use chrono::Local;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Order {
    pub id: i64,
    pub customer_id: i64,
    pub order_date: String,
    pub status: String,
    pub total_amount: f64,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i64,
    pub unit_price: f64,
}

const ORDER_VALID_STATUSES: &[&str] =
    &["pending", "confirmed", "shipped", "delivered", "cancelled"];

pub fn validate_status(status: &str, valid: &[&str]) -> Result<()> {
    if !valid.contains(&status) {
        bail!(
            "Invalid status '{}'. Valid values: {}",
            status,
            valid.join(", ")
        );
    }
    Ok(())
}

pub fn create_order(conn: &Connection, customer_id: i64, notes: Option<&str>) -> Result<i64> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let order_date = Local::now().format("%Y-%m-%d").to_string();
    conn.execute(
        "INSERT INTO orders (customer_id, order_date, status, total_amount, notes, created_at, updated_at) VALUES (?1, ?2, 'pending', 0.0, ?3, ?4, ?5)",
        params![customer_id, order_date, notes, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_orders(
    conn: &Connection,
    status_filter: Option<&str>,
    customer_id: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Result<Vec<Order>> {
    let mut sql = "SELECT id, customer_id, order_date, status, total_amount, notes, created_at, updated_at FROM orders WHERE 1=1".to_string();
    let mut args: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(s) = status_filter {
        sql.push_str(&format!(" AND status = ?{}", args.len() + 1));
        args.push(Box::new(s.to_string()));
    }
    if let Some(cid) = customer_id {
        sql.push_str(&format!(" AND customer_id = ?{}", args.len() + 1));
        args.push(Box::new(cid));
    }
    sql.push_str(" ORDER BY id");
    if let (Some(ps), Some(p)) = (page_size, page) {
        sql.push_str(&format!(
            " LIMIT ?{} OFFSET ?{}",
            args.len() + 1,
            args.len() + 2
        ));
        args.push(Box::new(ps));
        args.push(Box::new((p - 1) * ps));
    }
    let mut stmt = conn.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = args.iter().map(|a| a.as_ref()).collect();
    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(Order {
            id: row.get(0)?,
            customer_id: row.get(1)?,
            order_date: row.get(2)?,
            status: row.get(3)?,
            total_amount: row.get(4)?,
            notes: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

pub fn get_order(conn: &Connection, id: i64) -> Result<Option<Order>> {
    let mut stmt = conn.prepare("SELECT id, customer_id, order_date, status, total_amount, notes, created_at, updated_at FROM orders WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Order {
            id: row.get(0)?,
            customer_id: row.get(1)?,
            order_date: row.get(2)?,
            status: row.get(3)?,
            total_amount: row.get(4)?,
            notes: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    match rows.next() {
        Some(Ok(o)) => Ok(Some(o)),
        _ => Ok(None),
    }
}

fn apply_order_stock(conn: &Connection, id: i64, multiplier: i64) -> Result<()> {
    let items = list_order_items(conn, id)?;
    for item in &items {
        product::adjust_stock(conn, item.product_id, -item.quantity * multiplier)?;
    }
    Ok(())
}

fn status_was_confirmed_or_beyond(s: &str) -> bool {
    matches!(s, "confirmed" | "shipped" | "delivered")
}

pub fn update_order_status(conn: &Connection, id: i64, status: &str) -> Result<bool> {
    validate_status(status, ORDER_VALID_STATUSES)?;
    let order = match get_order(conn, id)? {
        Some(o) => o,
        None => return Ok(false),
    };
    let prev = order.status.as_str();

    if status == prev {
        return Ok(true);
    }

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    if !status_was_confirmed_or_beyond(prev) && status_was_confirmed_or_beyond(status) {
        apply_order_stock(conn, id, 1)?;
    } else if status_was_confirmed_or_beyond(prev) && status == "cancelled" {
        apply_order_stock(conn, id, -1)?;
    } else if status == "cancelled" {
    }

    conn.execute(
        "UPDATE orders SET status=?1, updated_at=?2 WHERE id=?3",
        params![status, now, id],
    )?;
    Ok(true)
}

pub fn delete_order(conn: &Connection, id: i64) -> Result<bool> {
    let order = get_order(conn, id)?;
    if let Some(ref o) = order {
        if status_was_confirmed_or_beyond(&o.status) {
            apply_order_stock(conn, id, -1)?;
        }
    }
    conn.execute("DELETE FROM order_items WHERE order_id = ?1", params![id])?;
    let n = conn.execute("DELETE FROM orders WHERE id = ?1", params![id])?;
    Ok(n > 0)
}

pub fn add_order_item(
    conn: &Connection,
    order_id: i64,
    product_id: i64,
    quantity: i64,
    unit_price: f64,
) -> Result<i64> {
    if quantity <= 0 {
        bail!("Quantity must be positive");
    }
    conn.execute(
        "INSERT INTO order_items (order_id, product_id, quantity, unit_price) VALUES (?1, ?2, ?3, ?4)",
        params![order_id, product_id, quantity, unit_price],
    )?;
    let item_id = conn.last_insert_rowid();

    let total: f64 = conn.query_row(
        "SELECT COALESCE(SUM(quantity * unit_price), 0.0) FROM order_items WHERE order_id = ?1",
        params![order_id],
        |row| row.get(0),
    )?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE orders SET total_amount=?1, updated_at=?2 WHERE id=?3",
        params![total, now, order_id],
    )?;
    Ok(item_id)
}

pub fn list_order_items(conn: &Connection, order_id: i64) -> Result<Vec<OrderItem>> {
    let mut stmt = conn.prepare("SELECT id, order_id, product_id, quantity, unit_price FROM order_items WHERE order_id = ?1")?;
    let rows = stmt.query_map(params![order_id], |row| {
        Ok(OrderItem {
            id: row.get(0)?,
            order_id: row.get(1)?,
            product_id: row.get(2)?,
            quantity: row.get(3)?,
            unit_price: row.get(4)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::customer;
    use crate::model::product;

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(include_str!("../db.sql")).unwrap();
        c
    }

    #[test]
    fn test_create_order_and_items() {
        let c = conn();
        let cust_id = customer::create_customer(&c, "Test", None, None, None).unwrap();
        let prod_id = product::create_product(&c, "P1", "P-001", 10.0, 100, None).unwrap();
        let oid = create_order(&c, cust_id, None).unwrap();
        add_order_item(&c, oid, prod_id, 5, 10.0).unwrap();
        let o = get_order(&c, oid).unwrap().unwrap();
        assert_eq!(o.total_amount, 50.0);
        assert_eq!(o.status, "pending");
        let items = list_order_items(&c, oid).unwrap();
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn test_stock_deduct_on_confirm() {
        let c = conn();
        let cust_id = customer::create_customer(&c, "T", None, None, None).unwrap();
        let pid = product::create_product(&c, "P1", "P-002", 10.0, 100, None).unwrap();
        let oid = create_order(&c, cust_id, None).unwrap();
        add_order_item(&c, oid, pid, 30, 10.0).unwrap();
        update_order_status(&c, oid, "confirmed").unwrap();
        let p = product::get_product(&c, pid).unwrap().unwrap();
        assert_eq!(p.stock, 70);
        update_order_status(&c, oid, "cancelled").unwrap();
        let p = product::get_product(&c, pid).unwrap().unwrap();
        assert_eq!(p.stock, 100);
    }

    #[test]
    fn test_insufficient_stock() {
        let c = conn();
        let cust_id = customer::create_customer(&c, "T", None, None, None).unwrap();
        let pid = product::create_product(&c, "P1", "P-003", 10.0, 5, None).unwrap();
        let oid = create_order(&c, cust_id, None).unwrap();
        add_order_item(&c, oid, pid, 10, 10.0).unwrap();
        assert!(update_order_status(&c, oid, "confirmed").is_err());
    }

    #[test]
    fn test_filter_by_status() {
        let c = conn();
        let cust_id = customer::create_customer(&c, "T", None, None, None).unwrap();
        let o1 = create_order(&c, cust_id, None).unwrap();
        let o2 = create_order(&c, cust_id, None).unwrap();
        update_order_status(&c, o1, "confirmed").unwrap();
        let list = list_orders(&c, Some("confirmed"), None, None, None).unwrap();
        assert_eq!(list.len(), 1);
        let list = list_orders(&c, Some("pending"), None, None, None).unwrap();
        assert_eq!(list.len(), 1);
    }
}
