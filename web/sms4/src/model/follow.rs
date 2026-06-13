use anyhow::{bail, Result};
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Follow {
    pub id: i64,
    pub follower_id: i64,
    pub followee_id: i64,
    pub created_at: String,
}

pub fn follow_user(conn: &Connection, follower_id: i64, followee_id: i64) -> Result<()> {
    if follower_id == followee_id {
        bail!("不能追蹤自己");
    }
    conn.execute(
        "INSERT OR IGNORE INTO follows (follower_id, followee_id) VALUES (?1, ?2)",
        params![follower_id, followee_id],
    )?;
    Ok(())
}

pub fn unfollow_user(conn: &Connection, follower_id: i64, followee_id: i64) -> Result<bool> {
    let n = conn.execute(
        "DELETE FROM follows WHERE follower_id = ?1 AND followee_id = ?2",
        params![follower_id, followee_id],
    )?;
    Ok(n > 0)
}

#[derive(Debug, Clone, Serialize)]
pub struct UserBrief {
    pub id: i64,
    pub username: String,
    pub display_name: String,
}

pub fn list_followers(conn: &Connection, user_id: i64) -> Result<Vec<UserBrief>> {
    let mut stmt = conn.prepare(
        "SELECT u.id, u.username, u.display_name
         FROM follows f JOIN users u ON f.follower_id = u.id
         WHERE f.followee_id = ?1 ORDER BY f.created_at DESC"
    )?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(UserBrief {
            id: row.get(0)?,
            username: row.get(1)?,
            display_name: row.get(2)?,
        })
    })?;
    let mut users = Vec::new();
    for row in rows {
        users.push(row?);
    }
    Ok(users)
}

pub fn list_following(conn: &Connection, user_id: i64) -> Result<Vec<UserBrief>> {
    let mut stmt = conn.prepare(
        "SELECT u.id, u.username, u.display_name
         FROM follows f JOIN users u ON f.followee_id = u.id
         WHERE f.follower_id = ?1 ORDER BY f.created_at DESC"
    )?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(UserBrief {
            id: row.get(0)?,
            username: row.get(1)?,
            display_name: row.get(2)?,
        })
    })?;
    let mut users = Vec::new();
    for row in rows {
        users.push(row?);
    }
    Ok(users)
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
    fn test_follow_and_list() {
        let c = conn();
        let u1 = user::create_user(&c, "alice", "Alice", None).unwrap();
        let u2 = user::create_user(&c, "bob", "Bob", None).unwrap();
        follow_user(&c, u1, u2).unwrap();
        let followers = list_followers(&c, u2).unwrap();
        assert_eq!(followers.len(), 1);
        assert_eq!(followers[0].username, "alice");
        let following = list_following(&c, u1).unwrap();
        assert_eq!(following.len(), 1);
        assert_eq!(following[0].username, "bob");
    }

    #[test]
    fn test_unfollow() {
        let c = conn();
        let u1 = user::create_user(&c, "alice", "Alice", None).unwrap();
        let u2 = user::create_user(&c, "bob", "Bob", None).unwrap();
        follow_user(&c, u1, u2).unwrap();
        assert!(unfollow_user(&c, u1, u2).unwrap());
        assert_eq!(list_followers(&c, u2).unwrap().len(), 0);
    }

    #[test]
    fn test_follow_self_fails() {
        let c = conn();
        let u1 = user::create_user(&c, "alice", "Alice", None).unwrap();
        assert!(follow_user(&c, u1, u1).is_err());
    }
}
