# rvemu4 — RISC-V 模擬器（Rust）

RV64GC 指令集模擬器，可執行 xv6/xv7 核心。

## 建置與執行

```
cargo build                     # 建置
cargo test                      # 34 單元測試
cargo run -- -kernel xv7-kernel.bin -drive disk.img -smp 1   # 執行 xv7
```

## 檔案結構

| 檔案 | 說明 |
|------|------|
| `src/cpu.rs` | Hart 結構、CSR、MMU、指令解碼/執行（step）、中斷/例外處理 |
| `src/memory.rs` | Bus、UART、PLIC、CLINT、VIRTIO（virtio-blk）、RTC、LR/SC 保留 |
| `src/elf.rs` | ELF loader |
| `src/main.rs` | 主迴圈：每 hart 執行 check_interrupts + step，輪詢 UART/stdin |
| `_doc/` | 各版本變更記錄 (v0.1–v0.9) |

## 中斷架構

- **check_interrupts** (cpu.rs:161): 每個 step 前呼叫，更新 MIP、觸發 trap
- **PLIC** (memory.rs:71–125): 支援 32 IRQ source，8 hart。claim 清除 pending，completion write 未實作
- **UART IRQ 10** (main.rs:69–75): 輪詢機制（每 4096 steps）。THR 寫入設 `pending_tx_irq`，`set_pending(10)` 後立刻清除
- **VIRTIO IRQ 1** (memory.rs:288): 僅在 `process_queue` 實際處理描述子時觸發

## 關鍵除錯技巧

- **DIVERGE crash** (cpu.rs:190–193): pc 超出 `[0x80000000, 0x88000000]` 時中止並 dump 全部狀態
- **SCHED_TRACE** (main.rs:57–66): 每 1M steps 印出排程器相關暫存器與 `proc[0].state`
- **PROGRESS** (main.rs:67): 每 1M steps 印出 `steps={}K pc={:#x}`
- 系統 stuck 時檢查 PLIC pending 是否有 IRQ 10 或 1 被不斷設定

## 已知問題

- PLIC completion write 被忽略（不影響功能）
- PMP 未強制執行
- AMO ordering bits 忽略
- Sv39 page walk 未經壓力測試
- 無浮點 D extension

## 版本記錄

`_doc/v0.9.md` — 中斷控制器修復（wfi、UART/VIRTIO IRQ 風暴）
`_doc/v0.8.md` — RVC decoder 終極修正（C.SDSP/C.LDSP brute-force 驗證）
`_doc/v0.7.md` — RVC decoder 修正與 34 測試
`_doc/v0.6.md` 之前 — 初始版本
