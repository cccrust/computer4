use anyhow::{bail, Result};
use chrono::Local;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Invoice {
    pub id: i64,
    pub invoice_number: String,
    pub order_id: Option<i64>,
    pub customer_id: i64,
    pub invoice_date: String,
    pub due_date: String,
    pub status: String,
    pub amount: f64,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

const INVOICE_VALID_STATUSES: &[&str] = &["unpaid", "paid", "overdue", "cancelled"];

fn next_invoice_number(conn: &Connection) -> Result<String> {
    let year = Local::now().format("%Y").to_string();
    conn.execute(
        "INSERT INTO sequences (name, value) VALUES ('invoice_number', 1)
         ON CONFLICT(name) DO UPDATE SET value = value + 1",
        [],
    )?;
    let seq: i64 = conn.query_row(
        "SELECT value FROM sequences WHERE name = 'invoice_number'",
        [],
        |row| row.get(0),
    )?;
    Ok(format!("INV-{}-{:04}", year, seq))
}

pub fn validate_email(email: &str) -> Result<()> {
    if !email.contains('@') || !email.contains('.') {
        bail!("Invalid email address: {}", email);
    }
    Ok(())
}

pub fn create_invoice(
    conn: &Connection,
    invoice_number: Option<&str>,
    order_id: Option<i64>,
    customer_id: i64,
    due_date: &str,
    amount: f64,
    notes: Option<&str>,
) -> Result<i64> {
    if amount <= 0.0 {
        bail!("Invoice amount must be positive");
    }
    let number = match invoice_number {
        Some(n) => n.to_string(),
        None => next_invoice_number(conn)?,
    };
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let invoice_date = Local::now().format("%Y-%m-%d").to_string();
    conn.execute(
        "INSERT INTO invoices (invoice_number, order_id, customer_id, invoice_date, due_date, status, amount, notes, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, 'unpaid', ?6, ?7, ?8, ?9)",
        params![number, order_id, customer_id, invoice_date, due_date, amount, notes, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_invoices(
    conn: &Connection,
    status_filter: Option<&str>,
    customer_id: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Result<Vec<Invoice>> {
    let mut sql = "SELECT id, invoice_number, order_id, customer_id, invoice_date, due_date, status, amount, notes, created_at, updated_at FROM invoices WHERE 1=1".to_string();
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
        Ok(Invoice {
            id: row.get(0)?,
            invoice_number: row.get(1)?,
            order_id: row.get(2)?,
            customer_id: row.get(3)?,
            invoice_date: row.get(4)?,
            due_date: row.get(5)?,
            status: row.get(6)?,
            amount: row.get(7)?,
            notes: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

pub fn get_invoice(conn: &Connection, id: i64) -> Result<Option<Invoice>> {
    let mut stmt = conn.prepare("SELECT id, invoice_number, order_id, customer_id, invoice_date, due_date, status, amount, notes, created_at, updated_at FROM invoices WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Invoice {
            id: row.get(0)?,
            invoice_number: row.get(1)?,
            order_id: row.get(2)?,
            customer_id: row.get(3)?,
            invoice_date: row.get(4)?,
            due_date: row.get(5)?,
            status: row.get(6)?,
            amount: row.get(7)?,
            notes: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    })?;
    match rows.next() {
        Some(Ok(inv)) => Ok(Some(inv)),
        _ => Ok(None),
    }
}

pub fn update_invoice_status(conn: &Connection, id: i64, status: &str) -> Result<bool> {
    if !INVOICE_VALID_STATUSES.contains(&status) {
        bail!(
            "Invalid status '{}'. Valid values: {}",
            status,
            INVOICE_VALID_STATUSES.join(", ")
        );
    }
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let n = conn.execute(
        "UPDATE invoices SET status=?1, updated_at=?2 WHERE id=?3",
        params![status, now, id],
    )?;
    Ok(n > 0)
}

pub fn delete_invoice(conn: &Connection, id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM invoices WHERE id = ?1", params![id])?;
    Ok(n > 0)
}

pub fn aging_report(conn: &Connection) -> Result<Vec<Invoice>> {
    let mut stmt = conn.prepare(
        "SELECT id, invoice_number, order_id, customer_id, invoice_date, due_date, status, amount, notes, created_at, updated_at
         FROM invoices WHERE status IN ('unpaid', 'overdue') ORDER BY due_date"
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Invoice {
            id: row.get(0)?,
            invoice_number: row.get(1)?,
            order_id: row.get(2)?,
            customer_id: row.get(3)?,
            invoice_date: row.get(4)?,
            due_date: row.get(5)?,
            status: row.get(6)?,
            amount: row.get(7)?,
            notes: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
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

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(include_str!("../db.sql")).unwrap();
        c
    }

    #[test]
    fn test_create_with_auto_number() {
        let c = conn();
        let cust_id = customer::create_customer(&c, "T", None, None, None).unwrap();
        let id = create_invoice(&c, None, None, cust_id, "2026-07-15", 100.0, None).unwrap();
        let inv = get_invoice(&c, id).unwrap().unwrap();
        assert!(inv.invoice_number.starts_with("INV-"));
        assert_eq!(inv.amount, 100.0);
    }

    #[test]
    fn test_create_with_custom_number() {
        let c = conn();
        let cust_id = customer::create_customer(&c, "T", None, None, None).unwrap();
        let id = create_invoice(
            &c,
            Some("CUSTOM-001"),
            None,
            cust_id,
            "2026-07-15",
            200.0,
            None,
        )
        .unwrap();
        let inv = get_invoice(&c, id).unwrap().unwrap();
        assert_eq!(inv.invoice_number, "CUSTOM-001");
    }

    #[test]
    fn test_filter_by_status() {
        let c = conn();
        let cust_id = customer::create_customer(&c, "T", None, None, None).unwrap();
        create_invoice(
            &c,
            Some("INV-001"),
            None,
            cust_id,
            "2026-07-15",
            100.0,
            None,
        )
        .unwrap();
        create_invoice(
            &c,
            Some("INV-002"),
            None,
            cust_id,
            "2026-07-15",
            200.0,
            None,
        )
        .unwrap();
        update_invoice_status(&c, 1, "paid").unwrap();
        let list = list_invoices(&c, Some("paid"), None, None, None).unwrap();
        assert_eq!(list.len(), 1);
        let list = list_invoices(&c, Some("unpaid"), None, None, None).unwrap();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_aging_report() {
        let c = conn();
        let cust_id = customer::create_customer(&c, "T", None, None, None).unwrap();
        create_invoice(
            &c,
            Some("INV-001"),
            None,
            cust_id,
            "2026-07-15",
            100.0,
            None,
        )
        .unwrap();
        create_invoice(
            &c,
            Some("INV-002"),
            None,
            cust_id,
            "2026-07-15",
            200.0,
            None,
        )
        .unwrap();
        update_invoice_status(&c, 2, "paid").unwrap();
        let aging = aging_report(&c).unwrap();
        assert_eq!(aging.len(), 1);
        assert_eq!(aging[0].invoice_number, "INV-001");
    }

    #[test]
    fn test_validate_amount() {
        let c = conn();
        let cust_id = customer::create_customer(&c, "T", None, None, None).unwrap();
        assert!(
            create_invoice(&c, Some("INV-003"), None, cust_id, "2026-07-15", 0.0, None).is_err()
        );
        assert!(create_invoice(
            &c,
            Some("INV-004"),
            None,
            cust_id,
            "2026-07-15",
            -10.0,
            None
        )
        .is_err());
    }
}
