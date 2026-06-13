use crate::cli::fmt;
use crate::model::invoice;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct InvoiceCommand {
    #[command(subcommand)]
    pub subcommand: InvoiceSubcommands,
}

#[derive(Subcommand)]
pub enum InvoiceSubcommands {
    Create {
        /// Invoice number (omit for auto-generate)
        invoice_number: Option<String>,
        #[arg(long)]
        order_id: Option<i64>,
        #[arg(long)]
        customer_id: i64,
        #[arg(long)]
        due_date: String,
        #[arg(long)]
        amount: f64,
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
}

pub fn run(conn: &Connection, cmd: &InvoiceSubcommands) -> Result<()> {
    match cmd {
        InvoiceSubcommands::Create {
            invoice_number,
            order_id,
            customer_id,
            due_date,
            amount,
            notes,
        } => {
            match invoice::create_invoice(
                conn,
                invoice_number.as_deref(),
                *order_id,
                *customer_id,
                due_date,
                *amount,
                notes.as_deref(),
            ) {
                Ok(id) => println!("已建立發票 #{}", id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        InvoiceSubcommands::List {
            status,
            customer_id,
            format,
            page,
            page_size,
        } => {
            let invoices =
                invoice::list_invoices(conn, status.as_deref(), *customer_id, *page, *page_size)?;
            if invoices.is_empty() {
                println!("查無發票資料。");
                return Ok(());
            }
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
            }
        }
        InvoiceSubcommands::Get { id, format } => match invoice::get_invoice(conn, *id)? {
            Some(inv) => {
                if format == "json" {
                    println!("{}", serde_json::to_string_pretty(&inv)?);
                } else {
                    println!("ID:            {}", inv.id);
                    println!("發票號碼:      {}", inv.invoice_number);
                    println!(
                        "訂單 ID:       {}",
                        inv.order_id
                            .map(|x| x.to_string())
                            .as_deref()
                            .unwrap_or("N/A")
                    );
                    println!("客戶 ID:       {}", inv.customer_id);
                    println!("發票日期:      {}", inv.invoice_date);
                    println!("到期日:        {}", inv.due_date);
                    println!("狀態:          {}", fmt::status_color(&inv.status));
                    println!("金額:          {}", fmt::thousands(inv.amount));
                    println!("備註:          {}", inv.notes.as_deref().unwrap_or("N/A"));
                    println!("建立時間:      {}", inv.created_at);
                    println!("更新時間:      {}", inv.updated_at);
                }
            }
            None => println!("發票 #{} 不存在。", id),
        },
        InvoiceSubcommands::UpdateStatus { id, status } => {
            match invoice::update_invoice_status(conn, *id, status) {
                Ok(true) => println!(
                    "發票 #{} 狀態已更新為 '{}'。",
                    id,
                    fmt::status_color(status)
                ),
                Ok(false) => println!("發票 #{} 不存在。", id),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        InvoiceSubcommands::Delete { id } => {
            if invoice::delete_invoice(conn, *id)? {
                println!("發票 #{} 已刪除。", id);
            } else {
                println!("發票 #{} 不存在。", id);
            }
        }
    }
    Ok(())
}
