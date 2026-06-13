use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn create_user(conn: &Connection, username: &str, display_name: &str, bio: Option<&str>) -> Result<i64> {
    conn.execute(
        "INSERT INTO users (username, display_name, bio) VALUES (?1, ?2, ?3)",
        params![username, display_name, bio],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_users(conn: &Connection, search: Option<&str>) -> Result<Vec<User>> {
    let mut sql = "SELECT id, username, display_name, bio, avatar, created_at, updated_at FROM users WHERE 1=1".to_string();
    let mut args: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(s) = search {
        let idx = args.len() + 1;
        sql.push_str(&format!(" AND (username LIKE ?{} OR display_name LIKE ?{})", idx, idx));
        args.push(Box::new(format!("%{}%", s)));
    }
    sql.push_str(" ORDER BY id");
    let mut stmt = conn.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = args.iter().map(|a| a.as_ref()).collect();
    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            display_name: row.get(2)?,
            bio: row.get(3)?,
            avatar: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    let mut users = Vec::new();
    for row in rows {
        users.push(row?);
    }
    Ok(users)
}

pub fn get_user(conn: &Connection, id: i64) -> Result<Option<User>> {
    let mut stmt = conn.prepare("SELECT id, username, display_name, bio, avatar, created_at, updated_at FROM users WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            display_name: row.get(2)?,
            bio: row.get(3)?,
            avatar: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    match rows.next() {
        Some(Ok(u)) => Ok(Some(u)),
        _ => Ok(None),
    }
}

pub fn update_user(conn: &Connection, id: i64, display_name: Option<&str>, bio: Option<&str>) -> Result<bool> {
    let existing = get_user(conn, id)?;
    let u = match existing {
        Some(u) => u,
        None => return Ok(false),
    };
    conn.execute(
        "UPDATE users SET display_name=?1, bio=?2, updated_at=datetime('now') WHERE id=?3",
        params![display_name.unwrap_or(&u.display_name), bio.or(u.bio.as_deref()), id],
    )?;
    Ok(true)
}

pub fn delete_user(conn: &Connection, id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
    Ok(n > 0)
}

pub fn get_followers_count(conn: &Connection, user_id: i64) -> Result<i64> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM follows WHERE followee_id = ?1", params![user_id], |row| row.get(0))?;
    Ok(count)
}

pub fn get_following_count(conn: &Connection, user_id: i64) -> Result<i64> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM follows WHERE follower_id = ?1", params![user_id], |row| row.get(0))?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        db::init_db(&c).unwrap();
        c
    }

    #[test]
    fn test_create_and_get() {
        let c = conn();
        let id = create_user(&c, "alice", "Alice", Some("Hello!")).unwrap();
        let u = get_user(&c, id).unwrap().unwrap();
        assert_eq!(u.username, "alice");
        assert_eq!(u.display_name, "Alice");
        assert_eq!(u.bio.unwrap(), "Hello!");
    }

    #[test]
    fn test_list_search() {
        let c = conn();
        create_user(&c, "alice", "Alice", None).unwrap();
        create_user(&c, "bob", "Bob", None).unwrap();
        let res = list_users(&c, Some("alice")).unwrap();
        assert_eq!(res.len(), 1);
        let res = list_users(&c, None).unwrap();
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_delete() {
        let c = conn();
        let id = create_user(&c, "temp", "Temp", None).unwrap();
        assert!(delete_user(&c, id).unwrap());
        assert!(get_user(&c, id).unwrap().is_none());
    }
}
