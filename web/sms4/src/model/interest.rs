use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Interest {
    pub id: i64,
    pub user_id: i64,
    pub tag: String,
}

pub fn add_interest(conn: &Connection, user_id: i64, tag: &str) -> Result<i64> {
    conn.execute(
        "INSERT OR IGNORE INTO interests (user_id, tag) VALUES (?1, ?2)",
        params![user_id, tag],
    )?;
    let id = conn.query_row(
        "SELECT id FROM interests WHERE user_id = ?1 AND tag = ?2",
        params![user_id, tag],
        |row| row.get(0),
    )?;
    Ok(id)
}

pub fn remove_interest(conn: &Connection, user_id: i64, tag: &str) -> Result<bool> {
    let affected = conn.execute(
        "DELETE FROM interests WHERE user_id = ?1 AND tag = ?2",
        params![user_id, tag],
    )?;
    Ok(affected > 0)
}

pub fn list_interests(conn: &Connection, user_id: i64) -> Result<Vec<Interest>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, tag FROM interests WHERE user_id = ?1 ORDER BY tag"
    )?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(Interest {
            id: row.get(0)?,
            user_id: row.get(1)?,
            tag: row.get(2)?,
        })
    })?;
    let mut list = Vec::new();
    for row in rows {
        list.push(row?);
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::model::user;

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        db::init_db(&c).unwrap();
        c
    }

    #[test]
    fn test_add_and_list() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None).unwrap();
        let id1 = add_interest(&c, uid, "爬山").unwrap();
        let id2 = add_interest(&c, uid, "攝影").unwrap();
        assert!(id1 > 0);
        assert!(id2 > 0);
        let list = list_interests(&c, uid).unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_remove() {
        let c = conn();
        let uid = user::create_user(&c, "bob", "Bob", None).unwrap();
        add_interest(&c, uid, "咖啡").unwrap();
        assert!(remove_interest(&c, uid, "咖啡").unwrap());
        assert!(!remove_interest(&c, uid, "咖啡").unwrap());
        assert_eq!(list_interests(&c, uid).unwrap().len(), 0);
    }

    #[test]
    fn test_duplicate_ignored() {
        let c = conn();
        let uid = user::create_user(&c, "carol", "Carol", None).unwrap();
        add_interest(&c, uid, "美食").unwrap();
        add_interest(&c, uid, "美食").unwrap();
        assert_eq!(list_interests(&c, uid).unwrap().len(), 1);
    }
}
