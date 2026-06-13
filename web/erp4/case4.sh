#!/usr/bin/env bash
# v0.5 整合測試：匯入／匯出／JSON 輸出／使用者／工作階段
set -euo pipefail

ERP4=${ERP4:-cargo run --}
DIR="${0%/*}"
DB="erp4_test_v05.db"
CUSTOMERS_CSV=$(mktemp)
PRODUCTS_CSV=$(mktemp)
SUPPLIERS_CSV=$(mktemp)
CUSTOMERS_2_CSV=$(mktemp)

cleanup() {
    rm -f "$DB" "$CUSTOMERS_CSV" "$PRODUCTS_CSV" "$SUPPLIERS_CSV" "$CUSTOMERS_2_CSV"
}
trap cleanup EXIT

# 1. 初始化
ERP4_DB="$DB" $ERP4 init

# 2. 使用者
echo "=== 使用者操作 ==="
ERP4_DB="$DB" $ERP4 user create --username alice --password secret --role user
ERP4_DB="$DB" $ERP4 user create --username bob --password pass456 --role user
ERP4_DB="$DB" $ERP4 user list
ERP4_DB="$DB" $ERP4 user get --username alice
ERP4_DB="$DB" $ERP4 user update --username alice --role manager
ERP4_DB="$DB" $ERP4 user delete --username bob
ERP4_DB="$DB" $ERP4 user list

# 3. 工作階段（登入／登出）
echo "=== 工作階段操作 ==="
ERP4_DB="$DB" $ERP4 session login --username admin --password admin123
ERP4_DB="$DB" $ERP4 session login --username admin --password wrongpass 2>&1 && echo "ERROR: should fail" || true
ERP4_DB="$DB" $ERP4 session list
ERP4_DB="$DB" $ERP4 session logout --username admin

# 4. CSV 匯入
echo "=== CSV 匯入 ==="
cat > "$CUSTOMERS_CSV" <<CSV
name,email,phone,address
匯入客戶一,imp1@test.com,0911-111111,台北市
匯入客戶二,imp2@test.com,0922-222222,台中市
CSV
ERP4_DB="$DB" $ERP4 customer import "$CUSTOMERS_CSV"
ERP4_DB="$DB" $ERP4 customer list

cat > "$PRODUCTS_CSV" <<CSV
name,sku,price,stock,description
匯入產品A,SKU-IMP-A,100,50,測試A
匯入產品B,SKU-IMP-B,200,30,測試B
CSV
ERP4_DB="$DB" $ERP4 product import "$PRODUCTS_CSV"
ERP4_DB="$DB" $ERP4 product list

cat > "$SUPPLIERS_CSV" <<CSV
name,contact_person,email,phone
匯入供應商X,王聯絡,supx@test.com,0933-333333
CSV
ERP4_DB="$DB" $ERP4 supplier import "$SUPPLIERS_CSV"
ERP4_DB="$DB" $ERP4 supplier list

# 5. JSON 輸出
echo "=== JSON 輸出 ==="
ERP4_DB="$DB" $ERP4 customer list --format json
ERP4_DB="$DB" $ERP4 customer get 1 --format json
ERP4_DB="$DB" $ERP4 product list --format json
ERP4_DB="$DB" $ERP4 product get 1 --format json
ERP4_DB="$DB" $ERP4 supplier list --format json
ERP4_DB="$DB" $ERP4 supplier get 1 --format json
ERP4_DB="$DB" $ERP4 order list --format json
ERP4_DB="$DB" $ERP4 purchase-order list --format json
ERP4_DB="$DB" $ERP4 invoice list --format json
ERP4_DB="$DB" $ERP4 report sales --format json
ERP4_DB="$DB" $ERP4 report inventory --format json
ERP4_DB="$DB" $ERP4 report aging --format json

# 6. 匯出
echo "=== 匯出 ==="
ERP4_DB="$DB" $ERP4 export --format json
ERP4_DB="$DB" $ERP4 export --format csv

# 7. CSV 匯入欄位順序不拘
cat > "$CUSTOMERS_2_CSV" <<CSV
phone,name,email
0999-888888,欄位順序測試,order@test.com
CSV
ERP4_DB="$DB" $ERP4 customer import "$CUSTOMERS_2_CSV"
ERP4_DB="$DB" $ERP4 customer list

echo "=== 全部通過 ==="
