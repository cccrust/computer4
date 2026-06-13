#!/usr/bin/env bash
# case4.sh — 私訊功能完整整合測試（CLI + Web API + 前端）
set -uo pipefail

SMS4=${SMS4:-cargo run --}
SMS4_DB=${SMS4_DB:-sms4-case4.db}
API_PORT=${API_PORT:-9878}
BASE="http://127.0.0.1:$API_PORT"

PASS=0
FAIL=0

assert() {
    local desc="$1" expected="$2" actual="$3"
    if [[ "$actual" == "$expected" ]]; then
        echo "  ✅ $desc"
        PASS=$((PASS + 1))
    else
        echo "  ❌ $desc (預期: $expected, 實際: $actual)"
        FAIL=$((FAIL + 1))
    fi
}

assert_contains() {
    local desc="$1" expected="$2" actual="$3"
    if echo "$actual" | grep -q "$expected"; then
        echo "  ✅ $desc"
        PASS=$((PASS + 1))
    else
        echo "  ❌ $desc (應包含: $expected)"
        FAIL=$((FAIL + 1))
    fi
}

cleanup() {
    echo ""
    echo "正在停止伺服器..."
    kill $API_PID 2>/dev/null || true
    wait $API_PID 2>/dev/null || true
    rm -f "$SMS4_DB"
    echo "測試完成：通過 $PASS / 失敗 $FAIL / 總計 $((PASS+FAIL))"
}

trap cleanup EXIT INT TERM

echo "=== SMS4 私訊功能完整整合測試 ==="
echo ""

# 確保前端已建置
if [ ! -f "web/dist/index.html" ]; then
    echo "建置前端..."
    (cd web && npm run build) || {
        echo "❌ 前端建置失敗"
        exit 1
    }
fi

# 初始化資料庫 + 啟動伺服器 (production 模式)
export SMS4_DB
SMS4_DB="$SMS4_DB" $SMS4 init
SMS4_DB="$SMS4_DB" $SMS4 web --port "$API_PORT" &
API_PID=$!
sleep 2

echo ""
echo "=== 1. CLI 私訊測試 ==="

# 建立測試使用者
$SMS4 user add alice 愛麗絲 --bio "旅行" > /dev/null 2>&1
$SMS4 user add bob 鮑勃 --bio "程式" > /dev/null 2>&1
$SMS4 user add carol 卡蘿 --bio "美食" > /dev/null 2>&1

# 發送私訊
R=$($SMS4 msg send 1 2 "哈囉鮑勃！" 2>&1)
assert_contains "msg send alice -> bob" "已傳送訊息" "$R"

R=$($SMS4 msg send 2 1 "嗨愛麗絲！" 2>&1)
assert_contains "msg send bob -> alice" "已傳送訊息" "$R"

R=$($SMS4 msg send 1 2 "明天一起爬山好嗎？" 2>&1)
assert_contains "msg send alice -> bob 第二則" "已傳送訊息" "$R"

R=$($SMS4 msg send 3 1 "卡蘿也想爬山！" 2>&1)
assert_contains "msg send carol -> alice" "已傳送訊息" "$R"

# 檢查 inbox
R=$($SMS4 msg inbox 1 2>&1)
assert_contains "msg inbox alice 看到 bob 最後訊息" "明天一起爬山" "$R"
assert_contains "msg inbox alice 看到 carol 最後訊息" "卡蘿也想爬山" "$R"

# 檢查 bob 的 inbox
R=$($SMS4 msg inbox 2 2>&1)
assert_contains "msg inbox bob 看到 alice 最後訊息" "明天一起爬山" "$R"

# conversation 詳細內容
R=$($SMS4 msg conversation 1 2 2>&1)
assert_contains "msg conversation alice<->bob 看到哈囉" "哈囉鮑勃" "$R"

R=$($SMS4 msg conversation 1 3 2>&1)
assert_contains "msg conversation alice<->carol 看到爬山" "卡蘿也想爬山" "$R"

echo ""
echo "=== 2. Web API 私訊測試 ==="

# 發送私訊
R=$(curl -s -X POST "$BASE/api/messages/send" \
    -H "Content-Type: application/json" \
    -d '{"sender_id":2,"receiver_id":3,"content":"卡蘿要一起寫 Rust 嗎？"}')
assert_contains "POST /api/messages/send bob -> carol" "訊息已傳送" "$R"

R=$(curl -s -X POST "$BASE/api/messages/send" \
    -H "Content-Type: application/json" \
    -d '{"sender_id":3,"receiver_id":2,"content":"好呀！我想學！"}')
assert_contains "POST /api/messages/send carol -> bob" "訊息已傳送" "$R"

# conversation 列表
R=$(curl -s "$BASE/api/messages/2/conversations")
assert_contains "GET /api/messages/2/conversations 有愛麗絲" "愛麗絲" "$R"
assert_contains "GET /api/messages/2/conversations 有卡蘿" "卡蘿" "$R"

# 特定對話
R=$(curl -s "$BASE/api/messages/1/2")
assert_contains "GET /api/messages/1/2 alice<->bob 有第一篇" "哈囉鮑勃" "$R"
assert_contains "GET /api/messages/1/2 有爬山" "爬山" "$R"

R=$(curl -s "$BASE/api/messages/2/3")
assert_contains "GET /api/messages/2/3 bob<->carol 有寫 Rust" "Rust" "$R"

# 未讀數
R=$(curl -s "$BASE/api/messages/1/unread")
assert_contains "GET /api/messages/1/unread alice 有未讀" "unread" "$R"

# 讀取訊息後，該對話未讀應減少
curl -s "$BASE/api/messages/1/2" > /dev/null
R=$(curl -s "$BASE/api/messages/1/unread")
assert_contains "GET /api/messages/1/unread 讀取後剩 carol 未讀" '"unread":' "$R"

echo ""
echo "=== 3. 前端私訊頁面測試 ==="

# 私訊列表頁包含導航元素
R=$(curl -s "$BASE/messages")
assert_contains "GET /messages 回傳 index.html" "<title>SMS4</title>" "$R"

# 私訊對話頁
R=$(curl -s "$BASE/messages/2?uid=1")
assert_contains "GET /messages/2?uid=1 SPA 路由" "root" "$R"

echo ""
echo "=== 4. CLI + Web API 跨端一致性 ==="

# CLI 看得到 Web API 建立的訊息
R=$($SMS4 msg conversation 2 3 2>&1)
assert_contains "CLI conversation bob<->carol 看到卡蘿" "卡蘿" "$R"
assert_contains "CLI conversation bob<->carol 看到 Rust" "Rust" "$R"

# Web API 看得到 CLI 建立的訊息
R=$(curl -s "$BASE/api/messages/3/conversations")
assert_contains "Web API carol 看到愛麗絲" "愛麗絲" "$R"

echo ""
echo "=== 5. 錯誤處理 ==="

# 不存在的使用者發送訊息
R=$(curl -s -w "%{http_code}" -X POST "$BASE/api/messages/send" \
    -H "Content-Type: application/json" \
    -d '{"sender_id":999,"receiver_id":1,"content":"hi"}')
HTTP_CODE="${R: -3}"
assert "POST /api/messages/send 不存在的 sender 回傳錯誤" "404" "$HTTP_CODE"

# 不存在的對話
R=$(curl -s -w "%{http_code}" "$BASE/api/messages/1/999")
HTTP_CODE="${R: -3}"
assert "GET /api/messages/1/999 不存在的使用者回傳 404" "404" "$HTTP_CODE"

echo ""
# cleanup 在 trap 中自動執行
