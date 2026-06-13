use anyhow::Result;
use rusqlite::Connection;

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    conn.execute_batch(include_str!("db.sql"))?;
    Ok(())
}
