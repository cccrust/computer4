#!/usr/bin/env bash
# case2.sh — Web API 整合測試
set -uo pipefail

SMS4=${SMS4:-cargo run --}
SMS4_DB=${SMS4_DB:-sms4-case2.db}
API_PORT=${API_PORT:-9876}
BASE="http://127.0.0.1:$API_PORT/api"

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

echo "=== SMS4 Web API 整合測試 ==="
echo ""

# 初始化資料庫 + 啟動伺服器
export SMS4_DB
SMS4_DB="$SMS4_DB" $SMS4 init
SMS4_DB="$SMS4_DB" $SMS4 web --port "$API_PORT" &
API_PID=$!
sleep 2

echo ""
echo "=== 1. 使用者 API ==="

# POST /api/users 建立使用者
R=$(curl -s -X POST "$BASE/users" \
    -H "Content-Type: application/json" \
    -d '{"username":"alice","display_name":"愛麗絲","bio":"旅行者"}')
assert "POST /api/users 回傳 201" "alice" "$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['username'])")"

R=$(curl -s -X POST "$BASE/users" \
    -H "Content-Type: application/json" \
    -d '{"username":"bob","display_name":"鮑勃","bio":"工程師"}')
assert "POST /api/users 建立 bob" "bob" "$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['username'])")"

R=$(curl -s -X POST "$BASE/users" \
    -H "Content-Type: application/json" \
    -d '{"username":"carol","display_name":"卡蘿","bio":"美食家"}')
assert "POST /api/users 建立 carol" "carol" "$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['username'])")"

# GET /api/users 列表
R=$(curl -s "$BASE/users")
assert_contains "GET /api/users 列出使用者" "alice" "$R"
assert_contains "GET /api/users 列出 bob" "bob" "$R"

# GET /api/users/1 取得單一使用者
R=$(curl -s "$BASE/users/1")
assert "GET /api/users/1 顯示名稱" "愛麗絲" "$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['display_name'])")"
assert "GET /api/users/1 粉絲數" "0" "$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['followers_count'])")"

# PUT /api/users/1 更新使用者
R=$(curl -s -X PUT "$BASE/users/1" \
    -H "Content-Type: application/json" \
    -d '{"bio":"旅行攝影師"}')
assert "PUT /api/users/1 更新簡介" "旅行攝影師" "$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['bio'])")"

# GET /api/users?search=ali 搜尋
R=$(curl -s "$BASE/users?search=bob")
assert_contains "GET /api/users?search=bob" "bob" "$R"

echo ""
echo "=== 2. 貼文 API ==="

# POST /api/posts 建立貼文
R=$(curl -s -X POST "$BASE/posts" \
    -H "Content-Type: application/json" \
    -d '{"user_id":1,"content":"今天天氣真好！去陽明山走了一趟 🌄"}')
assert "POST /api/posts 建立貼文" "今天天氣真好" "$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['content'][:6])")"

R=$(curl -s -X POST "$BASE/posts" \
    -H "Content-Type: application/json" \
    -d '{"user_id":2,"content":"Rust 的 borrow checker 好難 😅"}')
assert "POST /api/posts bob 貼文" "Rust" "$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['content'][:4])")"

R=$(curl -s -X POST "$BASE/posts" \
    -H "Content-Type: application/json" \
    -d '{"user_id":3,"content":"今天吃到一家超讚的甜點店！"}')
assert "POST /api/posts carol 貼文" "今天吃到" "$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['content'][:4])")"

# GET /api/posts 列表
R=$(curl -s "$BASE/posts")
assert_contains "GET /api/posts 列出貼文" "borrow checker" "$R"
assert_contains "GET /api/posts 3 篇貼文" "user_id" "$R"

# GET /api/posts/1 單篇貼文
R=$(curl -s "$BASE/posts/1")
assert_contains "GET /api/posts/1 貼文內容" "陽明山" "$R"

echo ""
echo "=== 3. 回覆 API ==="

# POST /api/posts/1/reply 回覆
R=$(curl -s -X POST "$BASE/posts/1/reply" \
    -H "Content-Type: application/json" \
    -d '{"user_id":2,"content":"陽明山現在花況如何？"}')
assert_contains "POST /posts/1/reply 回覆貼文" "花況" "$R"

R=$(curl -s -X POST "$BASE/posts/1/reply" \
    -H "Content-Type: application/json" \
    -d '{"user_id":3,"content":"求推薦路線！"}')
assert_contains "POST /posts/1/reply 第二則回覆" "路線" "$R"

# GET /api/posts/1 回覆應包含在結果中
R=$(curl -s "$BASE/posts/1")
assert_contains "GET /posts/1 包含回覆" "花況" "$R"
REPLIES=$(echo $R | python3 -c "import sys,json; d=json.load(sys.stdin); print(len(d['replies']))")
assert "貼文 #1 有 2 則回覆" "2" "$REPLIES"

echo ""
echo "=== 4. 追蹤 API ==="

