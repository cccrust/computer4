use crate::cli::fmt;
use crate::model::purchase_order;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct PurchaseOrderCommand {
    #[command(subcommand)]
    pub subcommand: PurchaseOrderSubcommands,
}

#[derive(Subcommand)]
pub enum PurchaseOrderSubcommands {
    Create {
        supplier_id: i64,
        #[arg(long)]
        notes: Option<String>,
    },
    List {
        #[arg(long)]
        status: Option<String>,
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
        po_id: i64,
        product_id: i64,
        quantity: i64,
        #[arg(long)]
        unit_price: Option<f64>,
    },
    Items {
        po_id: i64,
    },
}

pub fn run(conn: &Connection, cmd: &PurchaseOrderSubcommands) -> Result<()> {
    match cmd {
        PurchaseOrderSubcommands::Create { supplier_id, notes } => {
            match purchase_order::create_purchase_order(conn, *supplier_id, notes.as_deref()) {
                Ok(id) => println!("已建立採購單 #{} (供應商 #{})", id, supplier_id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        PurchaseOrderSubcommands::List {
            status,
            format,
            page,
            page_size,
        } => {
            let pos =
                purchase_order::list_purchase_orders(conn, status.as_deref(), *page, *page_size)?;
            if pos.is_empty() {
                println!("查無採購單資料。");
                return Ok(());
            }
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&pos)?);
            } else if format == "csv" {
                println!(
                    "{}",
                    fmt::format_csv_line(&[
                        "ID".into(),
                        "SupplierID".into(),
                        "Date".into(),
                        "Status".into(),
                        "Amount".into()
                    ])
                );
                for po in &pos {
                    println!(
                        "{}",
                        fmt::format_csv_line(&[
                            po.id.to_string(),
                            po.supplier_id.to_string(),
                            po.order_date.clone(),
                            po.status.clone(),
                            format!("{:.2}", po.total_amount),
                        ])
                    );
                }
            } else {
                println!(
                    "{}",
                    fmt::header(&format!(
                        "{:<4} {:<12} {:<12} {:<12} {:>10}",
                        "ID", "供應商ID", "日期", "狀態", "金額"
                    ))
                );
                println!("{}", "-".repeat(60));
                for po in &pos {
                    println!(
                        "{:<4} {:<12} {:<12} {:<12} {:>10}",
                        po.id,
                        po.supplier_id,
                        po.order_date,
                        fmt::status_color(&po.status),
                        fmt::thousands(po.total_amount)
                    );
                }
            }
        }
        PurchaseOrderSubcommands::Get { id, format } => {
            match purchase_order::get_purchase_order(conn, *id)? {
                Some(po) => {
                    if format == "json" {
                        println!("{}", serde_json::to_string_pretty(&po)?);
                    } else {
                        println!("ID:           {}", po.id);
                        println!("供應商 ID:    {}", po.supplier_id);
                        println!("訂單日期:     {}", po.order_date);
                        println!("狀態:         {}", fmt::status_color(&po.status));
                        println!("總金額:       {}", fmt::thousands(po.total_amount));
                        println!("備註:         {}", po.notes.as_deref().unwrap_or("N/A"));
                        println!("建立時間:     {}", po.created_at);
                        println!("更新時間:     {}", po.updated_at);

                        let items = purchase_order::list_purchase_order_items(conn, po.id)?;
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
                None => println!("採購單 #{} 不存在。", id),
            }
        }
        PurchaseOrderSubcommands::UpdateStatus { id, status } => {
            match purchase_order::update_purchase_order_status(conn, *id, status) {
                Ok(true) => println!(
                    "採購單 #{} 狀態已更新為 '{}'。",
                    id,
                    fmt::status_color(status)
                ),
                Ok(false) => println!("採購單 #{} 不存在。", id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        PurchaseOrderSubcommands::Delete { id } => {
            match purchase_order::delete_purchase_order(conn, *id) {
                Ok(true) => println!("採購單 #{} 已刪除。", id),
                Ok(false) => println!("採購單 #{} 不存在。", id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        PurchaseOrderSubcommands::AddItem {
            po_id,
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
            match purchase_order::add_purchase_order_item(
                conn,
                *po_id,
                *product_id,
                *quantity,
                price,
            ) {
                Ok(item_id) => println!("已新增明細 #{} 至採購單 #{}", item_id, po_id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        PurchaseOrderSubcommands::Items { po_id } => {
            let items = purchase_order::list_purchase_order_items(conn, *po_id)?;
            if items.is_empty() {
                println!("採購單 #{} 無明細資料。", po_id);
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
