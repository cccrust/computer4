use crate::cli::fmt;
use crate::model::interest;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct InterestCommand {
    #[command(subcommand)]
    pub subcommand: InterestSubcommands,
}

#[derive(Subcommand)]
pub enum InterestSubcommands {
    Add {
        user_id: i64,
        tag: String,
    },
    Remove {
        user_id: i64,
        tag: String,
    },
    List {
        user_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &InterestSubcommands) -> Result<()> {
    match cmd {
        InterestSubcommands::Add { user_id, tag } => {
            match interest::add_interest(conn, *user_id, tag) {
                Ok(id) => println!("{}", fmt::success_msg(&format!("已新增興趣標籤 #{}", id))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        InterestSubcommands::Remove { user_id, tag } => {
            if interest::remove_interest(conn, *user_id, tag)? {
                println!("{}", fmt::success_msg(&format!("已移除興趣標籤「{}」", tag)));
            } else {
                println!("{}", fmt::error_msg(&format!("興趣標籤「{}」不存在", tag)));
            }
        }
        InterestSubcommands::List { user_id } => {
            let list = interest::list_interests(conn, *user_id)?;
            if list.is_empty() {
                println!("{}", fmt::info_msg("尚無興趣標籤。"));
                return Ok(());
            }
            println!("{}", fmt::header("興趣標籤"));
            for (i, item) in list.iter().enumerate() {
                println!("{}. {}", i + 1, item.tag);
            }
        }
    }
    Ok(())
}
