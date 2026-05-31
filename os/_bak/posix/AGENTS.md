# AGENTS.md — v1.1 交接

## 現況

v1.1 Phase 1/2/3 核心實作已完成，但 **未完成 xv8 上 od/cat/head 的實際執行測試**。

## 已完成

### libposix（`os/posix/libposix/`）

完全重寫，支援 Mac (`#[cfg(unix)]`) 和 xv8 (`#[cfg(target_os = "none")`)：

- **`src/io.rs`**：兩路實作
  - Unix: `libc` + `std` 包裝
  - xv8: `ecall` inline asm（read=5, write=16, open=15, close=21, exit=10）
  - `args()`: Unix 用 `std::env::args()`，xv8 用 `Args::from_stack()` → `Vec<String>`
- **`src/lib.rs`**：匯出 `File`, `Read`, `Write`, `stdin/stdout/stderr`, `print/println/exit/args`
- **`Cargo.toml`**: `[target.'cfg(unix)'.dependencies] libc = "0.2"`

### tools 改造（`os/posix/tools/src/bin/`）

| 工具 | 狀態 |
|------|------|
| `od.rs` | ✅ 改用 `libposix::{File, Read, args, exit, println}`，Mac build 成功 |
| `cat.rs` | ✅ 改用 `libposix::{File, Read, Write, args, exit, stdin, stdout}`，Mac build 成功 |
| `head.rs` | ✅ 改用 `libposix::{File, Read, Write, args, exit, stdin, stdout}`，Mac build 成功 |

### xv8 整合

- **`xv8-rust-posix/user/Cargo.toml`**：已加 `libposix = { path = "../../posix/libposix" }`
- **`xv8-rust-posix/user/src/lib.rs`**：已加 `#![feature(alloc)]` + `extern crate alloc`

### 文件

- **`_doc/xv8-posix.md`**：完整架構說明，含「運作原理」段落（跨平臺圖解）
- **`_doc/v1.1.md`**：Phase 1/2/3 實作規劃，含狀態更新
- **`_doc/version1.x.md`**：v1.1 標記為 ✅ Phase 完成
- **`AGENTS.md`**：已加 Memory Model + libposix Dual Target 段落

### 測試腳本

- **`os/posix/test.sh`**：Mac 上測試 cat/head/od + `cargo build` + `cargo test`

## 未完成

### 最關鍵：xv8 上執行測試

**問題**：tools 的 `Cargo.toml` 頂層有 `libc = "0.2"` dependency，會導致 xv8 target（`riscv64gc-unknown-none-elf`）編譯失敗。需要確認這個問題是否已修復。

`libposix/Cargo.toml` 已經只有 `libc` for `target.'cfg(unix)'.dependencies`，但 `tools/Cargo.toml` 頂層仍有 `libc = "0.2"`。**需要移除**後重新測試：

```bash
# 移除 tools/Cargo.toml 頂層的 libc
sed -i '' '/^libc = "0.2"$/d' os/posix/tools/Cargo.toml
# 確認結果
sed -n '6,12p' os/posix/tools/Cargo.toml
```

然後編譯 xv8 target：

```bash
cd os/posix/tools
cargo build --target riscv64gc-unknown-none-elf --bin od 2>&1 | head -20
cargo build --target riscv64gc-unknown-none-elf --bin cat 2>&1 | head -20
cargo build --target riscv64gc-unknown-none-elf --bin head 2>&1 | head -20
```

### 將 elf 加入 xv8 fs.img

編譯成功後，需要把 elf 複製到 xv8 的 release 目錄，並執行 `./test.sh` 或手動跑 mkfs.sh 把它們加進 fs.img：

```bash
# 複製 elf 到 xv8 release 目錄
cp target/riscv64gc-unknown-none-elf/release/od \
   /path/to/xv8-rust-posix/target/riscv64gc-unknown-none-elf/release/od
# 類似 cat, head

# 重建 fs.img
cd /path/to/xv8-rust-posix
./test.sh
```

### 交付檢查表（v1.1.md）

- [ ] `od.rs` 在 xv8 上編譯成功並正確執行
- [ ] `cat.rs` 在 xv8 上編譯成功並正確執行
- [ ] `head.rs` 在 xv8 上編譯成功並正確執行

## 關鍵檔案位置

| 檔案 | 用途 |
|------|------|
| `os/posix/libposix/src/io.rs` | 兩路 I/O 實作 |
| `os/posix/libposix/src/lib.rs` | 匯出 API |
| `os/posix/tools/src/bin/od.rs` | 已改造的 od |
| `os/posix/tools/src/bin/cat.rs` | 已改造的 cat |
| `os/posix/tools/src/bin/head.rs` | 已改造的 head |
| `os/xv8-rust-posix/user/Cargo.toml` | 已有 libposix dependency |
| `os/xv8-rust-posix/_doc/v1.1.md` | 實作規劃文件 |
| `os/posix/test.sh` | Mac 測試腳本 |

## Syscall Numbers（xv8）

| 編號 | 名稱 | 用於 |
|------|------|------|
| 5 | read | libposix read |
| 10 | exit | libposix exit (noreturn) |
| 12 | sbrk | alloc (已存在) |
| 15 | open | libposix open |
| 16 | write | libposix write |
| 21 | close | libposix close |
| 34 | mmap | alloc (已存在) |