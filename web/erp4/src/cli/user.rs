use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

use crate::cli::fmt;
use crate::model::user;

#[derive(Parser)]
pub struct UserCommand {
    #[command(subcommand)]
    pub subcommand: UserSubcommands,
}

#[derive(Subcommand)]
pub enum UserSubcommands {
    Create {
        username: String,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        display_name: Option<String>,
        #[arg(long)]
        role: Option<String>,
    },
    List,
    Get {
        id: i64,
    },
    Update {
        id: i64,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        display_name: Option<String>,
        #[arg(long)]
        role: Option<String>,
    },
    Delete {
        id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &UserSubcommands) -> Result<()> {
    match cmd {
        UserSubcommands::Create {
            username,
            password,
            display_name,
            role,
        } => {
            let pwd = password.as_deref().unwrap_or("changeme");
            let disp = display_name.as_deref().unwrap_or(username);
            let r = role.as_deref().unwrap_or("admin");
            match user::create_user(conn, username, pwd, disp, r) {
                Ok(id) => println!("已建立使用者 #{}: {} ({})", id, username, r),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        UserSubcommands::List => {
            let users = user::list_users(conn)?;
            if users.is_empty() {
                println!("查無使用者資料。");
                return Ok(());
            }
            println!(
                "{}",
                fmt::header(&format!(
                    "{:<4} {:<15} {:<20} {:<10}",
                    "ID", "帳號", "顯示名稱", "角色"
                ))
            );
            println!("{}", "-".repeat(55));
            for u in &users {
                println!(
                    "{:<4} {:<15} {:<20} {:<10}",
                    u.id, u.username, u.display_name, u.role
                );
            }
        }
        UserSubcommands::Get { id } => match user::get_user(conn, *id)? {
            Some(u) => {
                println!("ID:           {}", u.id);
                println!("帳號:         {}", u.username);
                println!("顯示名稱:     {}", u.display_name);
                println!("角色:         {}", u.role);
                println!("建立時間:     {}", u.created_at);
                println!("更新時間:     {}", u.updated_at);
            }
            None => println!("使用者 #{} 不存在。", id),
        },
        UserSubcommands::Update {
            id,
            password,
            display_name,
            role,
        } => {
            match user::update_user(
                conn,
                *id,
                password.as_deref(),
                display_name.as_deref(),
                role.as_deref(),
            ) {
                Ok(true) => println!("使用者 #{} 已更新。", id),
                Ok(false) => println!("使用者 #{} 不存在。", id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        UserSubcommands::Delete { id } => {
            if user::delete_user(conn, *id)? {
                println!("使用者 #{} 已刪除。", id);
            } else {
                println!("使用者 #{} 不存在。", id);
            }
        }
    }
    Ok(())
}
