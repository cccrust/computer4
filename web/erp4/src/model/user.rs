use anyhow::{bail, Result};
use chrono::Local;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

pub fn hash_password(password: &str) -> Result<String> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hash failed: {}", e))?
        .to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn create_user(
    conn: &Connection,
    username: &str,
    password: &str,
    display_name: &str,
    role: &str,
) -> Result<i64> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let hash = hash_password(password)?;
    conn.execute(
        "INSERT INTO users (username, password, display_name, role, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![username, hash, display_name, role, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_users(conn: &Connection) -> Result<Vec<User>> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password, display_name, role, created_at, updated_at FROM users ORDER BY id",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            display_name: row.get(3)?,
            role: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

pub fn get_user(conn: &Connection, id: i64) -> Result<Option<User>> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password, display_name, role, created_at, updated_at FROM users WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            display_name: row.get(3)?,
            role: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    match rows.next() {
        Some(Ok(u)) => Ok(Some(u)),
        _ => Ok(None),
    }
}

pub fn get_user_by_username(conn: &Connection, username: &str) -> Result<Option<User>> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password, display_name, role, created_at, updated_at FROM users WHERE username = ?1",
    )?;
    let mut rows = stmt.query_map(params![username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            display_name: row.get(3)?,
            role: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    match rows.next() {
        Some(Ok(u)) => Ok(Some(u)),
        _ => Ok(None),
    }
}

pub fn authenticate_user(
    conn: &Connection,
    username: &str,
    password: &str,
) -> Result<Option<User>> {
    let user = match get_user_by_username(conn, username)? {
        Some(u) => u,
        None => return Ok(None),
    };
    if verify_password(password, &user.password)? {
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub fn update_user(
    conn: &Connection,
    id: i64,
    password: Option<&str>,
    display_name: Option<&str>,
    role: Option<&str>,
) -> Result<bool> {
    let existing = match get_user(conn, id)? {
        Some(u) => u,
        None => return Ok(false),
    };
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let password = match password {
        Some(p) => hash_password(p)?,
        None => existing.password,
    };
    let display_name = display_name.unwrap_or(&existing.display_name);
    let role = role.unwrap_or(&existing.role);
    conn.execute(
        "UPDATE users SET password=?1, display_name=?2, role=?3, updated_at=?4 WHERE id=?5",
        params![password, display_name, role, now, id],
    )?;
    Ok(true)
}

pub fn delete_user(conn: &Connection, id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
    Ok(n > 0)
}

pub fn ensure_admin(conn: &Connection) -> Result<()> {
    if get_user_by_username(conn, "admin")?.is_none() {
        create_user(conn, "admin", "admin123", "Administrator", "admin")?;
        println!("預設管理員帳號已建立：admin / admin123");
    }
    Ok(())
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
    fn test_create_and_get_user() {
        let c = conn();
        let id = create_user(&c, "alice", "secret123", "Alice", "admin").unwrap();
        let u = get_user(&c, id).unwrap().unwrap();
        assert_eq!(u.username, "alice");
        assert_eq!(u.display_name, "Alice");
        assert!(verify_password("secret123", &u.password).unwrap());
        assert!(verify_password("wrong", &u.password).unwrap() == false);
    }

    #[test]
    fn test_authenticate_user() {
        let c = conn();
        create_user(&c, "bob", "pass123", "Bob", "viewer").unwrap();
        let u = authenticate_user(&c, "bob", "pass123").unwrap();
        assert!(u.is_some());
        let u = authenticate_user(&c, "bob", "wrong").unwrap();
        assert!(u.is_none());
        let u = authenticate_user(&c, "nonexistent", "pass123").unwrap();
        assert!(u.is_none());
    }

    #[test]
    fn test_update_user() {
        let c = conn();
        let id = create_user(&c, "carol", "oldpass", "Carol", "viewer").unwrap();
        update_user(&c, id, Some("newpass"), None, Some("admin")).unwrap();
        let u = get_user(&c, id).unwrap().unwrap();
        assert!(verify_password("newpass", &u.password).unwrap());
        assert_eq!(u.role, "admin");
    }

    #[test]
    fn test_hash_and_verify() {
        let hash = hash_password("mypassword").unwrap();
        assert!(verify_password("mypassword", &hash).unwrap());
        assert!(!verify_password("wrong", &hash).unwrap());
    }
}
