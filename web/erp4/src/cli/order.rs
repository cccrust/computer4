use crate::cli::fmt;
use crate::model::order;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct OrderCommand {
    #[command(subcommand)]
    pub subcommand: OrderSubcommands,
}

#[derive(Subcommand)]
pub enum OrderSubcommands {
    Create {
        customer_id: i64,
        #[arg(long)]
        notes: Option<String>,
    },
    List {
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        customer_id: Option<i64>,
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
    UpdateStatus {
        id: i64,
        status: String,
    },
    Delete {
        id: i64,
    },
    AddItem {
        order_id: i64,
        product_id: i64,
        quantity: i64,
        #[arg(long)]
        unit_price: Option<f64>,
    },
    Items {
        order_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &OrderSubcommands) -> Result<()> {
    match cmd {
        OrderSubcommands::Create { customer_id, notes } => {
            match order::create_order(conn, *customer_id, notes.as_deref()) {
                Ok(id) => println!("已建立訂單 #{} (客戶 #{})", id, customer_id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        OrderSubcommands::List {
            status,
            customer_id,
            format,
            page,
            page_size,
        } => {
            let orders =
                order::list_orders(conn, status.as_deref(), *customer_id, *page, *page_size)?;
            if orders.is_empty() {
                println!("查無訂單資料。");
                return Ok(());
            }
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&orders)?);
            } else if format == "csv" {
                println!(
                    "{}",
                    fmt::format_csv_line(&[
                        "ID".into(),
                        "CustomerID".into(),
                        "Date".into(),
                        "Status".into(),
                        "Amount".into()
                    ])
                );
                for o in &orders {
                    println!(
                        "{}",
                        fmt::format_csv_line(&[
                            o.id.to_string(),
                            o.customer_id.to_string(),
                            o.order_date.clone(),
                            o.status.clone(),
                            format!("{:.2}", o.total_amount),
                        ])
                    );
                }
            } else {
                println!(
                    "{}",
                    fmt::header(&format!(
                        "{:<4} {:<12} {:<12} {:<12} {:>10}",
                        "ID", "客戶ID", "日期", "狀態", "金額"
                    ))
                );
                println!("{}", "-".repeat(60));
                for o in &orders {
                    println!(
                        "{:<4} {:<12} {:<12} {:<12} {:>10}",
                        o.id,
                        o.customer_id,
                        o.order_date,
                        fmt::status_color(&o.status),
                        fmt::thousands(o.total_amount)
                    );
                }
            }
        }
        OrderSubcommands::Get { id, format } => match order::get_order(conn, *id)? {
            Some(o) => {
                if format == "json" {
                    println!("{}", serde_json::to_string_pretty(&o)?);
                } else {
                    println!("ID:           {}", o.id);
                    println!("客戶 ID:      {}", o.customer_id);
                    println!("訂單日期:     {}", o.order_date);
                    println!("狀態:         {}", fmt::status_color(&o.status));
                    println!("總金額:       {}", fmt::thousands(o.total_amount));
                    println!("備註:         {}", o.notes.as_deref().unwrap_or("N/A"));
                    println!("建立時間:     {}", o.created_at);
                    println!("更新時間:     {}", o.updated_at);

                    let items = order::list_order_items(conn, o.id)?;
                    if !items.is_empty() {
                        println!("\n明細:");
                        println!("{:<4} {:<12} {:>8} {:>10}", "ID", "產品ID", "數量", "單價");
                        println!("{}", "-".repeat(40));
                        for item in &items {
                            println!(
                                "{:<4} {:<12} {:>8} {:>10}",
                                item.id,
                                item.product_id,
                                item.quantity,
                                fmt::thousands(item.unit_price)
                            );
                        }
                    }
                }
            }
            None => println!("訂單 #{} 不存在。", id),
        },
        OrderSubcommands::UpdateStatus { id, status } => {
            match order::update_order_status(conn, *id, status) {
                Ok(true) => println!(
                    "訂單 #{} 狀態已更新為 '{}'。",
                    id,
                    fmt::status_color(status)
                ),
                Ok(false) => println!("訂單 #{} 不存在。", id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        OrderSubcommands::Delete { id } => match order::delete_order(conn, *id) {
            Ok(true) => println!("訂單 #{} 已刪除。", id),
            Ok(false) => println!("訂單 #{} 不存在。", id),
            Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
        },
        OrderSubcommands::AddItem {
            order_id,
            product_id,
            quantity,
            unit_price,
        } => {
            let price = match unit_price {
                Some(p) => *p,
                None => {
                    let prod = match crate::model::product::get_product(conn, *product_id) {
                        Ok(Some(p)) => p,
                        Ok(None) => {
                            println!(
                                "{}",
                                fmt::error_msg(&format!("產品 #{} 不存在。", product_id))
                            );
                            return Ok(());
                        }
                        Err(e) => {
                            println!("{}", fmt::error_msg(&e.to_string()));
                            return Ok(());
                        }
                    };
                    prod.price
                }
            };
            match order::add_order_item(conn, *order_id, *product_id, *quantity, price) {
                Ok(item_id) => println!("已新增明細 #{} 至訂單 #{}", item_id, order_id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        OrderSubcommands::Items { order_id } => {
            let items = order::list_order_items(conn, *order_id)?;
            if items.is_empty() {
                println!("訂單 #{} 無明細資料。", order_id);
                return Ok(());
            }
            println!("{:<4} {:<12} {:>8} {:>10}", "ID", "產品ID", "數量", "單價");
            println!("{}", "-".repeat(40));
            for item in &items {
                println!(
                    "{:<4} {:<12} {:>8} {:>10}",
                    item.id,
                    item.product_id,
                    item.quantity,
                    fmt::thousands(item.unit_price)
                );
            }
        }
    }
    Ok(())
}
