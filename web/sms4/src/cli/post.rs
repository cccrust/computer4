use crate::cli::fmt;
use crate::model::post;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct PostCommand {
    #[command(subcommand)]
    pub subcommand: PostSubcommands,
}

#[derive(Subcommand)]
pub enum PostSubcommands {
    Add {
        user_id: i64,
        content: String,
    },
    Reply {
        post_id: i64,
        user_id: i64,
        content: String,
    },
    List {
        #[arg(long)]
        user_id: Option<i64>,
        #[arg(long, default_value_t = 20)]
        limit: i64,
        #[arg(long, default_value_t = 0)]
        offset: i64,
    },
    Get {
        id: i64,
    },
    Timeline {
        user_id: i64,
        #[arg(long, default_value_t = 20)]
        limit: i64,
        #[arg(long, default_value_t = 0)]
        offset: i64,
    },
    Delete {
        id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &PostSubcommands) -> Result<()> {
    match cmd {
        PostSubcommands::Add { user_id, content } => {
            match post::create_post(conn, *user_id, content, None) {
                Ok(id) => println!("{}", fmt::success_msg(&format!("已發布貼文 #{}", id))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        PostSubcommands::Reply { post_id, user_id, content } => {
            match post::create_post(conn, *user_id, content, Some(*post_id)) {
                Ok(id) => println!("{}", fmt::success_msg(&format!("已回覆貼文 #{} (回覆 #{})", post_id, id))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        PostSubcommands::List { user_id, limit, offset } => {
            let posts = post::list_posts(conn, *user_id, *limit, *offset)?;
            if posts.is_empty() {
                println!("{}", fmt::info_msg("尚無貼文。"));
                return Ok(());
            }
            for p in &posts {
                println!("{}", fmt::header(&format!("#{}{}", p.id, if p.replies_count > 0 { format!(" ({} 則回覆)", p.replies_count) } else { String::new() })));
                println!("  @{} ({})", p.username, p.display_name);
                println!("  {}", p.content);
                println!("  ❤ {}  {}", p.likes_count, p.created_at);
                println!();
            }
        }
        PostSubcommands::Get { id } => {
            match post::get_post(conn, *id)? {
                Some(p) => {
                    println!("{}", fmt::header(&format!("#{}", p.id)));
                    println!("  @{} ({})", p.username, p.display_name);
                    println!("  {}", p.content);
                    println!("  ❤ {}  💬 {}  {}", p.likes_count, p.replies_count, p.created_at);
                    println!();
                    if p.replies_count > 0 {
                        println!("{}", fmt::header("回覆:"));
                        let replies = post::list_replies(conn, *id)?;
                        for r in &replies {
                            println!("  #{} @{}: {}", r.id, r.username, r.content);
                            println!("         ❤ {}  {}", r.likes_count, r.created_at);
                        }
                    }
                }
                None => println!("貼文 #{} 不存在。", id),
            }
        }
        PostSubcommands::Timeline { user_id, limit, offset } => {
            let posts = post::get_timeline(conn, *user_id, *limit, *offset)?;
            if posts.is_empty() {
                println!("{}", fmt::info_msg("時間軸尚無貼文。試試追蹤更多使用者！"));
                return Ok(());
            }
            for p in &posts {
                println!("{}", fmt::header(&format!("#{}", p.id)));
                println!("  @{} ({})", p.username, p.display_name);
                println!("  {}", p.content);
                println!("  ❤ {}  💬 {}  {}", p.likes_count, p.replies_count, p.created_at);
                println!();
            }
        }
        PostSubcommands::Delete { id } => {
            if post::delete_post(conn, *id)? {
                println!("{}", fmt::success_msg(&format!("貼文 #{} 已刪除。", id)));
            } else {
                println!("貼文 #{} 不存在。", id);
            }
        }
    }
    Ok(())
}