# POST /api/follow 追蹤
R=$(curl -s -X POST "$BASE/follow" \
    -H "Content-Type: application/json" \
    -d '{"follower_id":1,"followee_id":2}')
assert_contains "POST /follow alice -> bob" "已追蹤" "$R"

R=$(curl -s -X POST "$BASE/follow" \
    -H "Content-Type: application/json" \
    -d '{"follower_id":1,"followee_id":3}')
assert_contains "POST /follow alice -> carol" "已追蹤" "$R"

R=$(curl -s -X POST "$BASE/follow" \
    -H "Content-Type: application/json" \
    -d '{"follower_id":2,"followee_id":1}')
assert_contains "POST /follow bob -> alice" "已追蹤" "$R"

# 測試不能追蹤自己
R=$(curl -s -X POST "$BASE/follow" \
    -H "Content-Type: application/json" \
    -d '{"follower_id":1,"followee_id":1}')
assert_contains "POST /follow 不能追蹤自己" "不能追蹤自己" "$R"

# GET /api/users/1/followers 粉絲列表
R=$(curl -s "$BASE/users/1/followers")
assert_contains "GET /users/1/followers 有 bob" "bob" "$R"

# GET /api/users/1/following 追蹤中
R=$(curl -s "$BASE/users/1/following")
assert_contains "GET /users/1/following 有 bob" "bob" "$R"
assert_contains "GET /users/1/following 有 carol" "carol" "$R"

# DELETE /api/follow 取消追蹤
R=$(curl -s -X DELETE "$BASE/follow" \
    -H "Content-Type: application/json" \
    -d '{"follower_id":1,"followee_id":3}')
assert_contains "DELETE /follow 取消追蹤 carol" "已取消" "$R"

R=$(curl -s "$BASE/users/1/following")
assert_contains "GET /users/1/following 只剩 bob" "bob" "$R"

echo ""
echo "=== 5. 按讚 API ==="

# POST /api/likes 按讚
R=$(curl -s -X POST "$BASE/likes" \
    -H "Content-Type: application/json" \
    -d '{"user_id":2,"post_id":1}')
assert_contains "POST /likes bob 對貼文 1 按讚" "已按讚" "$R"

R=$(curl -s -X POST "$BASE/likes" \
    -H "Content-Type: application/json" \
    -d '{"user_id":3,"post_id":1}')
assert_contains "POST /likes carol 對貼文 1 按讚" "已按讚" "$R"

R=$(curl -s -X POST "$BASE/likes" \
    -H "Content-Type: application/json" \
    -d '{"user_id":1,"post_id":2}')
assert_contains "POST /likes alice 對貼文 2 按讚" "已按讚" "$R"

# 重複按讚
R=$(curl -s -X POST "$BASE/likes" \
    -H "Content-Type: application/json" \
    -d '{"user_id":2,"post_id":1}')
assert_contains "POST /likes 重複按讚" "已經按過讚了" "$R"

# 驗證讚數
R=$(curl -s "$BASE/posts/1")
LIKE_COUNT=$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['post']['likes_count'])")
assert "貼文 #1 讚數為 2" "2" "$LIKE_COUNT"

# DELETE /api/likes 取消讚
R=$(curl -s -X DELETE "$BASE/likes" \
    -H "Content-Type: application/json" \
    -d '{"user_id":2,"post_id":1}')
assert_contains "DELETE /likes 取消讚" "已取消" "$R"

R=$(curl -s "$BASE/posts/1")
LIKE_COUNT=$(echo $R | python3 -c "import sys,json; print(json.load(sys.stdin)['post']['likes_count'])")
assert "取消後貼文 #1 讚數為 1" "1" "$LIKE_COUNT"

echo ""
echo "=== 6. 時間軸 API ==="

# GET /api/users/1/timeline (alice 追蹤了 bob)
R=$(curl -s "$BASE/users/1/timeline")
assert_contains "GET /users/1/timeline 有 bob 貼文" "borrow checker" "$R"
assert_contains "GET /users/1/timeline 有 alice 自己的貼文" "陽明山" "$R"

echo ""
echo "=== 7. 刪除操作 ==="

# DELETE /api/posts/3 刪除貼文
R=$(curl -s -X DELETE "$BASE/posts/3")
assert_contains "DELETE /posts/3 刪除貼文" "true" "$R"

# DELETE /api/users/3 刪除使用者 (應失敗，因為 carol 有追蹤關係)
R=$(curl -s -X DELETE "$BASE/users/3")
assert_contains "DELETE /users/3 因 FK 失敗" "錯誤" "$R"

echo ""
echo "=== 8. 錯誤處理 ==="

# 不存在的使用者
R=$(curl -s -w "%{http_code}" "$BASE/users/999")
HTTP_CODE="${R: -3}"
assert "GET /users/999 回傳 404" "404" "$HTTP_CODE"

# 不存在的貼文
R=$(curl -s -w "%{http_code}" "$BASE/posts/999")
HTTP_CODE="${R: -3}"
assert "GET /posts/999 回傳 404" "404" "$HTTP_CODE"

echo ""
# cleanup 會在 trap 中自動執行
