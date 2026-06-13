use anyhow::Result;
use chrono::{Duration, Local};
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Session {
    pub id: String,
    pub user_id: i64,
    pub expires_at: String,
    pub created_at: String,
}

pub fn create_session(conn: &Connection, user_id: i64) -> Result<Session> {
    let now = Local::now();
    let expires_at = now + Duration::hours(24);
    let id = uuid::Uuid::new_v4().to_string();
    let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let expires_str = expires_at.format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO sessions (id, user_id, expires_at, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![id, user_id, expires_str, now_str],
    )?;
    Ok(Session {
        id,
        user_id,
        expires_at: expires_str,
        created_at: now_str,
    })
}

pub fn get_session(conn: &Connection, id: &str) -> Result<Option<Session>> {
    let mut stmt =
        conn.prepare("SELECT id, user_id, expires_at, created_at FROM sessions WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Session {
            id: row.get(0)?,
            user_id: row.get(1)?,
            expires_at: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;
    match rows.next() {
        Some(Ok(s)) => Ok(Some(s)),
        _ => Ok(None),
    }
}

pub fn list_sessions(conn: &Connection) -> Result<Vec<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, expires_at, created_at FROM sessions ORDER BY created_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Session {
            id: row.get(0)?,
            user_id: row.get(1)?,
            expires_at: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

pub fn delete_session(conn: &Connection, id: &str) -> Result<bool> {
    let n = conn.execute("DELETE FROM sessions WHERE id = ?1", params![id])?;
    Ok(n > 0)
}

pub fn cleanup_expired_sessions(conn: &Connection) -> Result<i64> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let n = conn.execute("DELETE FROM sessions WHERE expires_at < ?1", params![now])?;
    Ok(n as i64)
}

pub fn is_session_valid(conn: &Connection, id: &str) -> Result<bool> {
    let session = match get_session(conn, id)? {
        Some(s) => s,
        None => return Ok(false),
    };
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    Ok(session.expires_at >= now)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::user;

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(include_str!("../db.sql")).unwrap();
        c
    }

    #[test]
    fn test_create_and_get_session() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "pass", "Alice", "admin").unwrap();
        let session = create_session(&c, uid).unwrap();
        assert!(!session.id.is_empty());
        assert_eq!(session.user_id, uid);
        let fetched = get_session(&c, &session.id).unwrap().unwrap();
        assert_eq!(fetched.id, session.id);
        assert_eq!(fetched.user_id, uid);
    }

    #[test]
    fn test_delete_session() {
        let c = conn();
        let uid = user::create_user(&c, "bob", "pass", "Bob", "viewer").unwrap();
        let session = create_session(&c, uid).unwrap();
        assert!(is_session_valid(&c, &session.id).unwrap());
        delete_session(&c, &session.id).unwrap();
        assert!(!is_session_valid(&c, &session.id).unwrap());
    }

    #[test]
    fn test_session_validity() {
        let c = conn();
        assert!(!is_session_valid(&c, "nonexistent").unwrap());
    }
}
