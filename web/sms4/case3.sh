#!/usr/bin/env bash
# case3.sh — 前端 + Web API 完整整合測試（production 模式）
set -uo pipefail

SMS4=${SMS4:-cargo run --}
SMS4_DB=${SMS4_DB:-sms4-case3.db}
API_PORT=${API_PORT:-9877}
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

echo "=== SMS4 前端 + Web API 完整整合測試 ==="
echo ""

# 確保前端已建置
if [ ! -f "web/dist/index.html" ]; then
    echo "建置前端..."
    (cd web && npm run build) || {
        echo "❌ 前端建置失敗"
        exit 1
    }
fi

# 初始化資料庫 + 啟動伺服器 (production 模式，無 --dev)
export SMS4_DB
SMS4_DB="$SMS4_DB" $SMS4 init
SMS4_DB="$SMS4_DB" $SMS4 web --port "$API_PORT" &
API_PID=$!
sleep 2

echo ""
echo "=== 1. 靜態檔案服務 ==="

# 測試 index.html 被正確服務
R=$(curl -s "$BASE/")
assert_contains "GET / 回傳 index.html" "<title>SMS4</title>" "$R"
assert_contains "GET / 包含 root div" 'id="root"' "$R"

# 測試 SPA 路由回傳 index.html（而非 404）
R=$(curl -s "$BASE/users")
assert_contains "GET /users SPA 路由回傳 index.html" "<title>SMS4</title>" "$R"

R=$(curl -s "$BASE/users/1")
assert_contains "GET /users/1 SPA 路由回傳 index.html" "<title>SMS4</title>" "$R"

R=$(curl -s "$BASE/posts/1")
assert_contains "GET /posts/1 SPA 路由回傳 index.html" "<title>SMS4</title>" "$R"

# 測試 JS bundle 可存取
JS_PATH=$(grep -o 'src="/[^"]*"' web/dist/index.html | sed 's/src="//;s/"//' | head -1)
R=$(curl -s -o /dev/null -w "%{http_code}" "$BASE$JS_PATH")
assert "GET $JS_PATH 回傳 200" "200" "$R"

echo ""
echo "=== 2. API + 前端資料流程 ==="

# 建立使用者
R=$(curl -s -X POST "$BASE/api/users" \
    -H "Content-Type: application/json" \
    -d '{"username":"alice","display_name":"愛麗絲","bio":"旅行攝影師"}')
assert_contains "POST /api/users 建立 alice" "alice" "$R"

R=$(curl -s -X POST "$BASE/api/users" \
    -H "Content-Type: application/json" \
    -d '{"username":"bob","display_name":"鮑勃"}')
assert_contains "POST /api/users 建立 bob" "bob" "$R"

R=$(curl -s -X POST "$BASE/api/users" \
    -H "Content-Type: application/json" \
    -d '{"username":"carol","display_name":"卡蘿","bio":"美食部落客"}')
assert_contains "POST /api/users 建立 carol" "carol" "$R"

# 建立貼文
R=$(curl -s -X POST "$BASE/api/posts" \
    -H "Content-Type: application/json" \
    -d '{"user_id":1,"content":"今天天氣真好！去陽明山走了一趟 🌄"}')
assert_contains "POST /api/posts alice 貼文" "陽明山" "$R"

R=$(curl -s -X POST "$BASE/api/posts" \
    -H "Content-Type: application/json" \
    -d '{"user_id":2,"content":"Rust 的 borrow checker 好難 😅"}')
assert_contains "POST /api/posts bob 貼文" "borrow checker" "$R"

R=$(curl -s -X POST "$BASE/api/posts" \
    -H "Content-Type: application/json" \
    -d '{"user_id":3,"content":"今天發現一家超讚的咖啡廳！"}')
assert_contains "POST /api/posts carol 貼文" "咖啡廳" "$R"

# 回覆
R=$(curl -s -X POST "$BASE/api/posts/1/reply" \
    -H "Content-Type: application/json" \
    -d '{"user_id":2,"content":"現在芒草季很美！"}')
assert_contains "POST /api/posts/1/reply 回覆" "芒草季" "$R"

# 追蹤
curl -s -X POST "$BASE/api/follow" \
    -H "Content-Type: application/json" \
    -d '{"follower_id":1,"followee_id":2}' > /dev/null
curl -s -X POST "$BASE/api/follow" \
    -H "Content-Type: application/json" \
    -d '{"follower_id":1,"followee_id":3}' > /dev/null
curl -s -X POST "$BASE/api/follow" \
    -H "Content-Type: application/json" \
    -d '{"follower_id":2,"followee_id":1}' > /dev/null

# 時間軸
R=$(curl -s "$BASE/api/users/1/timeline")
assert_contains "GET /api/users/1/timeline 有 bob 貼文" "borrow checker" "$R"
assert_contains "GET /api/users/1/timeline 有 carol 貼文" "咖啡廳" "$R"
assert_contains "GET /api/users/1/timeline 有 alice 自己的貼文" "陽明山" "$R"

# 按讚
curl -s -X POST "$BASE/api/likes" \
    -H "Content-Type: application/json" \
    -d '{"user_id":2,"post_id":1}' > /dev/null
curl -s -X POST "$BASE/api/likes" \
    -H "Content-Type: application/json" \
    -d '{"user_id":3,"post_id":1}' > /dev/null

R=$(curl -s "$BASE/api/posts/1")
LIKE_COUNT=$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['post']['likes_count'])")
assert "貼文 #1 讚數為 2" "2" "$LIKE_COUNT"

echo ""
echo "=== 3. 跨端資料一致性（CLI + Web API）==="

# 用 CLI 檢查 Web API 建立的資料
R=$($SMS4 user list 2>&1 | grep -c "alice")
assert "CLI 可看到 alice (>=1)" "1" "$(echo $R)"

R=$($SMS4 post list 2>&1 | grep -c "陽明山")
assert "CLI 可看到陽明山貼文 (>=1)" "1" "$(echo $R)"

# 用 CLI 新增資料，再透過 Web API 檢查
$SMS4 user add dave 大衛 --bio "音樂人" 2>&1 > /dev/null
R=$(curl -s "$BASE/api/users")
assert_contains "Web API 可看到 CLI 新增的 dave" "dave" "$R"

echo ""
echo "=== 4. 錯誤處理 ==="

R=$(curl -s -w "%{http_code}" "$BASE/api/users/999")
HTTP_CODE="${R: -3}"
assert "GET /api/users/999 回傳 404" "404" "$HTTP_CODE"

R=$(curl -s -w "%{http_code}" "$BASE/api/posts/999")
HTTP_CODE="${R: -3}"
assert "GET /api/posts/999 回傳 404" "404" "$HTTP_CODE"

echo ""
# cleanup 在 trap 中自動執行
