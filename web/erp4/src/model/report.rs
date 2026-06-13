use crate::model::invoice;
use crate::model::product;
use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SalesRow {
    pub product_id: i64,
    pub product_name: String,
    pub total_qty: i64,
    pub total_amount: f64,
}

pub fn sales_report(
    conn: &Connection,
    from: Option<&str>,
    to: Option<&str>,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Result<Vec<SalesRow>> {
    let mut sql = "SELECT oi.product_id, p.name, SUM(oi.quantity), SUM(oi.quantity * oi.unit_price)
        FROM order_items oi
        JOIN orders o ON o.id = oi.order_id
        JOIN products p ON p.id = oi.product_id
        WHERE o.status IN ('confirmed', 'shipped', 'delivered')"
        .to_string();
    let mut args: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(f) = from {
        args.push(Box::new(f.to_string()));
        sql.push_str(&format!(" AND o.order_date >= ?{}", args.len()));
    }
    if let Some(t) = to {
        args.push(Box::new(t.to_string()));
        sql.push_str(&format!(" AND o.order_date <= ?{}", args.len()));
    }
    sql.push_str(" GROUP BY oi.product_id ORDER BY total_amount DESC");
    if let (Some(ps), Some(p)) = (page_size, page) {
        sql.push_str(&format!(
            " LIMIT ?{} OFFSET ?{}",
            args.len() + 1,
            args.len() + 2
        ));
        args.push(Box::new(ps));
        args.push(Box::new((p - 1) * ps));
    }
    let mut stmt = conn.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = args.iter().map(|a| a.as_ref()).collect();
    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(SalesRow {
            product_id: row.get(0)?,
            product_name: row.get(1)?,
            total_qty: row.get(2)?,
            total_amount: row.get(3)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}

pub fn inventory_report(
    conn: &Connection,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Result<Vec<product::Product>> {
    product::list_products(conn, None, None, page, page_size)
}
