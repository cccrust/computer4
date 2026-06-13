use colored::*;

pub fn thousands(n: f64) -> String {
    let s = format!("{:.2}", n);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = parts[1];
    let mut result = String::new();
    for (i, c) in int_part.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    format!("{}.{}", result.chars().rev().collect::<String>(), dec_part)
}

pub fn header(text: &str) -> String {
    text.bold().yellow().to_string()
}

pub fn status_color(s: &str) -> ColoredString {
    match s {
        "cancelled" => s.red(),
        "paid" | "delivered" | "received" => s.green(),
        "overdue" => s.red().bold(),
        "confirmed" | "approved" => s.cyan(),
        _ => s.normal(),
    }
}

pub fn error_msg(msg: &str) -> String {
    format!("{} {}", "Error:".red().bold(), msg)
}

pub fn format_csv_line(values: &[String]) -> String {
    values
        .iter()
        .map(|v| {
            if v.contains(',') || v.contains('"') || v.contains('\n') {
                format!("\"{}\"", v.replace('"', "\"\""))
            } else {
                v.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thousands_basic() {
        assert_eq!(thousands(0.0), "0.00");
        assert_eq!(thousands(1.0), "1.00");
        assert_eq!(thousands(999.0), "999.00");
    }

    #[test]
    fn test_thousands_with_commas() {
        assert_eq!(thousands(1000.0), "1,000.00");
        assert_eq!(thousands(1234567.89), "1,234,567.89");
        assert_eq!(thousands(1000000.0), "1,000,000.00");
    }

    #[test]
    fn test_thousands_large() {
        assert_eq!(thousands(9876543210.12), "9,876,543,210.12");
    }

    #[test]
    fn test_format_csv_line_simple() {
        let row = vec!["1".into(), "Alice".into(), "alice@test.com".into()];
        assert_eq!(format_csv_line(&row), "1,Alice,alice@test.com");
    }

    #[test]
    fn test_format_csv_line_quotes() {
        let row = vec!["1".into(), "Alice, Inc.".into(), "alice@test.com".into()];
        assert_eq!(format_csv_line(&row), "1,\"Alice, Inc.\",alice@test.com");
    }

    #[test]
    fn test_format_csv_line_embedded_quote() {
        let row = vec!["he said \"hello\"".into()];
        assert_eq!(format_csv_line(&row), "\"he said \"\"hello\"\"\"");
    }

    #[test]
    fn test_status_color_variants() {
        let out = format!("{}", status_color("paid"));
        assert!(out.contains("paid"));
        let out = format!("{}", status_color("pending"));
        assert_eq!(out, "pending");
    }

    #[test]
    fn test_error_msg_format() {
        let msg = error_msg("Something went wrong");
        assert!(msg.contains("Error:"));
        assert!(msg.contains("Something went wrong"));
    }
}
