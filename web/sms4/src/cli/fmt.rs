use colored::*;

pub fn header(text: &str) -> ColoredString {
    text.bright_cyan().bold()
}

pub fn error_msg(text: &str) -> String {
    format!("{}", text.red().bold())
}

pub fn success_msg(text: &str) -> String {
    format!("{}", text.green().bold())
}

pub fn info_msg(text: &str) -> String {
    format!("{}", text.yellow())
}

pub fn label(text: &str) -> String {
    format!("{}", text.bright_white().bold())
}
