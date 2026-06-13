use anyhow::Result;
use chrono::Local;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Supplier {
    pub id: i64,
    pub name: String,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn create_supplier(
    conn: &Connection,
    name: &str,
    contact_person: Option<&str>,
    email: Option<&str>,
    phone: Option<&str>,
    address: Option<&str>,
) -> Result<i64> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO suppliers (name, contact_person, email, phone, address, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![name, contact_person, email, phone, address, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_suppliers(
    conn: &Connection,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Result<Vec<Supplier>> {
    let mut sql = "SELECT id, name, contact_person, email, phone, address, created_at, updated_at FROM suppliers WHERE 1=1".to_string();
    let mut args: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
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
        Ok(Supplier {
            id: row.get(0)?,
            name: row.get(1)?,
            contact_person: row.get(2)?,
            email: row.get(3)?,
            phone: row.get(4)?,
            address: row.get(5)?,
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

pub fn get_supplier(conn: &Connection, id: i64) -> Result<Option<Supplier>> {
    let mut stmt = conn.prepare("SELECT id, name, contact_person, email, phone, address, created_at, updated_at FROM suppliers WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Supplier {
            id: row.get(0)?,
            name: row.get(1)?,
            contact_person: row.get(2)?,
            email: row.get(3)?,
            phone: row.get(4)?,
            address: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;
    match rows.next() {
        Some(Ok(s)) => Ok(Some(s)),
        _ => Ok(None),
    }
}

pub fn update_supplier(
    conn: &Connection,
    id: i64,
    name: Option<&str>,
    contact_person: Option<&str>,
    email: Option<&str>,
    phone: Option<&str>,
    address: Option<&str>,
) -> Result<bool> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let existing = match get_supplier(conn, id)? {
        Some(s) => s,
        None => return Ok(false),
    };
    let name = name.unwrap_or(&existing.name);
    let contact_person = contact_person.or(existing.contact_person.as_deref());
    let email = email.or(existing.email.as_deref());
    let phone = phone.or(existing.phone.as_deref());
    let address = address.or(existing.address.as_deref());
    conn.execute(
        "UPDATE suppliers SET name=?1, contact_person=?2, email=?3, phone=?4, address=?5, updated_at=?6 WHERE id=?7",
        params![name, contact_person, email, phone, address, now, id],
    )?;
    Ok(true)
}

pub fn delete_supplier(conn: &Connection, id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM suppliers WHERE id = ?1", params![id])?;
    Ok(n > 0)
}
