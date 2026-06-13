#!/usr/bin/env bash
set -uo pipefail

SMS4=${SMS4:-cargo run --}
export SMS4_DB=${SMS4_DB:-sms4-case1.db}

echo "=== SMS4 Case Study 1: 手機社群平台基礎流程 ==="
echo "資料庫: $SMS4_DB"
echo ""

# ============================================================
# 1. 初始化
# ============================================================
echo ">>> 1. 初始化資料庫"
$SMS4 init
echo ""

# ============================================================
# 2. 使用者管理
# ============================================================
echo ">>> 2. 使用者管理"
$SMS4 user add alice 愛麗絲 --bio "喜歡旅行和攝影 📸"
$SMS4 user add bob 鮑勃 --bio "程式設計師 & 咖啡愛好者"
$SMS4 user add carol 卡蘿 --bio "美食部落客"
$SMS4 user add dave 大衛
$SMS4 user list
echo ""

$SMS4 user get 1
echo ""

$SMS4 user update 4 --bio "音樂創作者"
$SMS4 user get 4
echo ""

# ============================================================
# 3. 貼文管理
# ============================================================
echo ">>> 3. 貼文管理"
$SMS4 post add 1 "今天天氣真好，去陽明山走了一趟！"
$SMS4 post add 1 "剛看完一本好書，推薦給大家 📚"
$SMS4 post add 2 "Rust 的 borrow checker 真是令人又愛又恨 😅"
$SMS4 post add 2 "今天喝了一杯超棒的衣索比亞咖啡"
$SMS4 post add 3 "台北新開的甜點店，提拉米蘇超讚！"
$SMS4 post list
echo ""

# ============================================================
# 4. 回覆貼文
# ============================================================
echo ">>> 4. 回覆貼文"
$SMS4 post reply 1 2 "陽明山現在花況如何？"
$SMS4 post reply 1 3 "求推薦路線！"
$SMS4 post reply 3 1 "我也在學 Rust，真的很有挑戰性"
$SMS4 post reply 3 3 "在哪裡？想吃！"
echo ""

# 檢視貼文（含回覆）
$SMS4 post get 1
echo ""
$SMS4 post get 3
echo ""

# ============================================================
# 5. 追蹤功能
# ============================================================
echo ">>> 5. 追蹤功能"
$SMS4 follow add 1 2
$SMS4 follow add 1 3
$SMS4 follow add 2 1
$SMS4 follow add 3 1
$SMS4 follow add 4 1
$SMS4 follow add 4 2
$SMS4 follow add 4 3
echo ""

$SMS4 follow followers 1
echo ""
$SMS4 follow following 4
echo ""

# 測試不能追蹤自己
echo "--- 測試不能追蹤自己 ---"
$SMS4 follow add 1 1 || true
echo ""

# ============================================================
# 6. 讚功能
# ============================================================
echo ">>> 6. 按讚"
$SMS4 like add 2 1
$SMS4 like add 3 1
$SMS4 like add 4 1
$SMS4 like add 1 3
$SMS4 like add 4 3
echo ""

$SMS4 post get 1
echo ""
$SMS4 post get 3
echo ""

# 測試重複按讚
echo "--- 測試重複按讚 ---"
$SMS4 like add 2 1 || true
echo ""

$SMS4 like remove 2 1
$SMS4 post get 1
echo ""

# ============================================================
# 7. 時間軸（只看自己和追蹤的人）
# ============================================================
echo ">>> 7. 時間軸"
echo "--- 愛麗絲的時間軸 (追蹤 bob, carol) ---"
$SMS4 post timeline 1
echo ""

echo "--- 大衛的時間軸 (追蹤 alice, bob, carol) ---"
$SMS4 post timeline 4
echo ""

# ============================================================
# 8. 刪除操作
# ============================================================
echo ">>> 8. 刪除操作"
$SMS4 post delete 5
echo ""

$SMS4 follow remove 4 3
$SMS4 follow following 4
echo ""

$SMS4 user delete 4
$SMS4 user list
echo ""

# ============================================================
# 9. 最終狀態確認
# ============================================================
echo "=== 最終狀態 ==="
echo "--- 使用者 ---"
$SMS4 user list
echo "--- 貼文 ---"
$SMS4 post list
echo "--- 追蹤 ---"
$SMS4 follow followers 1
echo ""

echo "=== Case Study 1 完成 ==="
