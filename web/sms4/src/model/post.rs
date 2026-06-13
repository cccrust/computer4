use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub parent_id: Option<i64>,
    pub likes_count: i64,
    pub replies_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostWithUser {
    pub id: i64,
    pub content: String,
    pub parent_id: Option<i64>,
    pub likes_count: i64,
    pub replies_count: i64,
    pub created_at: String,
    pub user_id: i64,
    pub username: String,
    pub display_name: String,
}

pub fn create_post(conn: &Connection, user_id: i64, content: &str, parent_id: Option<i64>) -> Result<i64> {
    conn.execute(
        "INSERT INTO posts (user_id, content, parent_id) VALUES (?1, ?2, ?3)",
        params![user_id, content, parent_id],
    )?;
    let post_id = conn.last_insert_rowid();
    if let Some(pid) = parent_id {
        conn.execute("UPDATE posts SET replies_count = replies_count + 1 WHERE id = ?1", params![pid])?;
    }
    Ok(post_id)
}

pub fn list_posts(conn: &Connection, user_id: Option<i64>, limit: i64, offset: i64) -> Result<Vec<PostWithUser>> {
    let mut sql = String::from(
        "SELECT p.id, p.content, p.parent_id, p.likes_count, p.replies_count, p.created_at,
                u.id, u.username, u.display_name
         FROM posts p JOIN users u ON p.user_id = u.id WHERE p.parent_id IS NULL"
    );
    let mut args: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(uid) = user_id {
        args.push(Box::new(uid));
        sql.push_str(&format!(" AND p.user_id = ?{}", args.len()));
    }
    sql.push_str(" ORDER BY p.created_at DESC");
    args.push(Box::new(limit));
    args.push(Box::new(offset));
    sql.push_str(&format!(" LIMIT ?{} OFFSET ?{}", args.len() - 1, args.len()));
    let mut stmt = conn.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = args.iter().map(|a| a.as_ref()).collect();
    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(PostWithUser {
            id: row.get(0)?,
            content: row.get(1)?,
            parent_id: row.get(2)?,
            likes_count: row.get(3)?,
            replies_count: row.get(4)?,
            created_at: row.get(5)?,
            user_id: row.get(6)?,
            username: row.get(7)?,
            display_name: row.get(8)?,
        })
    })?;
    let mut posts = Vec::new();
    for row in rows {
        posts.push(row?);
    }
    Ok(posts)
}

pub fn list_replies(conn: &Connection, post_id: i64) -> Result<Vec<PostWithUser>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.content, p.parent_id, p.likes_count, p.replies_count, p.created_at,
                u.id, u.username, u.display_name
         FROM posts p JOIN users u ON p.user_id = u.id
         WHERE p.parent_id = ?1 ORDER BY p.created_at ASC"
    )?;
    let rows = stmt.query_map(params![post_id], |row| {
        Ok(PostWithUser {
            id: row.get(0)?,
            content: row.get(1)?,
            parent_id: row.get(2)?,
            likes_count: row.get(3)?,
            replies_count: row.get(4)?,
            created_at: row.get(5)?,
            user_id: row.get(6)?,
            username: row.get(7)?,
            display_name: row.get(8)?,
        })
    })?;
    let mut posts = Vec::new();
    for row in rows {
        posts.push(row?);
    }
    Ok(posts)
}

pub fn get_post(conn: &Connection, id: i64) -> Result<Option<PostWithUser>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.content, p.parent_id, p.likes_count, p.replies_count, p.created_at,
                u.id, u.username, u.display_name
         FROM posts p JOIN users u ON p.user_id = u.id WHERE p.id = ?1"
    )?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(PostWithUser {
            id: row.get(0)?,
            content: row.get(1)?,
            parent_id: row.get(2)?,
            likes_count: row.get(3)?,
            replies_count: row.get(4)?,
            created_at: row.get(5)?,
            user_id: row.get(6)?,
            username: row.get(7)?,
            display_name: row.get(8)?,
        })
    })?;
    match rows.next() {
        Some(Ok(p)) => Ok(Some(p)),
        _ => Ok(None),
    }
}

pub fn delete_post(conn: &Connection, id: i64) -> Result<bool> {
    let n = conn.execute("DELETE FROM posts WHERE id = ?1", params![id])?;
    Ok(n > 0)
}

pub fn get_timeline(conn: &Connection, user_id: i64, limit: i64, offset: i64) -> Result<Vec<PostWithUser>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.content, p.parent_id, p.likes_count, p.replies_count, p.created_at,
                u.id, u.username, u.display_name
         FROM posts p JOIN users u ON p.user_id = u.id
         WHERE p.parent_id IS NULL
           AND (p.user_id = ?1
             OR p.user_id IN (SELECT followee_id FROM follows WHERE follower_id = ?1))
         ORDER BY p.created_at DESC
         LIMIT ?2 OFFSET ?3"
    )?;
    let rows = stmt.query_map(params![user_id, limit, offset], |row| {
        Ok(PostWithUser {
            id: row.get(0)?,
            content: row.get(1)?,
            parent_id: row.get(2)?,
            likes_count: row.get(3)?,
            replies_count: row.get(4)?,
            created_at: row.get(5)?,
            user_id: row.get(6)?,
            username: row.get(7)?,
            display_name: row.get(8)?,
        })
    })?;
    let mut posts = Vec::new();
    for row in rows {
        posts.push(row?);
    }
    Ok(posts)
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
    fn test_create_and_list() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None).unwrap();
        let pid = create_post(&c, uid, "Hello world!", None).unwrap();
        let posts = list_posts(&c, None, 10, 0).unwrap();
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].content, "Hello world!");
        assert_eq!(posts[0].username, "alice");
    }

    #[test]
    fn test_reply() {
        let c = conn();
        let uid = user::create_user(&c, "alice", "Alice", None).unwrap();
        let pid = create_post(&c, uid, "First!", None).unwrap();
        let rid = create_post(&c, uid, "Reply!", Some(pid)).unwrap();
        let p = get_post(&c, pid).unwrap().unwrap();
        assert_eq!(p.replies_count, 1);
        let replies = list_replies(&c, pid).unwrap();
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0].content, "Reply!");
    }
}
