use crate::model::product;
use anyhow::{bail, Result};
use chrono::Local;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PurchaseOrder {
    pub id: i64,
    pub supplier_id: i64,
    pub order_date: String,
    pub status: String,
    pub total_amount: f64,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PurchaseOrderItem {
    pub id: i64,
    pub purchase_order_id: i64,
    pub product_id: i64,
    pub quantity: i64,
    pub unit_price: f64,
}

const PO_VALID_STATUSES: &[&str] = &["pending", "approved", "received", "cancelled"];

pub fn create_purchase_order(
    conn: &Connection,
    supplier_id: i64,
    notes: Option<&str>,
) -> Result<i64> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let order_date = Local::now().format("%Y-%m-%d").to_string();
    conn.execute(
        "INSERT INTO purchase_orders (supplier_id, order_date, status, total_amount, notes, created_at, updated_at) VALUES (?1, ?2, 'pending', 0.0, ?3, ?4, ?5)",
        params![supplier_id, order_date, notes, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_purchase_orders(
    conn: &Connection,
    status_filter: Option<&str>,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Result<Vec<PurchaseOrder>> {
    let mut sql = "SELECT id, supplier_id, order_date, status, total_amount, notes, created_at, updated_at FROM purchase_orders WHERE 1=1".to_string();
    let mut args: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(s) = status_filter {
        sql.push_str(&format!(" AND status = ?{}", args.len() + 1));
        args.push(Box::new(s.to_string()));
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
        Ok(PurchaseOrder {
            id: row.get(0)?,
            supplier_id: row.get(1)?,
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

pub fn get_purchase_order(conn: &Connection, id: i64) -> Result<Option<PurchaseOrder>> {
    let mut stmt = conn.prepare("SELECT id, supplier_id, order_date, status, total_amount, notes, created_at, updated_at FROM purchase_orders WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(PurchaseOrder {
            id: row.get(0)?,
            supplier_id: row.get(1)?,
            order_date: row.get(2)?,
            status: row.get(3)?,
            total_amount: row.get(4)?,
            notes: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    match rows.next() {
        Some(Ok(po)) => Ok(Some(po)),
        _ => Ok(None),
    }
}

fn apply_po_stock(conn: &Connection, id: i64, multiplier: i64) -> Result<()> {
    let items = list_purchase_order_items(conn, id)?;
    for item in &items {
        product::adjust_stock(conn, item.product_id, item.quantity * multiplier)?;
    }
    Ok(())
}

pub fn update_purchase_order_status(conn: &Connection, id: i64, status: &str) -> Result<bool> {
    let valid = PO_VALID_STATUSES;
    if !valid.contains(&status) {
        bail!(
            "Invalid status '{}'. Valid values: {}",
            status,
            valid.join(", ")
        );
    }
    let po = match get_purchase_order(conn, id)? {
        Some(po) => po,
        None => return Ok(false),
    };
    let prev = po.status.as_str();
    if status == prev {
        return Ok(true);
    }
    if status == "received" && prev != "received" {
        apply_po_stock(conn, id, 1)?;
    } else if status == "cancelled" && prev == "received" {
        apply_po_stock(conn, id, -1)?;
    }
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE purchase_orders SET status=?1, updated_at=?2 WHERE id=?3",
        params![status, now, id],
    )?;
    Ok(true)
}

pub fn delete_purchase_order(conn: &Connection, id: i64) -> Result<bool> {
    let po = get_purchase_order(conn, id)?;
    if let Some(ref p) = po {
        if p.status == "received" {
            apply_po_stock(conn, id, -1)?;
        }
    }
    conn.execute(
        "DELETE FROM purchase_order_items WHERE purchase_order_id = ?1",
        params![id],
    )?;
    let n = conn.execute("DELETE FROM purchase_orders WHERE id = ?1", params![id])?;
    Ok(n > 0)
}

pub fn add_purchase_order_item(
    conn: &Connection,
    po_id: i64,
    product_id: i64,
    quantity: i64,
    unit_price: f64,
) -> Result<i64> {
    if quantity <= 0 {
        bail!("Quantity must be positive");
    }
    conn.execute(
        "INSERT INTO purchase_order_items (purchase_order_id, product_id, quantity, unit_price) VALUES (?1, ?2, ?3, ?4)",
        params![po_id, product_id, quantity, unit_price],
    )?;
    let item_id = conn.last_insert_rowid();

    let total: f64 = conn.query_row(
        "SELECT COALESCE(SUM(quantity * unit_price), 0.0) FROM purchase_order_items WHERE purchase_order_id = ?1",
        params![po_id],
        |row| row.get(0),
    )?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE purchase_orders SET total_amount=?1, updated_at=?2 WHERE id=?3",
        params![total, now, po_id],
    )?;
    Ok(item_id)
}

pub fn list_purchase_order_items(conn: &Connection, po_id: i64) -> Result<Vec<PurchaseOrderItem>> {
    let mut stmt = conn.prepare("SELECT id, purchase_order_id, product_id, quantity, unit_price FROM purchase_order_items WHERE purchase_order_id = ?1")?;
    let rows = stmt.query_map(params![po_id], |row| {
        Ok(PurchaseOrderItem {
            id: row.get(0)?,
            purchase_order_id: row.get(1)?,
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
    use crate::model::product;
    use crate::model::supplier;

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(include_str!("../db.sql")).unwrap();
        c
    }

    #[test]
    fn test_create_po_and_receive() {
        let c = conn();
        let sid = supplier::create_supplier(&c, "S1", None, None, None, None).unwrap();
        let pid = product::create_product(&c, "P1", "PO-TEST-001", 10.0, 50, None).unwrap();
        let poid = create_purchase_order(&c, sid, None).unwrap();
        add_purchase_order_item(&c, poid, pid, 20, 8.0).unwrap();
        update_purchase_order_status(&c, poid, "received").unwrap();
        let p = product::get_product(&c, pid).unwrap().unwrap();
        assert_eq!(p.stock, 70);
        update_purchase_order_status(&c, poid, "cancelled").unwrap();
        let p = product::get_product(&c, pid).unwrap().unwrap();
        assert_eq!(p.stock, 50);
    }

    #[test]
    fn test_po_status_filter() {
        let c = conn();
        let sid = supplier::create_supplier(&c, "S1", None, None, None, None).unwrap();
        let po1 = create_purchase_order(&c, sid, None).unwrap();
        let po2 = create_purchase_order(&c, sid, None).unwrap();
        update_purchase_order_status(&c, po1, "approved").unwrap();
        let list = list_purchase_orders(&c, Some("approved"), None, None).unwrap();
        assert_eq!(list.len(), 1);
        let list = list_purchase_orders(&c, Some("pending"), None, None).unwrap();
        assert_eq!(list.len(), 1);
    }
}
