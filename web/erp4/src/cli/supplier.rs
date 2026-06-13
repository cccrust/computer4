use crate::cli::fmt;
use crate::cli::import;
use crate::model::supplier;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct SupplierCommand {
    #[command(subcommand)]
    pub subcommand: SupplierSubcommands,
}

#[derive(Subcommand)]
pub enum SupplierSubcommands {
    Add {
        name: String,
        #[arg(long)]
        contact_person: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        phone: Option<String>,
        #[arg(long)]
        address: Option<String>,
    },
    List {
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
        contact_person: Option<String>,
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

pub fn run(conn: &Connection, cmd: &SupplierSubcommands) -> Result<()> {
    match cmd {
        SupplierSubcommands::Add {
            name,
            contact_person,
            email,
            phone,
            address,
        } => {
            match supplier::create_supplier(
                conn,
                name,
                contact_person.as_deref(),
                email.as_deref(),
                phone.as_deref(),
                address.as_deref(),
            ) {
                Ok(id) => println!("已建立供應商 #{}: {}", id, name),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        SupplierSubcommands::List {
            format,
            page,
            page_size,
        } => {
            let suppliers = supplier::list_suppliers(conn, *page, *page_size)?;
            if suppliers.is_empty() {
                println!("查無供應商資料。");
                return Ok(());
            }
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&suppliers)?);
            } else if format == "csv" {
                println!(
                    "{}",
                    fmt::format_csv_line(&[
                        "ID".into(),
                        "Name".into(),
                        "Contact".into(),
                        "Email".into(),
                        "Phone".into()
                    ])
                );
                for s in &suppliers {
                    println!(
                        "{}",
                        fmt::format_csv_line(&[
                            s.id.to_string(),
                            s.name.clone(),
                            s.contact_person.as_deref().unwrap_or("").to_string(),
                            s.email.as_deref().unwrap_or("").to_string(),
                            s.phone.as_deref().unwrap_or("").to_string(),
                        ])
                    );
                }
            } else {
                println!(
                    "{}",
                    fmt::header(&format!(
                        "{:<4} {:<20} {:<20} {:<25} {:<15}",
                        "ID", "名稱", "聯絡人", "Email", "電話"
                    ))
                );
                println!("{}", "-".repeat(90));
                for s in &suppliers {
                    println!(
                        "{:<4} {:<20} {:<20} {:<25} {:<15}",
                        s.id,
                        s.name,
                        s.contact_person.as_deref().unwrap_or(""),
                        s.email.as_deref().unwrap_or(""),
                        s.phone.as_deref().unwrap_or("")
                    );
                }
            }
        }
        SupplierSubcommands::Get { id, format } => match supplier::get_supplier(conn, *id)? {
            Some(s) => {
                if format == "json" {
                    println!("{}", serde_json::to_string_pretty(&s)?);
                } else {
                    println!("ID:            {}", s.id);
                    println!("名稱:          {}", s.name);
                    println!(
                        "聯絡人:        {}",
                        s.contact_person.as_deref().unwrap_or("N/A")
                    );
                    println!("Email:         {}", s.email.as_deref().unwrap_or("N/A"));
                    println!("電話:          {}", s.phone.as_deref().unwrap_or("N/A"));
                    println!("地址:          {}", s.address.as_deref().unwrap_or("N/A"));
                    println!("建立時間:      {}", s.created_at);
                    println!("更新時間:      {}", s.updated_at);
                }
            }
            None => println!("供應商 #{} 不存在。", id),
        },
        SupplierSubcommands::Update {
            id,
            name,
            contact_person,
            email,
            phone,
            address,
        } => {
            match supplier::update_supplier(
                conn,
                *id,
                name.as_deref(),
                contact_person.as_deref(),
                email.as_deref(),
                phone.as_deref(),
                address.as_deref(),
            ) {
                Ok(true) => println!("供應商 #{} 已更新。", id),
                Ok(false) => println!("供應商 #{} 不存在。", id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        SupplierSubcommands::Delete { id } => {
            if supplier::delete_supplier(conn, *id)? {
                println!("供應商 #{} 已刪除。", id);
            } else {
                println!("供應商 #{} 不存在。", id);
            }
        }
        SupplierSubcommands::Import { file } => {
            let content = std::fs::read_to_string(file)?;
            let (count, errors) = import::import_suppliers(conn, &content)?;
            println!("供應商匯入完成：成功 {} 筆，失敗 {} 筆", count, errors);
        }
    }
    Ok(())
}
