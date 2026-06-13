# AGENTS.md

## 專案概要

Rust + SQLite CLI 手機社群軟體（Threads 風格）。單一 crate，二進位名稱為 `sms4`。

## 開發指令

```bash
cargo build                          # 除錯版
cargo build --release                # 正式版 (target/release/sms4)
cargo test                           # 執行所有測試
cargo fmt                            # 格式化
cargo clippy                         # 靜態分析
cargo check                          # 型別檢查
```

## 環境變數

- `SMS4_DB` — SQLite 資料庫路徑（預設 `sms4.db`）

## 測試

- 單元測試分布於 `src/model/*.rs`
- 使用 In-memory SQLite，Schema 載入來自 `src/db.sql`（`include_str!`）
- 整合測試腳本：`case1.sh`（基礎流程），需手動執行
- 腳本使用 `SMS4=${SMS4:-cargo run --}`，可覆寫為已編譯二進位

## CLI 指令

```bash
sms4 init                              # 初始化資料庫
sms4 user add <username> <name>        # 新增使用者
sms4 user list                         # 列出使用者
sms4 user get <id>                     # 檢視使用者
sms4 user update <id> --bio "..."      # 更新使用者
sms4 user delete <id>                  # 刪除使用者
sms4 post add <user_id> <content>      # 發布貼文
sms4 post reply <post_id> <user_id> <content>  # 回覆貼文
sms4 post list                         # 列出所有貼文
sms4 post get <id>                     # 檢視貼文（含回覆）
sms4 post timeline <user_id>           # 顯示時間軸（自己和追蹤的人）
sms4 post delete <id>                  # 刪除貼文
sms4 follow add <follower> <followee>  # 追蹤
sms4 follow remove <follower> <followee> # 取消追蹤
sms4 follow followers <user_id>        # 粉絲列表
sms4 follow following <user_id>        # 追蹤中列表
sms4 like add <user_id> <post_id>      # 按讚
sms4 like remove <user_id> <post_id>   # 取消讚
```

## 資料表

- **users** — 使用者帳號（username, display_name, bio）
- **posts** — 貼文（content, parent_id 支援回覆、likes_count, replies_count）
- **follows** — 追蹤關係（follower_id, followee_id, UNIQUE）
- **likes** — 按讚記錄（user_id, post_id, UNIQUE）

## 寫碼慣例

- 所有 CLI 輸出、註解為繁體中文
- 錯誤處理使用 `anyhow::Result`
- `#![allow(dead_code, unused)]` 存在於 `src/main.rs` crate root
- `colored` crate 原生支援 `NO_COLOR` 環境變數
