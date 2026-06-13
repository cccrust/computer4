use crate::cli::fmt;
use crate::model::follow;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct FollowCommand {
    #[command(subcommand)]
    pub subcommand: FollowSubcommands,
}

#[derive(Subcommand)]
pub enum FollowSubcommands {
    Add {
        follower_id: i64,
        followee_id: i64,
    },
    Remove {
        follower_id: i64,
        followee_id: i64,
    },
    Followers {
        user_id: i64,
    },
    Following {
        user_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &FollowSubcommands) -> Result<()> {
    match cmd {
        FollowSubcommands::Add { follower_id, followee_id } => {
            match follow::follow_user(conn, *follower_id, *followee_id) {
                Ok(()) => println!("{}", fmt::success_msg(&format!("使用者 #{} 已追蹤 #{}", follower_id, followee_id))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        FollowSubcommands::Remove { follower_id, followee_id } => {
            if follow::unfollow_user(conn, *follower_id, *followee_id)? {
                println!("{}", fmt::success_msg(&format!("使用者 #{} 已取消追蹤 #{}", follower_id, followee_id)));
            } else {
                println!("{}", fmt::info_msg("尚無此追蹤關係。"));
            }
        }
        FollowSubcommands::Followers { user_id } => {
            let users = follow::list_followers(conn, *user_id)?;
            if users.is_empty() {
                println!("{}", fmt::info_msg(&format!("使用者 #{} 尚無粉絲。", user_id)));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("粉絲 ({} 人):", users.len())));
            for u in &users {
                println!("  #{} @{} ({})", u.id, u.username, u.display_name);
            }
        }
        FollowSubcommands::Following { user_id } => {
            let users = follow::list_following(conn, *user_id)?;
            if users.is_empty() {
                println!("{}", fmt::info_msg(&format!("使用者 #{} 尚未追蹤任何人。", user_id)));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("追蹤中 ({} 人):", users.len())));
            for u in &users {
                println!("  #{} @{} ({})", u.id, u.username, u.display_name);
            }
        }
    }
    Ok(())
}
