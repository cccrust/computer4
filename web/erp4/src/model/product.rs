use anyhow::{anyhow, bail, Result};
use chrono::Local;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub price: f64,
    pub stock: i64,
    pub created_at: String,
    pub updated_at: String,
}

pub fn validate_product(price: f64, stock: i64) -> Result<()> {
    if price <= 0.0 {
        bail!("Price must be positive");
    }
    if stock < 0 {
        bail!("Stock cannot be negative");
    }
    Ok(())
}

pub fn create_product(
    conn: &Connection,
    name: &str,
    sku: &str,
    price: f64,
    stock: i64,
    description: Option<&str>,
) -> Result<i64> {
    validate_product(price, stock)?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO products (name, description, sku, price, stock, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![name, description, sku, price, stock, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_products(
    conn: &Connection,
    search: Option<&str>,
    low_stock: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Result<Vec<Product>> {
    let mut sql = "SELECT id, name, description, sku, price, stock, created_at, updated_at FROM products WHERE 1=1".to_string();
    let mut args: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(s) = search {
        let idx = args.len() + 1;
        sql.push_str(&format!(" AND (name LIKE ?{} OR sku LIKE ?{})", idx, idx));
        args.push(Box::new(format!("%{}%", s)));
    }
    if let Some(ls) = low_stock {
        sql.push_str(&format!(" AND stock <= ?{}", args.len() + 1));
        args.push(Box::new(ls));
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
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            sku: row.get(3)?,
            price: row.get(4)?,
            stock: row.get(5)?,
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

pub fn get_product(conn: &Connection, id: i64) -> Result<Option<Product>> {
    let mut stmt = conn.prepare("SELECT id, name, description, sku, price, stock, created_at, updated_at FROM products WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            sku: row.get(3)?,
            price: row.get(4)?,
            stock: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    match rows.next() {
        Some(Ok(p)) => Ok(Some(p)),
        _ => Ok(None),
    }
}

pub fn adjust_stock(conn: &Connection, id: i64, delta: i64) -> Result<()> {
    let product = get_product(conn, id)?.ok_or_else(|| anyhow!("Product #{} not found", id))?;
    let new_stock = product.stock + delta;
    if new_stock < 0 {
        bail!(
            "Insufficient stock for product #{} ({}): have {}, need {}",
            id,
            product.name,
            product.stock,
            -delta
        );
    }
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE products SET stock=?1, updated_at=?2 WHERE id=?3",
        params![new_stock, now, id],
    )?;
    Ok(())
}

pub fn update_product(
    conn: &Connection,
    id: i64,
    name: Option<&str>,
    sku: Option<&str>,
    price: Option<f64>,
    stock: Option<i64>,
    description: Option<&str>,
) -> Result<bool> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let existing = match get_product(conn, id)? {
        Some(p) => p,
        None => return Ok(false),
    };
    let name = name.unwrap_or(&existing.name);
    let sku = sku.unwrap_or(&existing.sku);
    let price = price.unwrap_or(existing.price);
    let stock = stock.unwrap_or(existing.stock);
    let description = description.or(existing.description.as_deref());
    validate_product(price, stock)?;
    conn.execute(
        "UPDATE products SET name=?1, description=?2, sku=?3, price=?4, stock=?5, updated_at=?6 WHERE id=?7",
        params![name, description, sku, price, stock, now, id],
    )?;
    Ok(true)
}

pub fn delete_product(conn: &Connection, id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM products WHERE id = ?1", params![id])?;
    Ok(n > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(include_str!("../db.sql")).unwrap();
        c
    }

    #[test]
    fn test_create_and_get() {
        let c = conn();
        let id = create_product(&c, "Test", "TST-001", 10.0, 100, None).unwrap();
        let p = get_product(&c, id).unwrap().unwrap();
        assert_eq!(p.name, "Test");
        assert_eq!(p.sku, "TST-001");
        assert_eq!(p.price, 10.0);
        assert_eq!(p.stock, 100);
    }

    #[test]
    fn test_validate_price() {
        assert!(validate_product(0.0, 10).is_err());
        assert!(validate_product(-1.0, 10).is_err());
        assert!(validate_product(10.0, -1).is_err());
        assert!(validate_product(10.0, 0).is_ok());
    }

    #[test]
    fn test_adjust_stock() {
        let c = conn();
        let id = create_product(&c, "Test", "TST-002", 10.0, 50, None).unwrap();
        adjust_stock(&c, id, -10).unwrap();
        assert_eq!(get_product(&c, id).unwrap().unwrap().stock, 40);
        adjust_stock(&c, id, 20).unwrap();
        assert_eq!(get_product(&c, id).unwrap().unwrap().stock, 60);
        assert!(adjust_stock(&c, id, -100).is_err());
    }

    #[test]
    fn test_list_search() {
        let c = conn();
        create_product(&c, "Widget Alpha", "W-A", 10.0, 10, None).unwrap();
        create_product(&c, "Widget Beta", "W-B", 20.0, 20, None).unwrap();
        create_product(&c, "Gadget", "G-001", 30.0, 5, None).unwrap();
        let res = list_products(&c, Some("Widget"), None, None, None).unwrap();
        assert_eq!(res.len(), 2);
        let res = list_products(&c, Some("Gadget"), None, None, None).unwrap();
        assert_eq!(res.len(), 1);
        let res = list_products(&c, None, Some(10), None, None).unwrap();
        assert_eq!(res.len(), 2);
    }
}
