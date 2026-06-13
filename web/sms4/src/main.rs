#![allow(dead_code, unused)]

mod cli;
mod db;
mod model;
mod web;

use anyhow::Result;
use clap::Parser;
use rusqlite::Connection;
use std::path::PathBuf;

fn get_db_path() -> PathBuf {
    let path = std::env::var("SMS4_DB").unwrap_or_else(|_| "sms4.db".to_string());
    PathBuf::from(path)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let db_path = get_db_path();
    let conn = Connection::open(&db_path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    match &cli.command {
        cli::Commands::Init => {
            db::init_db(&conn)?;
            println!("資料庫已初始化：{}", db_path.display());
        }
        cli::Commands::User(cmd) => {
            cli::user::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Post(cmd) => {
            cli::post::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Follow(cmd) => {
            cli::follow::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Like(cmd) => {
            cli::like::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Message(cmd) => {
            cli::message::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Profile(cmd) => {
            cli::profile::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Interest(cmd) => {
            cli::interest::run(&conn, &cmd.subcommand)?;
        }
        cli::Commands::Web { port, host, dev } => {
            web::start(conn, host, *port, *dev).await;
        }
    }

    Ok(())
}
