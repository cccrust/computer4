# rvemu4 — RISC-V 模擬器（Rust）

RV64GC 指令集模擬器，可執行 xv6/xv7 核心。

## 建置與執行

```
cargo build                     # 建置（唯一依賴：libc）
cargo test                      # 34 單元測試
cargo run --release -- -kernel xv7-kernel.bin -drive disk.img -smp 1   # 執行 xv7
./run.sh                        # run.sh 使用外部路徑的 xv6 kernel/fs.img
```

Edition 2024。僅 libc 一個外部依賴（終端 raw mode、select()）。無 `serde`、無 `clap`、無 `anyhow`。

## 檔案結構

| 檔案 | 說明 |
|------|------|
| `src/cpu.rs` | Hart、CSR、MMU (Sv39)、指令解碼/執行、中斷/例外、**34 個單元測試** |
| `src/memory.rs` | Bus、UART、PLIC、CLINT、VIRTIO (virtio-blk)、RTC、LR/SC 保留 |
| `src/elf.rs` | ELF32/ELF64 loader |
| `src/main.rs` | 主迴圈 + stdin 輪詢 (raw mode) |
| `_doc/` | 版本變更記錄 v0.1–v0.9 |

## 中斷架構

- **check_interrupts** (cpu.rs): 每個 step 前呼叫，更新 MIP、觸發 trap
- **PLIC** (memory.rs): 支援 32 IRQ source，8 hart。claim 清除 pending；**completion write 未實作**（不影響功能）
- **UART IRQ 10** (main.rs): 每 4096 steps 輪詢。THR 寫入設 `pending_tx_irq`，`set_pending(10)` 後立刻清除
- **VIRTIO IRQ 1** (memory.rs): 僅在 `process_queue` 實際處理描述子時觸發

## 關鍵除錯技巧

- **DIVERGE abort** (cpu.rs): pc 超出 `[0x80000000, 0x88000000]` 時中止並 dump 全部暫存器狀態
- **STATE dump** (main.rs): 每 50M steps 印出 hart 0 完整狀態（pc, sp, sepc, scause, priv, mip, mstatus, satp）
- 系統 stuck 時檢查 PLIC pending 是否有 IRQ 10 或 1 被不斷設定
- **Ctrl+A → x** 退出模擬器（終端 raw mode 下清 stdin 中斷觸發用）

## 已知限制

- PLIC completion write 被忽略
- PMP 未強制執行
- AMO ordering bits 忽略
- Sv39 page walk 未經壓力測試
- 無浮點 D extension
- 僅支援 8 hart 上限

## 版本記錄

`_doc/v0.9.md` — 中斷控制器修復（wfi、UART/VIRTIO IRQ 風暴）
`_doc/v0.8.md` — RVC decoder 終極修正（C.SDSP/C.LDSP brute-force 驗證）
`_doc/v0.7.md` — RVC decoder 修正與 34 測試
`_doc/v0.6.md` 之前 — 初始版本
