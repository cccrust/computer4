#!/usr/bin/env bash
set -uo pipefail

ERP4=${ERP4:-cargo run --}
export ERP4_DB=${ERP4_DB:-erp4-case2.db}

echo "=== ERP4 Case Study 2: v0.2 新功能測試 ==="
echo "資料庫: $ERP4_DB"
echo ""

$ERP4 init

# 先建立基礎資料供後續測試使用
$ERP4 customer add "Dummy" --email dummy@test.com
$ERP4 product add "DummyP" DUMMY-001 10.00 --stock 100

# ============================================================
# 1. 資料驗證
# ============================================================
echo ">>> 1. 資料驗證 (預期錯誤)"

echo "--- 1a. 產品價格為 0 ---"
$ERP4 product add "Bad" BAD-001 0 --stock 10 || true

echo "--- 1b. 產品庫存負數 ---"
$ERP4 product add "Bad3" BAD-003 100 --stock=-1 || true

echo "--- 1c. Email 格式錯誤 ---"
$ERP4 customer add "Bad Email" --email notanemail || true

echo "--- 1d. 訂單狀態無效 ---"
$ERP4 order create 1
$ERP4 order update-status 1 invalid_status || true

echo "--- 1e. 發票金額 <= 0 ---"
$ERP4 invoice create --customer-id 1 --due-date 2026-08-01 --amount 0 || true

echo ""

# ============================================================
# 2. 搜尋/篩選
# ============================================================
echo ">>> 2. 搜尋與篩選"

# 建立更多測試資料
$ERP4 customer add "台積電" --email contact@tsmc.com
$ERP4 customer add "聯發科" --email hr@mediatek.com
$ERP4 customer add "台達電" --email info@delta.com.tw
$ERP4 product add "CPU A14" CPU-001 15000.00 --stock 500
$ERP4 product add "GPU B8" GPU-001 32000.00 --stock 50
$ERP4 product add "RAM 16GB" RAM-016 1800.00 --stock 20
$ERP4 product add "SSD 1TB" SSD-001 3200.00 --stock 5

echo "--- 2a. 客戶搜尋 ---"
$ERP4 customer list --search "台"
echo ""

echo "--- 2b. 產品搜尋 ---"
$ERP4 product list --search "CPU"
echo ""

echo "--- 2c. 低庫存警示 ---"
$ERP4 product list --low-stock 10
echo ""

echo "--- 2d. 依狀態篩選訂單 ---"
$ERP4 order create 2 --notes "測試單"
$ERP4 order update-status 1 confirmed
$ERP4 order list --status confirmed
echo ""

echo "--- 2e. 依狀態篩選發票 ---"
$ERP4 invoice create --customer-id 1 --due-date 2026-08-15 --amount 50000
$ERP4 invoice create --customer-id 2 --due-date 2026-09-01 --amount 30000
$ERP4 invoice update-status 1 paid
$ERP4 invoice list --status unpaid
echo ""

echo "--- 2f. 依客戶篩選發票 ---"
$ERP4 invoice list --customer-id 1
echo ""

echo "--- 2g. 依客戶篩選訂單 ---"
$ERP4 order list --customer-id 2
echo ""

# ============================================================
# 3. 庫存自動連動
# ============================================================
echo ">>> 3. 庫存自動連動"

echo "--- 3a. 確認訂單前庫存 ---"
$ERP4 product get 1 | grep Stock
$ERP4 product get 2 | grep Stock

echo "--- 3b. 建立訂單並確認 (應扣庫) ---"
$ERP4 order create 3 --notes "庫存測試"
$ERP4 order add-item 3 1 10
$ERP4 order add-item 3 2 2

echo "確認前庫存:"
$ERP4 product get 1 | grep Stock
$ERP4 product get 2 | grep Stock

$ERP4 order update-status 3 confirmed
echo "確認後庫存 (DummyP 應剩 90, CPU 應剩 498):"
$ERP4 product get 1 | grep Stock
$ERP4 product get 2 | grep Stock

echo "--- 3c. 取消訂單 (應還原庫存) ---"
$ERP4 order update-status 3 cancelled
echo "取消後庫存 (應回到 500, 50):"
$ERP4 product get 1 | grep Stock
$ERP4 product get 2 | grep Stock

echo "--- 3d. 採購單收貨 (應入庫) ---"
$ERP4 supplier add "NVIDIA" --contact-person "Amy"
$ERP4 purchase-order create 1 --notes "GPU 補貨"
$ERP4 purchase-order add-item 1 3 100 --unit-price 28000.00
echo "收貨前 GPU 庫存 (原 50):"
$ERP4 product get 3 | grep Stock
$ERP4 purchase-order update-status 1 received
echo "收貨後 GPU 庫存 (應增為 150):"
$ERP4 product get 3 | grep Stock

echo "--- 3e. 庫存不足拒絕 (RAM 只有 20, 訂 100) ---"
$ERP4 order create 3
$ERP4 order add-item 4 4 100
$ERP4 order update-status 4 confirmed || true
echo ""

# ============================================================
# 4. 自動編號
# ============================================================
echo ">>> 4. 自動編號"
$ERP4 invoice create --customer-id 2 --due-date 2026-10-01 --amount 9999
$ERP4 invoice create --customer-id 3 --due-date 2026-10-01 --amount 8888
$ERP4 invoice list --status unpaid
echo "應顯示 INV-2026-XXXX 格式的自動編號"
echo ""

# ============================================================
# 5. 報表
# ============================================================
echo ">>> 5. 報表"

echo "--- 5a. 銷售報表 ---"
$ERP4 order create 3
$ERP4 order add-item 5 1 5
$ERP4 order add-item 5 2 1
$ERP4 order update-status 5 confirmed
$ERP4 report sales
echo ""

echo "--- 5b. 庫存報表 ---"
$ERP4 report inventory
echo ""

echo "--- 5c. 應收帳款報表 ---"
$ERP4 report aging
echo ""

# ============================================================
# 6. 最終確認
# ============================================================
echo "=== 最終狀態 ==="
echo "--- 客戶 ---"
$ERP4 customer list
echo "--- 產品 ---"
$ERP4 product list
echo "--- 供應商 ---"
$ERP4 supplier list
echo "--- 訂單 ---"
$ERP4 order list
echo "--- 採購單 ---"
$ERP4 purchase-order list
echo "--- 發票 ---"
$ERP4 invoice list

echo ""
echo "=== Case Study 2 完成 ==="
