use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

use crate::cli::fmt;
use crate::model::{session, user};

#[derive(Parser)]
pub struct SessionCommand {
    #[command(subcommand)]
    pub subcommand: SessionSubcommands,
}

#[derive(Subcommand)]
pub enum SessionSubcommands {
    Login {
        username: String,
        #[arg(long)]
        password: Option<String>,
    },
    Logout {
        session_id: String,
    },
    List,
}

pub fn run(conn: &Connection, cmd: &SessionSubcommands) -> Result<()> {
    match cmd {
        SessionSubcommands::Login { username, password } => {
            let pwd = password.as_deref().unwrap_or("admin123");
            match user::authenticate_user(conn, username, pwd)? {
                Some(u) => {
                    let s = session::create_session(conn, u.id)?;
                    println!("登入成功。Session ID: {}", s.id);
                    println!("使用者: {} ({})", u.display_name, u.role);
                }
                None => println!("{}", fmt::error_msg("帳號或密碼錯誤")),
            }
        }
        SessionSubcommands::Logout { session_id } => {
            if session::delete_session(conn, session_id)? {
                println!("已登出。");
            } else {
                println!("Session #{} 不存在。", session_id);
            }
        }
        SessionSubcommands::List => {
            let sessions = session::list_sessions(conn)?;
            if sessions.is_empty() {
                println!("無活躍 Session。");
                return Ok(());
            }
            println!(
                "{}",
                fmt::header(&format!(
                    "{:<40} {:<8} {:<20}",
                    "Session ID", "使用者ID", "到期時間"
                ))
            );
            println!("{}", "-".repeat(72));
            for s in &sessions {
                println!("{:<40} {:<8} {:<20}", s.id, s.user_id, s.expires_at);
            }
        }
    }
    Ok(())
}
