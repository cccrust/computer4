# AGENTS.md

## 專案概要

Rust + SQLite CLI + Web ERP 系統。單一 crate，二進位名稱為 `erp4`。

## 開發指令

```bash
cargo build                          # 除錯版
cargo build --release                # 正式版 (target/release/erp4)
cargo test                           # 執行所有測試
cargo fmt                            # 格式化
cargo clippy                         # 靜態分析
cargo check                          # 型別檢查
cd web && npm ci && npm run build    # 建置前端
cd web && npm run dev                # 前端 dev server (port 5173)
ERP4_DB=erp4-dev.db cargo run -- web --dev  # API dev server (port 8080)
```

無 CI、無 pre-commit hook、無 codegen、無 migration 系統。

## 環境變數

- `ERP4_DB` — SQLite 資料庫路徑（預設 `erp4.db`）
- `ERP4_JWT_SECRET` — JWT 簽章密鑰（預設開發用固定值）

## 測試

- 36 個單元測試，分布於 `src/model/*.rs`、`src/cli/import.rs` 與 `src/cli/fmt.rs`
- 使用 In-memory SQLite，Schema 載入來自 `src/db.sql`（`include_str!`）
- 整合測試腳本：`case1.sh`（基礎流程）、`case2.sh`（v0.2 新功能）、`case3.sh`（v0.3 新功能）、`case4.sh`（v0.5 新功能），需手動執行
- 腳本使用 `ERP4=${ERP4:-cargo run --}`，可覆寫為已編譯二進位

## 架構重點

- **入口:** `src/main.rs` → 讀取 `ERP4_DB`、開啟 SQLite、分派 CLI 或 Web 指令（`#[tokio::main]`）
- **CLI 層:** `src/cli/` (`clap` derive 模式)
- **模型層:** `src/model/` — CRUD + 業務邏輯 + 測試
- **Web 層:** `src/web/` (Axum 0.8) — JWT 認證 + RESTful API + React SPA
- **前端:** `web/` — React 19 + TypeScript + Vite + Tailwind CSS v4
- **資料庫初始化:** `src/db.rs` (`CREATE TABLE IF NOT EXISTS`)，11 張表

## Web 模式

```bash
erp4 web                   # 正式模式 (port 8080, 127.0.0.1, 嵌入靜態資源)
erp4 web --port 3000       # 自訂埠號
erp4 web --host 0.0.0.0    # 監聽所有介面
erp4 web --dev             # 開發模式 (無靜態資源服務，需另啟 Vite)
```

## 商務邏輯

- 庫存自動連動：`order confirmed` 扣庫、`order cancelled` 還原；`purchase-order received` 入庫、`purchase-order cancelled` 還原
- 狀態白名單：
  - Order: `pending`, `confirmed`, `shipped`, `delivered`, `cancelled`
  - PurchaseOrder: `pending`, `approved`, `received`, `cancelled`
  - Invoice: `unpaid`, `paid`, `overdue`, `cancelled`
- 發票自動編號格式：`INV-YYYY-NNNN`（`sequences` 表）

## 寫碼慣例

- **所有 CLI 輸出、註解為繁體中文**
- 錯誤處理使用 `anyhow::Result`
- `#![allow(dead_code, unused)]` 存在於 `src/main.rs` crate root
- 共用格式化工具位於 `src/cli/fmt.rs`（千分位、CSV、著色）
- `colored` crate 原生支援 `NO_COLOR` 環境變數
