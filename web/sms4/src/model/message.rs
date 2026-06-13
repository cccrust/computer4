use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub content: String,
    pub read: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageWithUser {
    pub id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub content: String,
    pub read: i64,
    pub created_at: String,
    pub sender_username: String,
    pub sender_display_name: String,
    pub receiver_username: String,
    pub receiver_display_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Conversation {
    pub other_user_id: i64,
    pub other_username: String,
    pub other_display_name: String,
    pub last_message: String,
    pub last_message_at: String,
    pub unread_count: i64,
}

pub fn send_message(conn: &Connection, sender_id: i64, receiver_id: i64, content: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO messages (sender_id, receiver_id, content) VALUES (?1, ?2, ?3)",
        params![sender_id, receiver_id, content],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_conversations(conn: &Connection, user_id: i64) -> Result<Vec<Conversation>> {
    let mut stmt = conn.prepare(
        "SELECT
            other_id, u.username, u.display_name,
            m.content, m.created_at,
            COALESCE(unr.cnt, 0)
        FROM (
            SELECT
                CASE WHEN sender_id = ?1 THEN receiver_id ELSE sender_id END AS other_id,
                MAX(id) AS last_msg_id
            FROM messages
            WHERE sender_id = ?1 OR receiver_id = ?1
            GROUP BY other_id
        ) latest
        JOIN messages m ON m.id = latest.last_msg_id
        JOIN users u ON u.id = latest.other_id
        LEFT JOIN (
            SELECT sender_id AS sid, COUNT(*) AS cnt
            FROM messages
            WHERE receiver_id = ?1 AND read = 0
            GROUP BY sender_id
        ) unr ON unr.sid = latest.other_id
        ORDER BY m.created_at DESC"
    )?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok(Conversation {
            other_user_id: row.get(0)?,
            other_username: row.get(1)?,
            other_display_name: row.get(2)?,
            last_message: row.get(3)?,
            last_message_at: row.get(4)?,
            unread_count: row.get(5)?,
        })
    })?;
    let mut convs = Vec::new();
    for row in rows {
        convs.push(row?);
    }
    Ok(convs)
}

pub fn list_messages(conn: &Connection, user_id: i64, other_id: i64) -> Result<Vec<MessageWithUser>> {
    let mut stmt = conn.prepare(
        "SELECT m.id, m.sender_id, m.receiver_id, m.content, m.read, m.created_at,
                su.username, su.display_name, ru.username, ru.display_name
         FROM messages m
         JOIN users su ON su.id = m.sender_id
         JOIN users ru ON ru.id = m.receiver_id
         WHERE (m.sender_id = ?1 AND m.receiver_id = ?2)
            OR (m.sender_id = ?2 AND m.receiver_id = ?1)
         ORDER BY m.created_at ASC"
    )?;
    let rows = stmt.query_map(params![user_id, other_id], |row| {
        Ok(MessageWithUser {
            id: row.get(0)?,
            sender_id: row.get(1)?,
            receiver_id: row.get(2)?,
            content: row.get(3)?,
            read: row.get(4)?,
            created_at: row.get(5)?,
            sender_username: row.get(6)?,
            sender_display_name: row.get(7)?,
            receiver_username: row.get(8)?,
            receiver_display_name: row.get(9)?,
        })
    })?;
    let mut msgs = Vec::new();
    for row in rows {
        msgs.push(row?);
    }
    Ok(msgs)
}

pub fn mark_read(conn: &Connection, user_id: i64, other_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE messages SET read = 1 WHERE receiver_id = ?1 AND sender_id = ?2 AND read = 0",
        params![user_id, other_id],
    )?;
    Ok(())
}

pub fn get_unread_count(conn: &Connection, user_id: i64) -> Result<i64> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM messages WHERE receiver_id = ?1 AND read = 0",
        params![user_id],
        |row| row.get(0),
    )?;
    Ok(count)
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
    fn test_send_and_list() {
        let c = conn();
        let u1 = user::create_user(&c, "alice", "Alice", None).unwrap();
        let u2 = user::create_user(&c, "bob", "Bob", None).unwrap();
        let mid = send_message(&c, u1, u2, "哈囉！").unwrap();
        assert!(mid > 0);
        let convs = list_conversations(&c, u1).unwrap();
        assert_eq!(convs.len(), 1);
        assert_eq!(convs[0].other_username, "bob");
        assert_eq!(convs[0].last_message, "哈囉！");
        let msgs = list_messages(&c, u1, u2).unwrap();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].content, "哈囉！");
    }

    #[test]
    fn test_unread_and_mark_read() {
        let c = conn();
        let u1 = user::create_user(&c, "alice", "Alice", None).unwrap();
        let u2 = user::create_user(&c, "bob", "Bob", None).unwrap();
        send_message(&c, u2, u1, "嗨 alice").unwrap();
        assert_eq!(get_unread_count(&c, u1).unwrap(), 1);
        mark_read(&c, u1, u2).unwrap();
        assert_eq!(get_unread_count(&c, u1).unwrap(), 0);
    }
}
