use std::fs;

use anyhow::{bail, Result};

/// Simple CSV parser supporting quoted fields with embedded commas and quotes.
pub fn parse_csv(filepath: &str) -> Result<(Vec<String>, Vec<Vec<String>>)> {
    let content = fs::read_to_string(filepath)?;
    let content = content.trim_start_matches('\u{feff}').to_string();
    let mut lines: Vec<Vec<String>> = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        lines.push(parse_csv_line(trimmed));
    }
    if lines.is_empty() {
        bail!("CSV 檔案為空");
    }
    let header = lines.remove(0);
    Ok((header, lines))
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if !in_quotes => in_quotes = true,
            '"' if in_quotes => {
                if chars.peek() == Some(&'"') {
                    current.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            }
            ',' if !in_quotes => {
                fields.push(current.trim().to_string());
                current = String::new();
            }
            _ => current.push(ch),
        }
    }
    fields.push(current.trim().to_string());
    fields
}

pub fn import_customers(conn: &rusqlite::Connection, filepath: &str) -> Result<(usize, usize)> {
    let (header, rows) = parse_csv(filepath)?;
    let col_map = map_columns(&header, &["name", "email", "phone", "address"])?;
    let mut success = 0usize;
    let mut errors = 0usize;
    for row in &rows {
        let name = row.get(col_map["name"]).map(|s| s.as_str()).unwrap_or("");
        if name.is_empty() {
            errors += 1;
            continue;
        }
        let email = col_map
            .get("email")
            .and_then(|&i| row.get(i))
            .map(|s| s.as_str());
        let phone = col_map
            .get("phone")
            .and_then(|&i| row.get(i))
            .map(|s| s.as_str());
        let address = col_map
            .get("address")
            .and_then(|&i| row.get(i))
            .map(|s| s.as_str());
        match crate::model::customer::create_customer(conn, name, email, phone, address) {
            Ok(_) => success += 1,
            Err(_) => errors += 1,
        }
    }
    Ok((success, errors))
}

pub fn import_products(conn: &rusqlite::Connection, filepath: &str) -> Result<(usize, usize)> {
    let (header, rows) = parse_csv(filepath)?;
    let col_map = map_columns(&header, &["name", "sku", "price", "stock", "description"])?;
    let mut success = 0usize;
    let mut errors = 0usize;
    for row in &rows {
        let name = row.get(col_map["name"]).map(|s| s.as_str()).unwrap_or("");
        let sku = row.get(col_map["sku"]).map(|s| s.as_str()).unwrap_or("");
        if name.is_empty() || sku.is_empty() {
            errors += 1;
            continue;
        }
        let price: f64 = col_map
            .get("price")
            .and_then(|&i| row.get(i))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0);
        let stock: i64 = col_map
            .get("stock")
            .and_then(|&i| row.get(i))
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0);
        let desc = col_map
            .get("description")
            .and_then(|&i| row.get(i))
            .map(|s| s.as_str());
        match crate::model::product::create_product(conn, name, sku, price, stock, desc) {
            Ok(_) => success += 1,
            Err(_) => errors += 1,
        }
    }
    Ok((success, errors))
}

pub fn import_suppliers(conn: &rusqlite::Connection, filepath: &str) -> Result<(usize, usize)> {
    let (header, rows) = parse_csv(filepath)?;
    let col_map = map_columns(
        &header,
        &["name", "contact_person", "email", "phone", "address"],
    )?;
    let mut success = 0usize;
    let mut errors = 0usize;
    for row in &rows {
        let name = row.get(col_map["name"]).map(|s| s.as_str()).unwrap_or("");
        if name.is_empty() {
            errors += 1;
            continue;
        }
        let contact = col_map
            .get("contact_person")
            .and_then(|&i| row.get(i))
            .map(|s| s.as_str());
        let email = col_map
            .get("email")
            .and_then(|&i| row.get(i))
            .map(|s| s.as_str());
        let phone = col_map
            .get("phone")
            .and_then(|&i| row.get(i))
            .map(|s| s.as_str());
        let address = col_map
            .get("address")
            .and_then(|&i| row.get(i))
            .map(|s| s.as_str());
        match crate::model::supplier::create_supplier(conn, name, contact, email, phone, address) {
            Ok(_) => success += 1,
            Err(_) => errors += 1,
        }
    }
    Ok((success, errors))
}

fn map_columns(
    header: &[String],
    expected: &[&str],
) -> Result<std::collections::HashMap<String, usize>> {
    let mut map = std::collections::HashMap::new();
    let header_lower: Vec<String> = header
        .iter()
        .map(|h| h.to_lowercase().trim().to_string())
        .collect();
    for &col in expected {
        if let Some(pos) = header_lower.iter().position(|h| h == col) {
            map.insert(col.to_string(), pos);
        }
    }
    if !map.contains_key(expected[0]) {
        bail!("CSV 缺少必要欄位 '{}'。標題列: {:?}", expected[0], header);
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_line_simple() {
        let fields = parse_csv_line("a,b,c");
        assert_eq!(fields, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_csv_line_quoted() {
        let fields = parse_csv_line("\"a,b\",c");
        assert_eq!(fields, vec!["a,b", "c"]);
    }

    #[test]
    fn test_parse_csv_line_escaped_quote() {
        let fields = parse_csv_line("\"he said \"\"hello\"\"\",end");
        assert_eq!(fields, vec!["he said \"hello\"", "end"]);
    }
}
