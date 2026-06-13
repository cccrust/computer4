#!/usr/bin/env bash
set -uo pipefail

ERP4=${ERP4:-cargo run --}
export ERP4_DB=${ERP4_DB:-erp4-case1.db}

echo "=== ERP4 Case Study 1: 完整企業流程 ==="
echo "資料庫: $ERP4_DB"
echo ""

# ============================================================
# 1. 初始化
# ============================================================
echo ">>> 1. 初始化資料庫"
$ERP4 init
echo ""

# ============================================================
# 2. 客戶管理
# ============================================================
echo ">>> 2. 客戶管理"
$ERP4 customer add "台灣電子股份有限公司" --email contact@taiwan-elec.com --phone "02-23456789" --address "台北市大安區信義路100號"
$ERP4 customer add "快速物流行" --email info@fastlogi.com --phone "03-98765432"
$ERP4 customer add "小明工作室" --email "hello@xiaoming.dev"
$ERP4 customer list
echo ""

$ERP4 customer get 1
echo ""

$ERP4 customer update 3 --phone "0912-345-678" --address "台中市西區公益路50號"
$ERP4 customer get 3
echo ""

# ============================================================
# 3. 產品管理
# ============================================================
echo ">>> 3. 產品管理"
$ERP4 product add "智慧手環 Pro" BRACELET-PRO-001 2999.00 --stock 200 --description "心率/血氧/睡眠監測"
$ERP4 product add "無線藍芽耳機" EARPHONE-002 1599.00 --stock 350
$ERP4 product add "Type-C 充電線 1M" CABLE-TC-1M 199.00 --stock 1000 --description "PD 100W 快充"
$ERP4 product add "筆電支架鋁合金" STAND-AL-001 899.00 --stock 80 --description "可調高度/散熱"
$ERP4 product list
echo ""

$ERP4 product get 2
echo ""

# 更新庫存
$ERP4 product update 1 --stock 180
$ERP4 product get 1
echo ""

# ============================================================
# 4. 供應商管理
# ============================================================
echo ">>> 4. 供應商管理"
$ERP4 supplier add "深圳鴻海製造" --contact-person "王經理" --email "wang@honghai.cn" --phone "+86-755-88886666"
$ERP4 supplier add "新竹科園材料" --contact-person "Lisa Chen" --email "lisa@hctech.tw" --phone "03-5771234"
$ERP4 supplier list
echo ""

$ERP4 supplier get 1
echo ""

$ERP4 supplier update 2 --contact-person "Tom Wang"
$ERP4 supplier get 2
echo ""

# ============================================================
# 5. 採購單 (向供應商下單補貨)
# ============================================================
echo ">>> 5. 採購單管理"

# 向深圳鴻海採購智慧手環
$ERP4 purchase-order create 1 --notes "Q3 備貨"
$ERP4 purchase-order add-item 1 1 100 --unit-price 1200.00
$ERP4 purchase-order add-item 1 3 500 --unit-price 50.00
$ERP4 purchase-order items 1
$ERP4 purchase-order get 1
echo ""

# 向新竹科園採購耳機
$ERP4 purchase-order create 2 --notes "緊急補貨"
$ERP4 purchase-order add-item 2 2 200 --unit-price 800.00
$ERP4 purchase-order add-item 2 4 50 --unit-price 400.00
$ERP4 purchase-order get 2
echo ""

# 更新狀態：收貨
$ERP4 purchase-order update-status 1 received
$ERP4 purchase-order update-status 2 approved
$ERP4 purchase-order list
echo ""

# ============================================================
# 6. 銷售訂單 (向客戶出貨)
# ============================================================
echo ">>> 6. 訂單管理"

# 台灣電子下單
$ERP4 order create 1 --notes "官方訂單 #2026001"
$ERP4 order add-item 1 1 10
$ERP4 order add-item 1 2 20
$ERP4 order items 1
$ERP4 order get 1
echo ""

# 快速物流行下單
$ERP4 order create 2
$ERP4 order add-item 2 3 200
$ERP4 order add-item 2 4 30 --unit-price 850.00
$ERP4 order get 2
echo ""

# 小明工作室下單
$ERP4 order create 3 --notes "個人訂單"
$ERP4 order add-item 3 2 2
$ERP4 order add-item 3 3 5
$ERP4 order get 3
echo ""

# 更新訂單狀態
$ERP4 order update-status 1 confirmed
$ERP4 order update-status 2 shipped
$ERP4 order update-status 3 delivered
$ERP4 order list
echo ""

# ============================================================
# 7. 發票管理
# ============================================================
echo ">>> 7. 發票管理"

# 台灣電子的發票 (金額與訂單相同)
$ERP4 invoice create INV-2026-0001 --order-id 1 1 2026-07-15 61970.00
$ERP4 invoice create INV-2026-0002 --order-id 2 2 2026-07-20 65300.00
$ERP4 invoice create INV-2026-0003 --order-id 3 3 2026-06-30 4193.00 --notes "個人電子發票"
$ERP4 invoice list
echo ""

# 更新發票狀態
$ERP4 invoice update-status 2 paid
$ERP4 invoice get 2
echo ""

# ============================================================
# 8. 查詢與刪除操作
# ============================================================
echo ">>> 8. 刪除操作"

# 刪除發票 (order 3 有對應發票，須先刪除)
$ERP4 invoice delete 3
echo ""

# 刪除訂單 3 (含明細自動清除)
$ERP4 order delete 3
echo ""

# 產品 4 目前無未完成的訂單參照 (訂單 2 已 shipped)，可安全刪除
$ERP4 product delete 4
echo ""

# 演示 FK 保護：嘗試刪除有訂單關聯的客戶 (應失敗)
echo "--- 以下應顯示 FK 錯誤 (客戶 1 有關聯訂單) ---"
$ERP4 customer delete 1 || true
echo ""

# 最後列出所有資料確認狀態
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
echo "=== Case Study 1 完成 ==="
