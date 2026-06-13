use crate::cli::fmt;
use crate::cli::import;
use crate::model::product;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct ProductCommand {
    #[command(subcommand)]
    pub subcommand: ProductSubcommands,
}

#[derive(Subcommand)]
pub enum ProductSubcommands {
    Add {
        name: String,
        sku: String,
        price: f64,
        #[arg(long, default_value_t = 0)]
        stock: i64,
        #[arg(long)]
        description: Option<String>,
    },
    List {
        #[arg(long, short)]
        search: Option<String>,
        #[arg(long)]
        low_stock: Option<i64>,
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
        sku: Option<String>,
        #[arg(long)]
        price: Option<f64>,
        #[arg(long)]
        stock: Option<i64>,
        #[arg(long)]
        description: Option<String>,
    },
    Delete {
        id: i64,
    },
    Import {
        file: String,
    },
}

pub fn run(conn: &Connection, cmd: &ProductSubcommands) -> Result<()> {
    match cmd {
        ProductSubcommands::Add {
            name,
            sku,
            price,
            stock,
            description,
        } => {
            match product::create_product(conn, name, sku, *price, *stock, description.as_deref()) {
                Ok(id) => println!("已建立產品 #{}: {} ({})", id, name, sku),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        ProductSubcommands::List {
            search,
            low_stock,
            format,
            page,
            page_size,
        } => {
            let products =
                product::list_products(conn, search.as_deref(), *low_stock, *page, *page_size)?;
            if products.is_empty() {
                println!("查無產品資料。");
                return Ok(());
            }
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&products)?);
            } else if format == "csv" {
                println!(
                    "{}",
                    fmt::format_csv_line(&[
                        "ID".into(),
                        "Name".into(),
                        "SKU".into(),
                        "Price".into(),
                        "Stock".into()
                    ])
                );
                for p in &products {
                    println!(
                        "{}",
                        fmt::format_csv_line(&[
                            p.id.to_string(),
                            p.name.clone(),
                            p.sku.clone(),
                            format!("{:.2}", p.price),
                            p.stock.to_string(),
                        ])
                    );
                }
            } else {
                println!(
                    "{}",
                    fmt::header(&format!(
                        "{:<4} {:<20} {:<12} {:>8} {:>6}",
                        "ID", "名稱", "SKU", "價格", "庫存"
                    ))
                );
                println!("{}", "-".repeat(80));
                for p in &products {
                    println!(
                        "{:<4} {:<20} {:<12} {:>8} {:>6}",
                        p.id,
                        p.name,
                        p.sku,
                        fmt::thousands(p.price),
                        p.stock
                    );
                }
            }
        }
        ProductSubcommands::Get { id, format } => match product::get_product(conn, *id)? {
            Some(p) => {
                if format == "json" {
                    println!("{}", serde_json::to_string_pretty(&p)?);
                } else {
                    println!("ID:          {}", p.id);
                    println!("名稱:        {}", p.name);
                    println!("SKU:         {}", p.sku);
                    println!("價格:        {}", fmt::thousands(p.price));
                    println!("庫存:        {}", p.stock);
                    println!("描述:        {}", p.description.as_deref().unwrap_or("N/A"));
                    println!("建立時間:    {}", p.created_at);
                    println!("更新時間:    {}", p.updated_at);
                }
            }
            None => println!("產品 #{} 不存在。", id),
        },
        ProductSubcommands::Update {
            id,
            name,
            sku,
            price,
            stock,
            description,
        } => {
            match product::update_product(
                conn,
                *id,
                name.as_deref(),
                sku.as_deref(),
                *price,
                *stock,
                description.as_deref(),
            ) {
                Ok(true) => println!("產品 #{} 已更新。", id),
                Ok(false) => println!("產品 #{} 不存在。", id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        ProductSubcommands::Delete { id } => {
            if product::delete_product(conn, *id)? {
                println!("產品 #{} 已刪除。", id);
            } else {
                println!("產品 #{} 不存在。", id);
            }
        }
        ProductSubcommands::Import { file } => {
            let content = std::fs::read_to_string(file)?;
            let (count, errors) = import::import_products(conn, &content)?;
            println!("產品匯入完成：成功 {} 筆，失敗 {} 筆", count, errors);
        }
    }
    Ok(())
}
