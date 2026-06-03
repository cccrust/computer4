use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::sync::Mutex;

use crate::SharedState;

pub struct TerminalSession {
    pub id: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum TermMsg {
    #[serde(rename = "init")]
    Init { cwd: Option<String> },
    #[serde(rename = "input")]
    Input { data: String },
    #[serde(rename = "resize")]
    Resize { cols: u16, rows: u16 },
}

#[derive(Serialize)]
struct OutMsg<'a> {
    #[serde(rename = "type")]
    kind: &'a str,
    data: &'a str,
}

pub async fn handle_terminal_ws(socket: WebSocket, _state: SharedState) {
    let (mut sender, mut receiver) = socket.split();

    // Wait for init message
    let init_cwd = if let Some(Ok(Message::Text(txt))) = receiver.next().await {
        if let Ok(TermMsg::Init { cwd }) = serde_json::from_str(&txt) {
            cwd.unwrap_or_else(|| std::env::var("HOME").unwrap_or_else(|_| "/".to_string()))
        } else {
            std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
        }
    } else {
        std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
    };

    // Spawn shell
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    
    let mut child = match Command::new(&shell)
        .current_dir(&init_cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("TERM", "xterm-256color")
        .env("PS1", "\\[\\033[1;32m\\]os4web\\[\\033[0m\\]:\\[\\033[1;34m\\]\\w\\[\\033[0m\\]$ ")
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            let msg = format!("Failed to spawn shell: {}", e);
            let _ = sender.send(Message::Text(
                serde_json::to_string(&OutMsg { kind: "output", data: &msg }).unwrap()
            )).await;
            return;
        }
    };

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    let sender = Arc::new(Mutex::new(sender));

    // Stdout reader task
    let sender_out = sender.clone();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 4096];
        loop {
            match stdout.read(&mut buf).await {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buf[..n]).to_string();
                    let msg = serde_json::to_string(&OutMsg { kind: "output", data: &text }).unwrap();
                    let mut s = sender_out.lock().await;
                    if s.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    // Stderr reader task
    let sender_err = sender.clone();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 4096];
        loop {
            match stderr.read(&mut buf).await {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buf[..n]).to_string();
                    let msg = serde_json::to_string(&OutMsg { kind: "output", data: &text }).unwrap();
                    let mut s = sender_err.lock().await;
                    if s.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    // Handle incoming messages from client
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(txt) => {
                if let Ok(term_msg) = serde_json::from_str::<TermMsg>(&txt) {
                    match term_msg {
                        TermMsg::Input { data } => {
                            if stdin.write_all(data.as_bytes()).await.is_err() {
                                break;
                            }
                        }
                        TermMsg::Resize { .. } => {
                            // PTY resize would require nix/pty crate; skip for now
                        }
                        _ => {}
                    }
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    let _ = child.kill().await;
}
