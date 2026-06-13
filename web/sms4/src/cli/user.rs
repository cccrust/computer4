use crate::cli::fmt;
use crate::model::user;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct UserCommand {
    #[command(subcommand)]
    pub subcommand: UserSubcommands,
}

#[derive(Subcommand)]
pub enum UserSubcommands {
    Add {
        username: String,
        display_name: String,
        #[arg(long)]
        bio: Option<String>,
    },
    List {
        #[arg(long, short)]
        search: Option<String>,
    },
    Get {
        id: i64,
    },
    Update {
        id: i64,
        #[arg(long)]
        display_name: Option<String>,
        #[arg(long)]
        bio: Option<String>,
    },
    Delete {
        id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &UserSubcommands) -> Result<()> {
    match cmd {
        UserSubcommands::Add { username, display_name, bio } => {
            match user::create_user(conn, username, display_name, bio.as_deref()) {
                Ok(id) => println!("{}", fmt::success_msg(&format!("已建立使用者 #{}: @{} ({})", id, username, display_name))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        UserSubcommands::List { search } => {
            let users = user::list_users(conn, search.as_deref())?;
            if users.is_empty() {
                println!("{}", fmt::info_msg("查無使用者。"));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("{:<4} {:<20} {:<25} {:<30}", "ID", "帳號", "顯示名稱", "簡介")));
            println!("{}", "-".repeat(85));
            for u in &users {
                println!("{:<4} @{:<18} {:<25} {:<30}",
                    u.id, u.username, u.display_name, u.bio.as_deref().unwrap_or(""));
            }
        }
        UserSubcommands::Get { id } => {
            match user::get_user(conn, *id)? {
                Some(u) => {
                    let followers = user::get_followers_count(conn, *id)?;
                    let following = user::get_following_count(conn, *id)?;
                    println!("ID:           {}", u.id);
                    println!("帳號:         @{}", u.username);
                    println!("顯示名稱:     {}", u.display_name);
                    println!("簡介:         {}", u.bio.as_deref().unwrap_or("N/A"));
                    println!("大頭貼:       {}", u.avatar.as_deref().unwrap_or("N/A"));
                    println!("粉絲:         {}", followers);
                    println!("追蹤中:       {}", following);
                    println!("建立時間:     {}", u.created_at);
                    println!("更新時間:     {}", u.updated_at);
                }
                None => println!("使用者 #{} 不存在。", id),
            }
        }
        UserSubcommands::Update { id, display_name, bio } => {
            match user::update_user(conn, *id, display_name.as_deref(), bio.as_deref()) {
                Ok(true) => println!("{}", fmt::success_msg(&format!("使用者 #{} 已更新。", id))),
                Ok(false) => println!("使用者 #{} 不存在。", id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        UserSubcommands::Delete { id } => {
            if user::delete_user(conn, *id)? {
                println!("{}", fmt::success_msg(&format!("使用者 #{} 已刪除。", id)));
            } else {
                println!("使用者 #{} 不存在。", id);
            }
        }
    }
    Ok(())
}
