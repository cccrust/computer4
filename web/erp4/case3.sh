#!/usr/bin/env bash
set -uo pipefail

ERP4=${ERP4:-cargo run --}
export ERP4_DB=${ERP4_DB:-erp4-case3.db}

echo "=== ERP4 Case Study 3: v0.3 新功能測試 ==="
echo "資料庫: $ERP4_DB"
echo ""

$ERP4 init

# 建立基礎資料
$ERP4 customer add "科技公司A" --email contact@techA.com
$ERP4 customer add "科技公司B" --email info@techB.com
$ERP4 customer add "科技公司C" --email hello@techC.com
$ERP4 customer add "科技公司D" --email admin@techD.com
$ERP4 customer add "科技公司E" --email support@techE.com

$ERP4 product add "筆電 X1" NB-X1 35000.00 --stock 100
$ERP4 product add "筆電 X2" NB-X2 45000.00 --stock 50
$ERP4 product add "滑鼠 M1" MOUSE-M1 599.00 --stock 200
$ERP4 product add "鍵盤 K1" KB-K1 1299.00 --stock 30
$ERP4 product add "螢幕 27" MON-27 8999.00 --stock 15

$ERP4 supplier add "供應商X" --contact-person "張經理"
$ERP4 supplier add "供應商Y" --contact-person "李副總"

echo ""
echo "======================================"
echo ">>> 1. CSV 匯出測試"
echo "======================================"

echo "--- 1a. 客戶列表 CSV ---"
$ERP4 customer list --format csv
echo ""

echo "--- 1b. 產品列表 CSV ---"
$ERP4 product list --format csv
echo ""

echo "--- 1c. 供應商列表 CSV ---"
$ERP4 supplier list --format csv
echo ""

echo "--- 1d. 銷售報表 CSV ---"
$ERP4 order create 1
$ERP4 order add-item 1 1 2
$ERP4 order update-status 1 confirmed
$ERP4 report sales --format csv
echo ""

echo "--- 1e. 庫存報表 CSV ---"
$ERP4 report inventory --format csv
echo ""

echo "--- 1f. 應收帳款報表 CSV ---"
$ERP4 invoice create --customer-id 1 --due-date 2026-12-31 --amount 70000
$ERP4 report aging --format csv
echo ""

echo "======================================"
echo ">>> 2. 分頁測試"
echo "======================================"

echo "--- 2a. 客戶分頁 (page=1, page-size=2) ---"
$ERP4 customer list --page 1 --page-size 2
echo ""
echo "--- 2b. 客戶分頁 (page=2, page-size=2) ---"
$ERP4 customer list --page 2 --page-size 2
echo ""
echo "--- 2c. 客戶分頁 (page=3, page-size=2) ---"
$ERP4 customer list --page 3 --page-size 2
echo ""

echo "--- 2d. 產品分頁 (page=1, page-size=3) ---"
$ERP4 product list --page 1 --page-size 3
echo ""
echo "--- 2e. 產品分頁 (page=2, page-size=3) ---"
$ERP4 product list --page 2 --page-size 3
echo ""

echo "======================================"
echo ">>> 3. 錯誤訊息測試"
echo "======================================"

echo "--- 3a. 價格 <= 0 ---"
$ERP4 product add "Bad" BAD-001 0 --stock 10 || true
echo ""

echo "--- 3b. 庫存負數 ---"
$ERP4 product add "Bad2" BAD-002 100 --stock=-1 || true
echo ""

echo "--- 3c. Email 格式錯誤 ---"
$ERP4 customer add "Bad Email" --email noatsign || true
echo ""

echo "--- 3d. 無效訂單狀態 ---"
$ERP4 order update-status 1 invalid_status || true
echo ""

echo "--- 3e. 發票金額 <= 0 ---"
$ERP4 invoice create --customer-id 1 --due-date 2026-12-31 --amount 0 || true
echo ""

echo "======================================"
echo ">>> 4. 輸出格式混合使用"
echo "======================================"

echo "--- 4a. 產品篩選 + CSV ---"
$ERP4 product list --search "筆電" --format csv
echo ""

echo "--- 4b. 產品篩選 + 分頁 ---"
$ERP4 product list --search "筆電" --page 1 --page-size 10
echo ""

echo "--- 4c. 訂單狀態篩選 + CSV ---"
$ERP4 order list --status confirmed --format csv
echo ""

echo "======================================"
echo ">>> 5. 最終確認"
echo "======================================"
echo "--- 客戶總數: 5 ---"
$ERP4 customer list
echo ""
echo "--- 產品總數: 5 ---"
$ERP4 product list
echo ""
echo "--- 供應商 ---"
$ERP4 supplier list
echo ""

echo ""
echo "=== Case Study 3 完成 ==="
