#!/usr/bin/env bash
# case5.sh — 交友配對功能完整整合測試（CLI + Web API + 前端）
set -uo pipefail

SMS4=${SMS4:-cargo run --}
SMS4_DB=${SMS4_DB:-sms4-case5.db}
API_PORT=${API_PORT:-9879}
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

echo "=== SMS4 交友配對功能完整整合測試 ==="
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
echo "=== 1. CLI 交友資料設定 ==="

# 建立測試使用者
$SMS4 user add alice 愛麗絲 --bio "喜歡旅行和攝影" > /dev/null 2>&1
$SMS4 user add bob 鮑勃 --bio "程式設計師" > /dev/null 2>&1
$SMS4 user add carol 卡蘿 --bio "美食部落客" > /dev/null 2>&1

# 設定 alice 的交友資料
R=$($SMS4 profile set 1 --birthday 1995-03-15 --gender male --city 台北 --occupation 工程師 --height 175 --looking-for friend --about-me "喜歡爬山和攝影" 2>&1)
assert_contains "profile set alice" "已更新" "$R"

# 設定 bob 的交友資料
R=$($SMS4 profile set 2 --birthday 1998-07-20 --gender female --city 台中 --occupation 設計師 --height 160 --looking-for any --about-me "咖啡和貓" 2>&1)
assert_contains "profile set bob" "已更新" "$R"

# 設定 carol 的交友資料
R=$($SMS4 profile set 3 --birthday 2000-12-01 --gender female --city 高雄 --occupation 美食部落客 --height 165 --looking-for friend --about-me "到處吃美食" 2>&1)
assert_contains "profile set carol" "已更新" "$R"

# 檢視交友資料
R=$($SMS4 profile show 1 2>&1)
assert_contains "profile show alice 看得到生日" "1995-03-15" "$R"
assert_contains "profile show alice 看得到性別" "male" "$R"
assert_contains "profile show alice 看得到城市" "台北" "$R"

# 設定興趣標籤
R=$($SMS4 interest add 1 爬山 2>&1)
assert_contains "interest add alice 爬山" "已新增" "$R"
R=$($SMS4 interest add 1 攝影 2>&1)
assert_contains "interest add alice 攝影" "已新增" "$R"
R=$($SMS4 interest add 2 咖啡 2>&1)
assert_contains "interest add bob 咖啡" "已新增" "$R"
R=$($SMS4 interest add 2 設計 2>&1)
assert_contains "interest add bob 設計" "已新增" "$R"
R=$($SMS4 interest add 3 美食 2>&1)
assert_contains "interest add carol 美食" "已新增" "$R"
R=$($SMS4 interest add 3 攝影 2>&1)
assert_contains "interest add carol 攝影" "已新增" "$R"

# 列出興趣標籤
R=$($SMS4 interest list 1 2>&1)
assert_contains "interest list alice 有爬山" "爬山" "$R"
assert_contains "interest list alice 有攝影" "攝影" "$R"

# 移除此興趣標籤
R=$($SMS4 interest remove 1 爬山 2>&1)
assert_contains "interest remove alice 爬山" "已移除" "$R"
R=$($SMS4 interest list 1 2>&1)
assert_contains "interest list alice 剩攝影" "攝影" "$R"
assert "interest list alice 無爬山" "0" "$(echo "$R" | grep -c "爬山")"

# 重新加入爬山
$SMS4 interest add 1 爬山 > /dev/null 2>&1

echo ""
echo "=== 2. CLI 交友搜尋 ==="

# 依性別搜尋
R=$($SMS4 profile search --gender male 2>&1)
assert_contains "search gender=male 看到愛麗絲" "愛麗絲" "$R"
assert "search gender=male 回傳 1 人" "1" "$(echo "$R" | grep -c "配對結果")"

# 依城市模糊搜尋
R=$($SMS4 profile search --city 台 2>&1)
assert_contains "search city=台 看到 bob" "鮑勃" "$R"
assert_contains "search city=台 看到 台北" "愛麗絲" "$R"

# 依興趣標籤搜尋
R=$($SMS4 profile search --tags 攝影 2>&1)
assert_contains "search tags=攝影 看到愛麗絲" "愛麗絲" "$R"
assert_contains "search tags=攝影 看到卡蘿" "卡蘿" "$R"

