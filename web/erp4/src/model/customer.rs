use anyhow::{bail, Result};
use chrono::Local;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn validate_email_opt(email: Option<&str>) -> Result<()> {
    if let Some(e) = email {
        if !e.is_empty() && (!e.contains('@') || !e.contains('.')) {
            bail!("Invalid email address: {}", e);
        }
    }
    Ok(())
}

pub fn create_customer(
    conn: &Connection,
    name: &str,
    email: Option<&str>,
    phone: Option<&str>,
    address: Option<&str>,
) -> Result<i64> {
    validate_email_opt(email)?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO customers (name, email, phone, address, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![name, email, phone, address, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_customers(
    conn: &Connection,
    search: Option<&str>,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Result<Vec<Customer>> {
    let mut sql =
        "SELECT id, name, email, phone, address, created_at, updated_at FROM customers WHERE 1=1"
            .to_string();
    let mut args: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(s) = search {
        let idx = args.len() + 1;
        sql.push_str(&format!(
            " AND (name LIKE ?{} OR email LIKE ?{} OR phone LIKE ?{})",
            idx, idx, idx
        ));
        args.push(Box::new(format!("%{}%", s)));
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
        Ok(Customer {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            phone: row.get(3)?,
            address: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    let mut customers = Vec::new();
    for row in rows {
        customers.push(row?);
    }
    Ok(customers)
}

pub fn get_customer(conn: &Connection, id: i64) -> Result<Option<Customer>> {
    let mut stmt = conn.prepare("SELECT id, name, email, phone, address, created_at, updated_at FROM customers WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Customer {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            phone: row.get(3)?,
            address: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    match rows.next() {
        Some(Ok(c)) => Ok(Some(c)),
        _ => Ok(None),
    }
}

pub fn update_customer(
    conn: &Connection,
    id: i64,
    name: Option<&str>,
    email: Option<&str>,
    phone: Option<&str>,
    address: Option<&str>,
) -> Result<bool> {
    validate_email_opt(email)?;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let existing = get_customer(conn, id)?;
    let c = match existing {
        Some(c) => c,
        None => return Ok(false),
    };
    let name = name.unwrap_or(&c.name);
    let email = email.or(c.email.as_deref());
    let phone = phone.or(c.phone.as_deref());
    let address = address.or(c.address.as_deref());
    conn.execute(
        "UPDATE customers SET name=?1, email=?2, phone=?3, address=?4, updated_at=?5 WHERE id=?6",
        params![name, email, phone, address, now, id],
    )?;
    Ok(true)
}

pub fn delete_customer(conn: &Connection, id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM customers WHERE id = ?1", params![id])?;
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
        let id = create_customer(&c, "Alice", Some("alice@test.com"), Some("123"), None).unwrap();
        let cust = get_customer(&c, id).unwrap().unwrap();
        assert_eq!(cust.name, "Alice");
        assert_eq!(cust.email.unwrap(), "alice@test.com");
    }

    #[test]
    fn test_list_search() {
        let c = conn();
        create_customer(&c, "Bob Inc", Some("bob@inc.com"), None, None).unwrap();
        create_customer(&c, "Alice Corp", Some("alice@corp.com"), None, None).unwrap();
        let res = list_customers(&c, Some("Alice"), None, None).unwrap();
        assert_eq!(res.len(), 1);
        let res = list_customers(&c, Some("corp"), None, None).unwrap();
        assert_eq!(res.len(), 1);
        let res = list_customers(&c, None, None, None).unwrap();
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_email_validation() {
        assert!(validate_email_opt(Some("bad")).is_err());
        assert!(validate_email_opt(Some("good@test.com")).is_ok());
        assert!(validate_email_opt(None).is_ok());
    }
}
