use anyhow::Result;
use rusqlite::{params, Connection};

pub fn like_post(conn: &Connection, user_id: i64, post_id: i64) -> Result<bool> {
    let n = conn.execute(
        "INSERT OR IGNORE INTO likes (user_id, post_id) VALUES (?1, ?2)",
        params![user_id, post_id],
    )?;
    if n > 0 {
        conn.execute("UPDATE posts SET likes_count = likes_count + 1 WHERE id = ?1", params![post_id])?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn unlike_post(conn: &Connection, user_id: i64, post_id: i64) -> Result<bool> {
    let n = conn.execute(
        "DELETE FROM likes WHERE user_id = ?1 AND post_id = ?2",
        params![user_id, post_id],
    )?;
    if n > 0 {
        conn.execute("UPDATE posts SET likes_count = MAX(0, likes_count - 1) WHERE id = ?1", params![post_id])?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn is_liked(conn: &Connection, user_id: i64, post_id: i64) -> Result<bool> {
    let n: i64 = conn.query_row(
        "SELECT COUNT(*) FROM likes WHERE user_id = ?1 AND post_id = ?2",
        params![user_id, post_id],
        |row| row.get(0),
    )?;
    Ok(n > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::model::{post, user};

    fn conn() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        db::init_db(&c).unwrap();
        c
    }

    #[test]
    fn test_like_unlike() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None).unwrap();
        let pid = post::create_post(&c, uid, "Hello!", None).unwrap();
        assert!(like_post(&c, uid, pid).unwrap());
        assert!(is_liked(&c, uid, pid).unwrap());
        let p = post::get_post(&c, pid).unwrap().unwrap();
        assert_eq!(p.likes_count, 1);
        assert!(unlike_post(&c, uid, pid).unwrap());
        assert!(!is_liked(&c, uid, pid).unwrap());
    }

    #[test]
    fn test_like_duplicate() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None).unwrap();
        let pid = post::create_post(&c, uid, "Hello!", None).unwrap();
        assert!(like_post(&c, uid, pid).unwrap());
        assert!(!like_post(&c, uid, pid).unwrap());
        let p = post::get_post(&c, pid).unwrap().unwrap();
        assert_eq!(p.likes_count, 1);
    }
}
