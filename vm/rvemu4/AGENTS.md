# rvemu4 — RISC-V 模擬器（Rust）

RV64GC 指令集模擬器，可執行 xv6/xv6-rust-octopos 核心。

## 建置與執行

```
cargo build                     # 建置（唯一依賴：libc）
cargo test                      # 131 單元測試
cargo run --release -- -kernel xv7-kernel.bin -drive disk.img -smp 1   # 執行 xv7
./run.sh                        # 使用外部路徑的 xv6 kernel/fs.img
./run_xv6rust.sh                # 使用 xv6-rust-octopos kernel/fs.img
```

Edition 2024。僅 libc 一個外部依賴（終端 raw mode、select()）。無 `serde`、無 `clap`、無 `anyhow`。

## 檔案結構

| 檔案 | 說明 |
|------|------|
| `src/cpu.rs` | Hart、CSR、MMU (Sv39)、指令解碼/執行、中斷/例外、**131 個單元測試** |
| `src/memory.rs` | Bus、UART、PLIC、CLINT、VIRTIO (virtio-blk)、RTC、LR/SC 保留 |
| `src/elf.rs` | ELF32/ELF64 loader |
| `src/main.rs` | 主迴圈 + stdin 輪詢 (raw mode)、終端機恢復（atexit + SIGINT + Drop guard） |
| `_doc/` | 版本變更記錄 v0.1–v1.2 |

## RVC 解碼注意事項（CI/CSS 格式陷阱）

CI 格式（C.LWSP、C.LDSP）與 CSS 格式（C.SWSP、C.SDSP）的 6 位元立即值 `uimm` **不共用同一條位元對應公式**。取決於 funct3：
- `f3=2`（C.LWSP）、`f3=6`（C.SWSP）：shift=2，使用 word 位元對應
- `f3=3`（C.LDSP）、`f3=7`（C.SDSP）：shift=3，使用 double 位元對應

相同 offsets 下，word 與 double 指令在同一位元位置的 uimm 貢獻值不同（例如 bit5 在 C.LWSP 中貢獻 2，在 C.LDSP 中貢獻 1）。修改解碼器時必須以 `riscv64-unknown-elf-objdump` 產生的編碼為參考驗證。

## 測試參考編碼產生

```
riscv64-unknown-elf-as -march=rv64gc -o /tmp/t.o /tmp/t.s
riscv64-unknown-elf-objdump -d /tmp/t.o
```

## 中斷架構

- **check_interrupts** (cpu.rs): 每個 step 前呼叫，更新 MIP、觸發 trap
- **PLIC** (memory.rs): 支援 32 IRQ source，8 hart。claim 清除 pending；**completion write 未實作**（不影響功能）
- **UART IRQ 10** (main.rs): 每 4096 steps 輪詢。THR 寫入設 `pending_tx_irq`，`set_pending(10)` 後立刻清除
- **VIRTIO IRQ 1** (memory.rs): 僅在 `process_queue` 實際處理描述子時觸發

## 關鍵除錯技巧

- **DIVERGE abort** (cpu.rs): pc 超出 `[0x80000000, 0x88000000]` 時中止並 dump 全部暫存器狀態；注意 trampoline 頁面 (`0x3FFFFFF000`) 為合法範圍
- **STATE dump** (main.rs): 每 50M steps 印出 hart 0 完整狀態（pc, sp, sepc, scause, priv, mip, mstatus, satp）
- 系統 stuck 時檢查 PLIC pending 是否有 IRQ 10 或 1 被不斷設定
- **Ctrl+A → x** 退出模擬器（終端自動恢復）
- 終端 raw mode 在以下情況自動恢復：Ctrl-A x（立即）、panic（Drop guard）、`process::exit`（atexit）、Ctrl-C（SIGINT handler）

## 已知限制

- PLIC completion write 被忽略
- PMP 未強制執行
- AMO ordering bits 忽略
- Sv39 page walk 未經壓力測試
- 無浮點 D extension
- 僅支援 8 hart 上限
- poweroff ECALL 未實作（核心 `poweroff` 指令會 panic）

## 版本記錄

`_doc/v1.2.md` — 修正 C.ADDI4SPN/C.LWSP/C.SWSP 解碼器、終端機自動恢復
`_doc/v1.1.md` — 獨立執行 ELF 程式規劃
`_doc/v1.0.md` — VIRTIO 實體位址修正、DIVERGE 放寬、xv6 開機至 Shell
`_doc/v0.10.md` — 中斷競爭條件修正（wfi、UART/VIRTIO IRQ 風暴）
