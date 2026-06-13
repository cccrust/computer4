use anyhow::Result;
use clap::Parser;
use rusqlite::Connection;
use serde::Serialize;

use crate::cli::fmt;
use crate::model::{customer, invoice, order, product, purchase_order, supplier};

#[derive(Parser)]
pub struct ExportCommand {
    #[arg(long, default_value = "json")]
    pub format: String,
}

pub fn run(conn: &Connection, cmd: &ExportCommand) -> Result<()> {
    if cmd.format == "csv" {
        println!("{}", export_csv(conn)?);
    } else {
        println!("{}", export_json(conn)?);
    }
    Ok(())
}

#[derive(Serialize)]
struct ExportData {
    customers: Vec<customer::Customer>,
    products: Vec<product::Product>,
    suppliers: Vec<supplier::Supplier>,
    orders: Vec<order::Order>,
    purchase_orders: Vec<purchase_order::PurchaseOrder>,
    invoices: Vec<invoice::Invoice>,
}

pub fn export_json(conn: &Connection) -> Result<String> {
    let data = ExportData {
        customers: customer::list_customers(conn, None, None, None)?,
        products: product::list_products(conn, None, None, None, None)?,
        suppliers: supplier::list_suppliers(conn, None, None)?,
        orders: order::list_orders(conn, None, None, None, None)?,
        purchase_orders: purchase_order::list_purchase_orders(conn, None, None, None)?,
        invoices: invoice::list_invoices(conn, None, None, None, None)?,
    };
    Ok(serde_json::to_string_pretty(&data)?)
}

fn csv_table(name: &str, header: &[String], rows: &[Vec<String>]) -> Vec<String> {
    let mut out = Vec::new();
    out.push(format!("# {}", name));
    out.push(fmt::format_csv_line(header));
    for row in rows {
        out.push(fmt::format_csv_line(row));
    }
    out.push(String::new());
    out
}

pub fn export_csv(conn: &Connection) -> Result<String> {
    let mut parts = Vec::new();

    let customers = customer::list_customers(conn, None, None, None)?;
    let cust_rows: Vec<Vec<String>> = customers
        .iter()
        .map(|c| {
            vec![
                c.id.to_string(),
                c.name.clone(),
                c.email.clone().unwrap_or_default(),
                c.phone.clone().unwrap_or_default(),
                c.address.clone().unwrap_or_default(),
            ]
        })
        .collect();
    parts.extend(csv_table(
        "customers",
        &["ID", "Name", "Email", "Phone", "Address"].map(String::from),
        &cust_rows,
    ));

    let products = product::list_products(conn, None, None, None, None)?;
    let prod_rows: Vec<Vec<String>> = products
        .iter()
        .map(|p| {
            vec![
                p.id.to_string(),
                p.name.clone(),
                p.sku.clone(),
                format!("{:.2}", p.price),
                p.stock.to_string(),
                p.description.clone().unwrap_or_default(),
            ]
        })
        .collect();
    parts.extend(csv_table(
        "products",
        &["ID", "Name", "SKU", "Price", "Stock", "Description"].map(String::from),
        &prod_rows,
    ));

    let suppliers = supplier::list_suppliers(conn, None, None)?;
    let supp_rows: Vec<Vec<String>> = suppliers
        .iter()
        .map(|s| {
            vec![
                s.id.to_string(),
                s.name.clone(),
                s.contact_person.clone().unwrap_or_default(),
                s.email.clone().unwrap_or_default(),
                s.phone.clone().unwrap_or_default(),
            ]
        })
        .collect();
    parts.extend(csv_table(
        "suppliers",
        &["ID", "Name", "Contact", "Email", "Phone"].map(String::from),
        &supp_rows,
    ));

    Ok(parts.join("\n"))
}
