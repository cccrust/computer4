# xv8-rust-posix 版本規劃 (v1.1 ~ v1.9)

> v1.x = 非網路 POSIX 功能。網路全部歸入 v2.x。
> 目標：讓 xv8 能支援 Rust `std`/`alloc` 的底層函數，使 `os/posix/tools` 的工具不需複製即可直接在 xv8 上編譯執行。

## v1.1 — alloc / libposix Dual Target
**主題**：記憶體模型統一 + `libposix` 兩路支援

**狀態**：✅ Phase 1 完成 | ✅ Phase 2 完成 | ✅ Phase 3 完成

### 架構確認
- xv8 是 static binary（kernel + user 靜態連結），只有一個 `#[global_allocator]`（`buddy-alloc`）
- `user/src/lib.rs` 已加 `extern crate alloc`，自動使用 kernel 的 allocator
- `sys_sbrk` (syscall 12) 是 user 向 buddy-alloc 要記憶體的介面

### 運作原理

`libposix` 是**跨平臺 abstraction layer**，同一份 source code 自動適配 Mac/Linux 和 xv8：

```
os/posix/tools/src/bin/od.rs
        │
        │  cargo build        cargo build --target riscv64gc-unknown-none-elf
        ▼                      ▼
┌───────────────┐      ┌─────────────────┐
│   Mac/Linux   │      │      xv8        │
│  libposix::*  │      │   libposix::*   │
│   (Unix)      │      │ (target_os=none)│
├───────────────┤      ├─────────────────┤
│ libc::open    │      │  ecall open     │
│ std::env::args│      │ Args::from_stack│
│ std::process  │      │  ecall exit     │
│ std::io::Read │      │  ecall read     │
└───────────────┘      └─────────────────┘
```

`libposix/src/io.rs` 用 `#[cfg(unix)]` / `#[cfg(target_os = "none")]` cfg gates 提供相同 API 的不同實作。工具程式只寫 `use libposix::{File, args, exit}`，編譯時平臺決定用哪個實作。

好處：**同一份 source code** 適用所有平臺，無需複製。

### Phase 1
- [x] `libposix::args()` 在 xv8 上從 `Args` → `Vec<String>`
- [x] `libposix::exit()` 在 xv8 上正確終止（ecall a7=10）

### Phase 2
- [x] `libposix/src/io.rs` 兩路實作完成（Unix + xv8）
- [x] `od.rs` 改用 `libposix`（Mac 上 build 成功）
- [ ] `cat.rs`、`head.rs` 改造

### Phase 3
- [x] `xv8-rust-posix/user` 引用 `libposix` 作為 dependency

### 驗證
```sh
# Mac 上測試 libposix
cd os/posix/libposix && cargo build
cd os/posix/tools && cargo build --bin od  # 成功

# xv8 上測試
cd os/xv8-rust-posix && ./test.sh  # 全部 15 個測試通過
```

**詳細內容**：見 `_doc/v1.1.md`

---

## v1.2 — 工具改造 ✅
**主題**：將 `os/posix/tools/src/bin/` 的工具從 `std` 改為 `libposix`

### 工具清單
| 工具 | 狀態 | 備註 |
|------|------|------|
| `od` | ✅ 完成 | 用 `libposix::File`/`args`/`exit` |
| `cat` | ✅ 完成 | |
| `head` | ✅ 完成 | |
| `hexdump` | 待改造 | |
| `bash` | 待改造 | |

**前提**：v1.1 Phase 2 完成後才能開始。

---

## v1.3 — 完整 I/O 改造
**主題**：所有 `os/posix/tools` 工具完成 `libposix` 改造

### 工具清單（全部 134+ binaries）
- 基本工具：`cat`, `ls`, `cd`, `echo`, `pwd`, `kill`, `sleep`, `uptime`, `init`, `poweroff`
- 檔案操作：`cp`, `mv`, `rm`, `mkdir`, `rmdir`, `ln`, `touch`, `chmod`, `chown`, `stat`, `file`
- 文字處理：`head`, `tail`, `sort`, `uniq`, `cut`, `tr`, `wc`, `grep`, `sed`, `awk`, `od`, `hexdump`
- 路徑處理：`dirname`, `basename`, `readlink`, `symlink`, `link`, `unlink`
- 系統資訊：`id`, `whoami`, `uname`, `primes`
- 格式轉換：`dd`, `install`
- 其他：`test`, `expr`, `printf`, `tee`, `nice`, `nohup`, `find`, `env`, `printenv`

**前提**：v1.2 完成後逐一改造。

---

## v1.4 — tmpfs / devpts
**主題**：虛擬檔案系統

| 功能 | 說明 |
|------|------|
| `tmpfs` | 基於記憶體的檔案系統，`open`/`read`/`write`/`unlink` |
| `devpts` | PTY 虛擬終端機，`/dev/pts/*` |

---

## v1.5 — POSIX 訊號框架
**主題**：完整 POSIX 訊號支援

| 功能 | 說明 |
|------|------|
| `sigaction` | 核心 syscall |
| `sigprocmask` | 阻塞/解除阻塞訊號 |
| `sigpending` | 查詢待處理訊號 |
| `sigsuspend` | 原子性更換訊號遮罩並睡眠 |
| SIGCHLD 處理 | `wait` 相關 |

---

## v1.6 — mmap 增強 / POSIX 共享記憶體
**主題**：記憶體映射

| 功能 | 說明 |
|------|------|
| `mmap(MAP_SHARED)` | 共享記憶體對應 |
| `mmap(MAP_PRIVATE)` | ✅ 已有（v0.5）|
| `munmap` | 解除映射 |
| `mprotect` | 改變保護模式 |

---

## v1.7 — lseek / ftruncate / fstat
**主題**：完整檔案操作

| 功能 | 說明 |
|------|------|
| `lseek` | syscall 28，移動檔案指標 |
| `ftruncate` | 截斷檔案 |
| `fstat` / `stat` / `lstat` | 取得檔案狀態 |
| `access` | 檢查檔案存取權限 |

---

## v1.8 — uid/gid + 權限
**主題**：安全模型

| 功能 | 說明 |
|------|------|
| `getuid` / `getgid` | 回報 0（single user）|
| `getpid` / `getppid` | ✅ 已有 |
| `chmod` / `chown` | 改變權限（`fchmod`/`fchown`）|
| `umask` | 設定檔案建立遮罩 |

---

## v1.9 — 終端機 termios
**主題**：完整終端機控制

| 功能 | 說明 |
|------|------|
| `tcgetattr` / `tcsetattr` | 取得/設定終端機屬性 |
| `tcsendbreak` / `tcdrain` | 傳送 break、排空 |
| `tcflush` / `tcflow` | 清除/流動控制 |
| `isatty` | 檢查是否為終端機 |

---

## v2.x 主題（網路）
- TCP/UDP socket
- `socket`/`bind`/`connect`/`listen`/`accept`
- POSIX `poll`/`select`

**詳細規劃**：見 `_doc/version2.x.md`