set -x
RUST_BACKTRACE=1 cargo run
# 伺服器在 http://0.0.0.0:8080 執行
# 資料庫為 SQLite (shop4.db)，自動初始化並預載 10 個分類。