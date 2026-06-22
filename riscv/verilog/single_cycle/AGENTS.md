# single_cycle — RV64IM + Zicsr + C 單週期處理器

Verilog 實作的 RISC-V 單週期 CPU（每 cycle 執行一條指令），支援 RV64IM + Zicsr + Zifencei + C extension。搭配 Verilator 或 Icarus Verilog 模擬。

## 架構概覽

```
rv64i_cpu.v — CPU 本體（單一模組，含 imem + dmem + CSR + CLINT）
tb_rv64i.v  — testbench（UART $write、ecall syscall handler、PASS/FAIL 判斷）
```

### CPU 規格

| 項目 | 內容 |
|---|---|
| ISA | RV64IM + Zicsr + Zifencei + C (RVC) + A (lr.w/sc.w) |
| 記憶體 | 統一 imem/dmem：16K words × 32-bit = 64KB，`$readmemh("program.hex")` |
| 基底位址 | 0x80000000（參數化 `BASE_ADDR`） |
| MMIO | UART 0x10000000–0x1000000F、CLINT 0x02000000–0x0200FFFF |
| CSR | mstatus, mie, mtvec, mscratch, mepc, mcause, mcycle, minstret |
| 中斷 | 僅 timer IRQ（MTIE bit 7），mret 返回 |
| 暫存器檔 | 32 × 64-bit，x0 硬接 0 |

### testbench（tb_rv64i.v）

- `$write("%c", ...)` 為 UART 輸出
- `dbg_ecall` 觸發 syscall handler：exit(0)、putchar、puts
- 預設 timeout 100000 cycles（約 1ms @ 10ns clock）
- `uart_active` 判斷有無 UART 輸出；無輸出則 FAIL
- FIRST 200 cycle 會 trace PC/sp/a0/a1/a2/a7

## 檔案地圖

| 檔案 | 說明 |
|---|---|
| `rv64i_cpu.v` | CPU 主體（854 lines） |
| `tb_rv64i.v` | 測試平台 |
| `link.ld` | 一般程式 linker script（起址 0x00000000） |
| `link_os.ld` | OS linker script（起址 0x80000000） |
| `link_run.sh` | .o → .elf → .bin → .hex 一鍵轉換 |
| `bin2hex.py` | flat binary 轉 Verilog hex 格式 |
| `stubs.s` | Rust panic stub（跳至 `rust_begin_unwind`） |
| `program.hex` | 當前載入的程式（gitignore） |
| `obj_dir/` | Verilator 編譯產物 |
| `_doc/` | 版本設計文件（v0.1–v0.4） |
| `_version/` | 各版本原始碼備份（v0.1–v0.3） |

## 測試腳本

| 命令 | 用途 |
|---|---|
| `./test.sh` | Icarus：編譯 + 執行預設 program.hex |
| `./test_all.sh` | 完整測試套件（Phase 1–3 + rv4 範例） |
| `./test_rv4.sh` | 跑 rv4 範例（hello/sum/fact/fib） |
| `./test_mini_riscv_os.sh` | 建置 + 執行 mini-riscv-os（Verilator） |
| `./link_run.sh <file.o>` | 快速轉換 .o → program.hex |

### test_all.sh 測試階段

1. **Phase 1（UART）** — `test_uart.S`：寫 "Hello\n" 到 UART
2. **Phase 2（MRET）** — `test_mret.S`：csrw mepc → mret → 跳指定位址
3. **Phase 3（Timer）** — `test_timer.S`：CLINT timer IRQ → handler → mret
4. **rv4 examples** — hello / sum / fact / fib（透過 ecall puts 輸出）

### mini-riscv-os 建置流程

```
Step 1: cargo build --release --target riscv32imac-unknown-none-elf  (mini-riscv-os)
Step 2: gcc -c start.s sys.s + link with libmini_riscv_os.a → os.elf
Step 3: objcopy → bin → hex（pad to 16384 words）
Step 4: verilator --binary rv64i_cpu.v tb_rv64i.v
Step 5: run simulation
```

## 版本歷史

| 版本 | 新增功能 |
|---|---|
| v0.1 | RV64I 基礎指令集 |
| v0.2 | M-extension + Zicsr + Zifencei |
| v0.3 | RVC + ECALL + ELF loader |
| v0.4 | 記憶體映射 0x80000000、mret、mscratch/mie、A-ext (lr.w/sc.w)、CLINT timer IRQ、UART MMIO、wfi (NOP)、中斷管線 |

## 已知問題 / 注意事項

- **單週期**：無 pipeline，無 forwarding，每條指令固定 1 cycle
- **中斷時機**：posedge 同時處理指令寫回 + 中斷檢查；timer IRQ 在 mstatus.MIE=1 且 mtime ≥ mtimecmp 時觸發，`mepc = pc + pc_inc`
- **debug `$write`** 在第 625–629 行有一組 probe timer_handler 指令的 DEBUG 輸出，可視需要註解
- **A-ext 簡化**：lr.w/sc.w 在單核永遠成功（sc.w 無需 reservation）
- **wfi** 實作為 NOP
- **多 hart**：僅實作 hart 0，`mhartid` 未實作（mini-riscv-os 只用 hart 0）

## 參考

- `riscv/mini-riscv-os/` — 搭配測試的 OS（Rust staticlib + asm startup）
- `riscv/rv4/` — RISC-V 模擬器（.o 範例用於 test_all.sh）
- `_doc/v0.4-mini-riscv-os.md` — mini-riscv-os 的完整硬體需求與實作討論
