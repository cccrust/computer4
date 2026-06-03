# computer4

自製電腦系統 monorepo。每個子目錄都是獨立的 Rust crate（根目錄無 Cargo workspace）。僅能個別建置/測試。

## How to Investigate

Read the highest-value sources first:  
- `README*`, root manifests, workspace config, lockfiles  
- build, test, lint, formatter, typecheck, and codegen config  
- CI workflows and pre-commit / task runner config  
- existing instruction files (`AGENTS.md`, `CLAUDE.md`, `.cursor/rules/`, `.cursorrules`, `.github/copilot-instructions.md`)  
- repo-local OpenCode config such as `opencode.json`

If architecture is still unclear after reading config and docs, inspect a small number of representative code files to find the real entrypoints, package boundaries, and execution flow. Prefer reading the files that explain how the system is wired together over random leaf files.

## What to extract

Look for the highest-signal facts for an agent working in this repo:
- exact developer commands, especially non-obvious ones
- how to run a single test, a single package, or a focused verification step
- required command order when it matters, such as `lint -> typecheck -> test`
- monorepo or multi-package boundaries, ownership of major directories, and the real app/library entrypoints
- framework or toolchain quirks: generated code, migrations, codegen, build artifacts, special env loading, dev servers, infra deploy flow
- repo-specific style or workflow conventions that differ from defaults
- testing quirks: fixtures, integration test prerequisites, snapshot workflows, required services, flaky or expensive suites
- important constraints from existing instruction files worth preserving

## Questions

Ask the user only if the repo cannot answer something important. Use the `question` tool for one short batch at most.

## Monorepo 地圖

