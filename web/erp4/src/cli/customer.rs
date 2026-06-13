use crate::cli::fmt;
use crate::cli::import;
use crate::model::customer;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct CustomerCommand {
    #[command(subcommand)]
    pub subcommand: CustomerSubcommands,
}

#[derive(Subcommand)]
pub enum CustomerSubcommands {
    Add {
        name: String,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        phone: Option<String>,
        #[arg(long)]
        address: Option<String>,
    },
    List {
        #[arg(long, short)]
        search: Option<String>,
        #[arg(long, default_value = "table")]
        format: String,
        #[arg(long)]
        page: Option<i64>,
        #[arg(long)]
        page_size: Option<i64>,
    },
    Get {
        id: i64,
        #[arg(long, default_value = "table")]
        format: String,
    },
    Update {
        id: i64,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        phone: Option<String>,
        #[arg(long)]
        address: Option<String>,
    },
    Delete {
        id: i64,
    },
    Import {
        file: String,
    },
}

pub fn run(conn: &Connection, cmd: &CustomerSubcommands) -> Result<()> {
    match cmd {
        CustomerSubcommands::Add {
            name,
            email,
            phone,
            address,
        } => {
            match customer::create_customer(
                conn,
                name,
                email.as_deref(),
                phone.as_deref(),
                address.as_deref(),
            ) {
                Ok(id) => println!("已建立客戶 #{}: {}", id, name),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        CustomerSubcommands::List {
            search,
            format,
            page,
            page_size,
        } => {
            let customers = customer::list_customers(conn, search.as_deref(), *page, *page_size)?;
            if customers.is_empty() {
                println!("查無客戶資料。");
                return Ok(());
            }
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&customers)?);
            } else if format == "csv" {
                println!(
                    "{}",
                    fmt::format_csv_line(&[
                        "ID".into(),
                        "Name".into(),
                        "Email".into(),
                        "Phone".into(),
                        "Address".into()
                    ])
                );
                for c in &customers {
                    println!(
                        "{}",
                        fmt::format_csv_line(&[
                            c.id.to_string(),
                            c.name.clone(),
                            c.email.as_deref().unwrap_or("").to_string(),
                            c.phone.as_deref().unwrap_or("").to_string(),
                            c.address.as_deref().unwrap_or("").to_string(),
                        ])
                    );
                }
            } else {
                println!(
                    "{}",
                    fmt::header(&format!(
                        "{:<4} {:<20} {:<25} {:<15} {:<30}",
                        "ID", "名稱", "Email", "電話", "地址"
                    ))
                );
                println!("{}", "-".repeat(100));
                for c in &customers {
                    println!(
                        "{:<4} {:<20} {:<25} {:<15} {:<30}",
                        c.id,
                        c.name,
                        c.email.as_deref().unwrap_or(""),
                        c.phone.as_deref().unwrap_or(""),
                        c.address.as_deref().unwrap_or("")
                    );
                }
            }
        }
        CustomerSubcommands::Get { id, format } => match customer::get_customer(conn, *id)? {
            Some(c) => {
                if format == "json" {
                    println!("{}", serde_json::to_string_pretty(&c)?);
                } else {
                    println!("ID:        {}", c.id);
                    println!("名稱:      {}", c.name);
                    println!("Email:     {}", c.email.as_deref().unwrap_or("N/A"));
                    println!("電話:      {}", c.phone.as_deref().unwrap_or("N/A"));
                    println!("地址:      {}", c.address.as_deref().unwrap_or("N/A"));
                    println!("建立時間:  {}", c.created_at);
                    println!("更新時間:  {}", c.updated_at);
                }
            }
            None => println!("客戶 #{} 不存在。", id),
        },
        CustomerSubcommands::Update {
            id,
            name,
            email,
            phone,
            address,
        } => {
            match customer::update_customer(
                conn,
                *id,
                name.as_deref(),
                email.as_deref(),
                phone.as_deref(),
                address.as_deref(),
            ) {
                Ok(true) => println!("客戶 #{} 已更新。", id),
                Ok(false) => println!("客戶 #{} 不存在。", id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        CustomerSubcommands::Delete { id } => {
            if customer::delete_customer(conn, *id)? {
                println!("客戶 #{} 已刪除。", id);
            } else {
                println!("客戶 #{} 不存在。", id);
            }
        }
        CustomerSubcommands::Import { file } => {
            let content = std::fs::read_to_string(file)?;
            let (count, errors) = import::import_customers(conn, &content)?;
            println!("客戶匯入完成：成功 {} 筆，失敗 {} 筆", count, errors);
        }
    }
    Ok(())
}
