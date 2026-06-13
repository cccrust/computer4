use crate::cli::fmt;
use crate::model::{invoice, product, report};
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct ReportCommand {
    #[command(subcommand)]
    pub subcommand: ReportSubcommands,
}

#[derive(Subcommand)]
pub enum ReportSubcommands {
    Sales {
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
        #[arg(long, default_value = "table")]
        format: String,
        #[arg(long)]
        page: Option<i64>,
        #[arg(long)]
        page_size: Option<i64>,
    },
    Inventory {
        #[arg(long, default_value = "table")]
        format: String,
        #[arg(long)]
        page: Option<i64>,
        #[arg(long)]
        page_size: Option<i64>,
    },
    Aging {
        #[arg(long, default_value = "table")]
        format: String,
    },
}

pub fn run(conn: &Connection, cmd: &ReportSubcommands) -> Result<()> {
    match cmd {
        ReportSubcommands::Sales {
            from,
            to,
            format,
            page,
            page_size,
        } => {
            let rows =
                report::sales_report(conn, from.as_deref(), to.as_deref(), *page, *page_size)?;
            if rows.is_empty() {
                println!("無銷售資料。");
                return Ok(());
            }
            let total_qty: i64 = rows.iter().map(|r| r.total_qty).sum();
            let total_amount: f64 = rows.iter().map(|r| r.total_amount).sum();
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&rows)?);
            } else if format == "csv" {
                println!(
                    "{}",
                    fmt::format_csv_line(&[
                        "ProductID".into(),
                        "Product".into(),
                        "Qty Sold".into(),
                        "Amount".into()
                    ])
                );
                for r in &rows {
                    println!(
                        "{}",
                        fmt::format_csv_line(&[
                            r.product_id.to_string(),
                            r.product_name.clone(),
                            r.total_qty.to_string(),
                            format!("{:.2}", r.total_amount),
                        ])
                    );
                }
                println!(
                    "{}",
                    fmt::format_csv_line(&[
                        "".into(),
                        "Total".into(),
                        total_qty.to_string(),
                        format!("{:.2}", total_amount)
                    ])
                );
            } else {
                println!(
                    "{}",
                    fmt::header(&format!(
                        "{:<4} {:<24} {:>10} {:>14}",
                        "ID", "產品", "銷售量", "金額"
                    ))
                );
                println!("{}", "-".repeat(56));
                for r in &rows {
                    println!(
                        "{:<4} {:<24} {:>10} {:>14}",
                        r.product_id,
                        r.product_name,
                        r.total_qty,
                        fmt::thousands(r.total_amount)
                    );
                }
                println!("{}", "-".repeat(56));
                println!(
                    "{:>4} {:<24} {:>10} {:>14}",
                    "",
                    "總計",
                    total_qty,
                    fmt::thousands(total_amount)
                );
            }
        }
        ReportSubcommands::Inventory {
            format,
            page,
            page_size,
        } => {
            let products = product::list_products(conn, None, None, *page, *page_size)?;
            if products.is_empty() {
                println!("無產品資料。");
                return Ok(());
            }
            let total_value: f64 = products.iter().map(|p| p.price * p.stock as f64).sum();
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
                        "Stock".into(),
                        "Value".into()
                    ])
                );
                for p in &products {
                    let value = p.price * p.stock as f64;
                    println!(
                        "{}",
                        fmt::format_csv_line(&[
                            p.id.to_string(),
                            p.name.clone(),
                            p.sku.clone(),
                            format!("{:.2}", p.price),
                            p.stock.to_string(),
                            format!("{:.2}", value),
                        ])
                    );
                }
                println!(
                    "{}",
                    fmt::format_csv_line(&[
                        "".into(),
                        "".into(),
                        "".into(),
                        "".into(),
                        "Total".into(),
                        format!("{:.2}", total_value)
                    ])
                );
            } else {
                println!(
                    "{}",
                    fmt::header(&format!(
                        "{:<4} {:<24} {:<12} {:>8} {:>6} {:>14}",
                        "ID", "名稱", "SKU", "價格", "庫存", "價值"
                    ))
                );
                println!("{}", "-".repeat(80));
                for p in &products {
                    let value = p.price * p.stock as f64;
                    let stock_warn = if p.stock < 10 { " *" } else { "" };
                    println!(
                        "{:<4} {:<24} {:<12} {:>8} {:>6}{} {:>14}",
                        p.id,
                        p.name,
                        p.sku,
                        fmt::thousands(p.price),
                        p.stock,
                        stock_warn,
                        fmt::thousands(value)
                    );
                }
                println!("{}", "-".repeat(80));
                println!("{:>54} {:>14}", "庫存總價值:", fmt::thousands(total_value));
            }
        }
        ReportSubcommands::Aging { format } => {
            let invoices = invoice::aging_report(conn)?;
            if invoices.is_empty() {
                println!("無逾期未付發票。");
                return Ok(());
            }
            let total: f64 = invoices.iter().map(|i| i.amount).sum();
            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&invoices)?);
            } else if format == "csv" {
                println!(
                    "{}",
                    fmt::format_csv_line(&[
                        "ID".into(),
                        "Invoice#".into(),
                        "CustomerID".into(),
                        "Due Date".into(),
                        "Status".into(),
                        "Amount".into()
                    ])
                );
                for inv in &invoices {
                    println!(
                        "{}",
                        fmt::format_csv_line(&[
                            inv.id.to_string(),
                            inv.invoice_number.clone(),
                            inv.customer_id.to_string(),
                            inv.due_date.clone(),
                            inv.status.clone(),
                            format!("{:.2}", inv.amount),
                        ])
                    );
                }
                println!(
                    "{}",
                    fmt::format_csv_line(&[
                        "".into(),
                        "".into(),
                        "".into(),
                        "".into(),
                        "Total".into(),
                        format!("{:.2}", total)
                    ])
                );
            } else {
                println!(
                    "{}",
                    fmt::header(&format!(
                        "{:<4} {:<15} {:<12} {:<12} {:<10} {:>10}",
                        "ID", "發票號碼", "客戶ID", "到期日", "狀態", "金額"
                    ))
                );
                println!("{}", "-".repeat(75));
                for inv in &invoices {
                    println!(
                        "{:<4} {:<15} {:<12} {:<12} {:<10} {:>10}",
                        inv.id,
                        inv.invoice_number,
                        inv.customer_id,
                        inv.due_date,
                        fmt::status_color(&inv.status),
                        fmt::thousands(inv.amount)
                    );
                }
                println!("{}", "-".repeat(75));
                println!("{:>55} {:>10}", "逾期總金額:", fmt::thousands(total));
            }
        }
    }
    Ok(())
}
