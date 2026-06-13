use crate::cli::fmt;
use crate::model::message;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct MessageCommand {
    #[command(subcommand)]
    pub subcommand: MessageSubcommands,
}

#[derive(Subcommand)]
pub enum MessageSubcommands {
    Send {
        sender_id: i64,
        receiver_id: i64,
        content: String,
    },
    Inbox {
        user_id: i64,
    },
    Conversation {
        user_id: i64,
        other_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &MessageSubcommands) -> Result<()> {
    match cmd {
        MessageSubcommands::Send { sender_id, receiver_id, content } => {
            match message::send_message(conn, *sender_id, *receiver_id, content) {
                Ok(id) => println!("{}", fmt::success_msg(&format!("已傳送訊息 #{}", id))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        MessageSubcommands::Inbox { user_id } => {
            let unread = message::get_unread_count(conn, *user_id)?;
            let convs = message::list_conversations(conn, *user_id)?;
            if convs.is_empty() {
                println!("{}", fmt::info_msg("尚無對話。"));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("對話列表 (未讀 {}):", unread)));
            println!("{}", fmt::header(&format!("{:<4} {:<20} {:<10} {:<30} {:<20}", "ID", "名稱", "未讀", "最後訊息", "時間")));
            println!("{}", "-".repeat(90));
            for c in &convs {
                println!("{:<4} {:<20} {:<10} {:<30} {:<20}",
                    c.other_user_id,
                    c.other_display_name,
                    if c.unread_count > 0 { format!("🔴 {}", c.unread_count) } else { "".into() },
                    if c.last_message.len() > 28 { format!("{}...", &c.last_message[..27]) } else { c.last_message.clone() },
                    &c.last_message_at[..19],
                );
            }
        }
        MessageSubcommands::Conversation { user_id, other_id } => {
            message::mark_read(conn, *user_id, *other_id)?;
            let msgs = message::list_messages(conn, *user_id, *other_id)?;
            if msgs.is_empty() {
                println!("{}", fmt::info_msg("尚無訊息。"));
                return Ok(());
            }
            for m in &msgs {
                let tag = if m.sender_id == *user_id { "→" } else { "←" };
                println!("{} @{}: {}", tag, m.sender_username, m.content);
                println!("   {}", m.created_at);
                println!();
            }
        }
    }
    Ok(())
}