| 目錄 | Crate | Edition | 說明 |
|---|---|---|---|
| **compiler/** | | | |
| `compiler/lli4/` | lli4 | 2021 | LLVM IR 直譯器 — `lli4::interpret()` |
| `compiler/rustc4/` | rustc4 | 2021 | Rust → LLVM IR 編譯器 — `rustc4::compile()` |
| `compiler/rv4/` | rv4 | 2021 | RISC-V 模擬器 (RV32I/RV64I/RV64GC) — `rv4::run_elf()` |
| `compiler/objdump/` | objdump_lib | 2021 | ELF 分析器 (clap CLI) |
| `compiler/py4/` | (standalone) | — | Python 直譯器 — `py4.rs` + `lib4.rs` |
| **database/** | | | |
| `database/db6/` | db6 | 2021 | 旗艦 KV+SQL+FTS+Msgq。REPL / server / gRPC。另有 [AGENTS.md](database/db6/AGENTS.md) |
| `database/sql4/` | sql4 | 2024 | SQLite-like，支援 CJK FTS |
| `database/btree/` | btree | 2024 | BTree 引擎（有 `test.sh`） |
| `database/lsm/` | lsm | 2021 | LSM-Tree 引擎（有 `test.sh`） |
| `database/fts/` | fts | 2021 | 全文檢索 |
| `database/swisstable/` | swisstable | 2021 | Swiss Table（有 examples/） |
| `database/patricia-trie/` | patricia-trie | 2024 | Patricia trie |
| `database/redblacktree/` | redblacktree | 2024 | LLRB 樹。另有 [AGENTS.md](database/redblacktree/AGENTS.md) |
| `database/inodefs/` | inodefs | 2021 | Inode 虛擬檔案系統 |
| **math4/** | math4rs | 2021 | 統計、繪圖、ndarray、代數、微積分、線性代數、幾何。另有 [AGENTS.md](math4/AGENTS.md) |
| **crypto/** | | | |
| `crypto/ssl4/` | ssl4 | 2021 | SSL/TLS (rustls + tokio-rustls) |
| `crypto/keygen/` | keygen | 2021 | RSA/ECDSA 金鑰與憑證 CLI 產生器 |
| **gui/** | | | |
| `gui/win4/` | win4 | 2021 | 視窗管理器 (eframe/egui) |
| `gui/game4/` | game4 | 2021 | 遊戲框架 — WebSocket server + JS 前端 |
| **web/** | | | |
| `web/browser5/` | browser5 | 2021 | 瀏覽器，使用自製 xdom4/js4 — 區域路徑依賴 |
| `web/md4browser/` | md4browser | 2021 | Markdown 瀏覽器 (eframe) |
| `web/xdom4/` | xdom4 | 2021 | XML/DOM 函式庫（CSS 選擇器） |
| `web/js4/` | js4 | 2021 | JavaScript 引擎（tokenizer → AST → interpreter） |
| **media/** | | | |
| `media/jpeg/` | jpeg | 2021 | JPEG 編解碼器 (PPM↔JPEG) |
| `media/mp3/` | mpeg_codec | 2021 | MP3 解碼/編碼器 |
| `media/aplayer4/` | aplayer4 | 2024 | 音訊播放器 (rodio + crossterm TUI) |
| **eda/** | | | |
| `eda/verilog2rust/` | verilog2rust | 2021 | Verilog → Rust (rhdl) 轉換器 + rhdl 硬體描述函式庫 |
| **os/** | | | |
| `os/mini-riscv-os/` | mini-riscv-os | 2021 | 最小 RISC-V OS 核心（`#![no_std]` staticlib，QEMU） |
| `os/xv8-rust-posix/` | xv8 (kernel) + user | 2021 | POSIX 相容 xv7 進化版（nightly，QEMU） |
| `os/posix/tools/` | tools | 2021 | **124+ POSIX 工具**（`sh`、`ls`、`diff`、`grep`、`awk` 等）。134 binary targets，214 tests。另有 [_doc/](os/posix/_doc/) 下多版本文件 |
| **tool/** | | | |
| `tool/vi4/` | vi4 | 2021 | 終端機文字編輯器 (crossterm) |

## 建置與測試

`rustc4` 寫出 `.ir` → `lli4` 直譯 `.ir`

| 範例 | 說明 |
|---|---|
| `cargo build` | 當前 crate（永不 --workspace） |
| `cargo test` | 當前 crate |
| `cargo run` | 有 `main.rs` 者可用 |
| `./test.sh` | 多數 crate 用此腳本（build + test） |
| `./run.sh` | GUI/媒體 crate 經常用此腳本 |
| `./git.sh <msg> <branch>` | git add . && commit -m "$msg-$branch" && push |
| `cd compiler/py4 && rustc py4.rs -o py4 && ./py4` | 無 Cargo 的獨立 `rustc` crate |
| `cd tool/regex4 && rustc regex4.rs -o regex4 && ./regex4` | 無 Cargo 的獨立 `rustc` crate |

## 慣例

- **無根 workspace** — 每個頂層 crate 有自己的 `Cargo.lock` 和 `target/`
- **例外：** `os/xv6-rust-octopus/` 和 `os/xv7-rust-octopus/` 各為 Cargo workspace（核心 + 使用者 + mkfs）
- Edition：多數 = 2021；`sql4`、`btree`、`patricia-trie`、`redblacktree`、`lz4`、`aplayer4` + octopos 核心/使用者/mkfs = 2024
- 原始碼註解使用繁體中文
- 無 CI/CD，無根目錄 `rust-toolchain.toml`（octopos 內部各自鎖定 nightly）
- `rustc` 獨立 crate：`compiler/py4/`（`py4.rs` + `lib4.rs`）、`tool/regex4/`（`regex4.rs`）

## Wiki 參考

`_wiki/` 目錄包含本專案領域知識的詳細說明，涵蓋 RISC-V、LLVM IR、ruHDL、LSM-Tree、全文檢索、Swiss Table、Patricia Trie、LLRB 樹、ELF 格式等主題。

## 各套件專屬指令檔

- [`math4/AGENTS.md`](math4/AGENTS.md) — NaN 處理、多項式升冪順序、R/JS 命名
- [`database/db6/AGENTS.md`](database/db6/AGENTS.md) — 架構、REPL 指令、引擎 trait
- [`database/redblacktree/AGENTS.md`](database/redblacktree/AGENTS.md) — CLI 用法、結構