# 依關鍵字搜尋 (about_me)
R=$($SMS4 profile search -q 咖啡 2>&1)
assert_contains "search q=咖啡 看到 bob" "鮑勃" "$R"

# 依年齡區間搜尋
R=$($SMS4 profile search --age-min 25 --age-max 30 2>&1)
assert_contains "search age 25-30 看到 bob" "鮑勃" "$R"
assert_contains "search age 25-30 看到卡蘿" "卡蘿" "$R"

# 無符合結果
R=$($SMS4 profile search --gender male --city 台中 2>&1)
assert_contains "search gender=male city=台中 無結果" "沒有符合條件" "$R"

echo ""
echo "=== 3. Web API 交友設定測試 ==="

# 更新 alice 的交友資料
R=$(curl -s -X PUT "$BASE/api/profiles/1" \
    -H "Content-Type: application/json" \
    -d '{"city":"新北","occupation":"資深工程師","about_me":"喜歡爬山和攝影，最近迷上露營"}')
assert_contains "PUT /api/profiles/1 回傳成功" "已更新" "$R"

# 讀取 alice 的交友資料
R=$(curl -s "$BASE/api/profiles/1")
assert_contains "GET /api/profiles/1 有城市" "新北" "$R"
assert_contains "GET /api/profiles/1 有興趣標籤" "爬山" "$R"

# Web API 興趣標籤管理
R=$(curl -s -X POST "$BASE/api/interests" \
    -H "Content-Type: application/json" \
    -d '{"user_id":1,"tag":"露營"}')
assert_contains "POST /api/interests 新增露營" "已新增" "$R"

R=$(curl -s "$BASE/api/interests/1")
assert_contains "GET /api/interests/1 有露營" "露營" "$R"

R=$(curl -s -X DELETE "$BASE/api/interests" \
    -H "Content-Type: application/json" \
    -d '{"user_id":1,"tag":"露營"}')
assert_contains "DELETE /api/interests 移除露營" "已移除" "$R"

echo ""
echo "=== 4. Web API 交友搜尋測試 ==="

# 多條件搜尋
R=$(curl -s "$BASE/api/profiles/search?gender=female&city=台")
assert_contains "search gender=female city=台 看得到鮑勃" "鮑勃" "$R"
assert "search gender=female city=台 回傳 1 人" "1" "$(echo "$R" | python3 -c "import sys,json; print(json.load(sys.stdin)['count'])")"

# 興趣搜尋
R=$(curl -s "$BASE/api/profiles/search?tags=攝影")
assert_contains "search tags=攝影 回傳結果" "results" "$R"

echo ""
echo "=== 5. 前端頁面測試 ==="

R=$(curl -s "$BASE/profile/1")
assert_contains "GET /profile/1 SPA 路由" "root" "$R"

R=$(curl -s "$BASE/search")
assert_contains "GET /search SPA 路由" "root" "$R"

R=$(curl -s "$BASE/profile/edit")
assert_contains "GET /profile/edit SPA 路由" "root" "$R"

echo ""
echo "=== 6. 錯誤處理 ==="

# 不存在的使用者檢視交友資料
R=$(curl -s -w "%{http_code}" "$BASE/api/profiles/999")
HTTP_CODE="${R: -3}"
assert "GET /api/profiles/999 回傳 404" "404" "$HTTP_CODE"

# 重複興趣標籤
R=$(curl -s -X POST "$BASE/api/interests" \
    -H "Content-Type: application/json" \
    -d '{"user_id":2,"tag":"咖啡"}')
assert_contains "POST /api/interests 重複標籤仍回成功" "id" "$R"

# 移除不存在的興趣標籤
R=$(curl -s -w "%{http_code}" -X DELETE "$BASE/api/interests" \
    -H "Content-Type: application/json" \
    -d '{"user_id":2,"tag":"不存在標籤"}')
HTTP_CODE="${R: -3}"
assert "DELETE /api/interests 不存在標籤回傳 404" "404" "$HTTP_CODE"

echo ""
# cleanup 在 trap 中自動執行
