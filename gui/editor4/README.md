# editor4 – Rust 多分頁文字編輯器 & 內建終端

`editor4` 是一個基於 **eframe / egui** 的輕量級 GUI 文字編輯器，支援多分頁、檔案操作、搜尋以及內建的簡易終端（Shell）功能。它位於 `computer4/gui/editor4/`，是一個獨立的 Rust crate。

## 功能特色
- 多分頁（普通編輯頁與終端頁）
- 檔案開啟 / 儲存（使用 [`rfd`](https://crates.io/crates/rfd)）
- 即時搜尋（Ctrl+F）
- 內建終端：`Ctrl+T` 新增終端頁，輸入指令後按 Enter 執行，結果即寫回編輯區
- 基本文字編輯操作（插入、刪除、換行、Backspace）

## 快捷鍵總覽
| 快捷鍵 | 功能 |
|--------|------|
| **Ctrl+N** | 新增普通編輯分頁 |
| **Ctrl+T** | 新增終端分頁（提示 `>>> `） |
| **Ctrl+O** | 開啟檔案 |
| **Ctrl+S** | 儲存當前檔案 |
| **Ctrl+Shift+S** | 另存新檔 |
| **Ctrl+W** | 關閉當前分頁 |
| **Ctrl+F** | 開啟搜尋面板 |
| **Esc** | 關閉搜尋面板 |
| **←/→/↑/↓** | 移動光標 |
| **Home/End** | 行首/行尾 |
| **PageUp/PageDown** | 捲動一頁 |
| **Enter** | 普通頁換行 / 終端頁執行指令 |
| **Backspace / Delete** | 刪除或合併行 |

## 終端分頁使用範例
1. 按 **Ctrl+T** 新增終端分頁，會顯示 `>>> ` 提示。 
2. 輸入指令（不需要手動輸入 `>>> `），例如 `ls -l`，然後 **Enter**。 
3. 程式會以 `sh -c "<指令>"` 執行，將 stdout & stderr 的每行寫回緩衝區，最後再插入新 `>>> ` 提示，光標自動定位。

## 建置與測試
```bash
# 取得程式碼（已在 monorepo 中）
cd /Users/Shared/ccc/project/computer4/gui/editor4

# 編譯 (Debug)
cargo build

# 執行單元測試（兩個測試，均通過）
cargo test -- --nocapture
```

## 執行
```bash
cargo run
```
會彈出 1200×800 視窗，標題為 **editor4**。

## 專案結構
```
editor4/
├─ Cargo.toml
├─ test.sh      # 執行 `cargo test -- --nocapture`
└─ src/
   ├─ main.rs   # 入口，呼叫 eframe::run_native
   ├─ editor.rs # UI、事件、渲染、快捷鍵、分頁管理
   └─ buffer.rs # 文字緩衝區模型（支援終端模式）
```

## 常見問題 (FAQ)
- **指令執行失敗顯示 `Failed to run command`**：確認指令在系統 `PATH` 中，或使用絕對路徑。
- **為什麼在普通分頁按 Enter 不執行指令？** 終端功能僅在 `Buffer::is_terminal()` 為 true（即使用 `new_terminal()` 建立的緩衝區）時啟用。
- **搜尋高亮顏色不易辨識**：高亮顏色在 `editor.rs` 中可自行調整 `egui::Color32::from_rgba_premultiplied(255,200,0,40)`。

## 授權
本專案遵循 MIT License。