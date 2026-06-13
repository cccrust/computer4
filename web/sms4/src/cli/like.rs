use crate::cli::fmt;
use crate::model::like;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct LikeCommand {
    #[command(subcommand)]
    pub subcommand: LikeSubcommands,
}

#[derive(Subcommand)]
pub enum LikeSubcommands {
    Add {
        user_id: i64,
        post_id: i64,
    },
    Remove {
        user_id: i64,
        post_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &LikeSubcommands) -> Result<()> {
    match cmd {
        LikeSubcommands::Add { user_id, post_id } => {
            if like::like_post(conn, *user_id, *post_id)? {
                println!("{}", fmt::success_msg(&format!("使用者 #{} 對貼文 #{} 按讚", user_id, post_id)));
            } else {
                println!("{}", fmt::info_msg("已經按過讚了"));
            }
        }
        LikeSubcommands::Remove { user_id, post_id } => {
            if like::unlike_post(conn, *user_id, *post_id)? {
                println!("{}", fmt::success_msg(&format!("使用者 #{} 取消對貼文 #{} 的讚", user_id, post_id)));
            } else {
                println!("{}", fmt::info_msg("尚未按讚"));
            }
        }
    }
    Ok(())
}
