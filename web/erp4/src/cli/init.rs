use anyhow::Result;
use rusqlite::Connection;

use crate::db;
use crate::model::user;

pub fn run(conn: &Connection) -> Result<()> {
    db::init_db(conn)?;
    user::ensure_admin(conn)?;
    println!("ERP 資料庫初始化完成。");
    Ok(())
}
