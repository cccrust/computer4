#!/usr/bin/env bash
# seed.sh — 填入大量測試假資料
set -euo pipefail

SMS4=${SMS4:-cargo run --}
DB="${SMS4_DB:-sms4-dev.db}"

if [ -f "$DB" ]; then
    echo "⚠️  資料庫 $DB 已存在，刪除重建..."
    rm -f "$DB"
fi

echo ""
echo "=== 1. 初始化資料庫 ==="
SMS4_DB="$DB" $SMS4 init

echo ""
echo "=== 2. 建立使用者 (15 筆) ==="
SMS4_DB="$DB" $SMS4 user add alice    愛麗絲   --bio "喜歡旅行和攝影 📸"
SMS4_DB="$DB" $SMS4 user add bob      鮑勃     --bio "程式設計師 & 咖啡愛好者"
SMS4_DB="$DB" $SMS4 user add carol    卡蘿     --bio "美食部落客"
SMS4_DB="$DB" $SMS4 user add dave     大衛     --bio "音樂創作者"
SMS4_DB="$DB" $SMS4 user add eve      小伊     --bio "瑜伽老師"
SMS4_DB="$DB" $SMS4 user add frank    法蘭克   --bio "股票投資人"
SMS4_DB="$DB" $SMS4 user add grace    葛蕾絲   --bio "插畫家"
SMS4_DB="$DB" $SMS4 user add henry    亨利     --bio "健身教練"
SMS4_DB="$DB" $SMS4 user add iris     艾瑞絲   --bio "美妝達人"
SMS4_DB="$DB" $SMS4 user add jack     傑克     --bio "旅遊部落客"
SMS4_DB="$DB" $SMS4 user add kate     凱特     --bio "書評作家"
SMS4_DB="$DB" $SMS4 user add leo      里歐     --bio "寵物攝影師"
SMS4_DB="$DB" $SMS4 user add may      小梅     --bio "手作達人"
SMS4_DB="$DB" $SMS4 user add nick     尼克     --bio "科技評論"
SMS4_DB="$DB" $SMS4 user add olivia   奧莉維亞 --bio "環保倡議者"

echo ""
echo "=== 3. 建立貼文 (30 筆) ==="

SMS4_DB="$DB" $SMS4 post add 1 "今天天氣真好，去陽明山走了一趟！滿山遍野的芒草超美"
SMS4_DB="$DB" $SMS4 post add 1 "剛看完《原子習慣》，推薦給想要改變生活的人 📚"
SMS4_DB="$DB" $SMS4 post add 2 "Rust 的 borrow checker 真是令人又愛又恨 😅 但寫出來的程式就是安心"
SMS4_DB="$DB" $SMS4 post add 2 "今天喝了一杯衣索比亞 74158 品種的咖啡，果香炸裂！"
SMS4_DB="$DB" $SMS4 post add 3 "台北東區新開的甜點店「甜蜜時光」，提拉米蘇入口即化！"
SMS4_DB="$DB" $SMS4 post add 3 "自製番茄肉醬義大利麵，簡單又好吃 🍝"
SMS4_DB="$DB" $SMS4 post add 4 "新寫了一首鋼琴曲，靈感來自雨天的窗景 🎹"
SMS4_DB="$DB" $SMS4 post add 5 "清晨的瑜珈練習，讓一整天都充滿能量 🧘"
SMS4_DB="$DB" $SMS4 post add 6 "台積電法說會重點整理，毛利率表現優於預期 📈"
SMS4_DB="$DB" $SMS4 post add 7 "新作品完成！水彩畫了一隻慵懶的貓咪 🎨"
SMS4_DB="$DB" $SMS4 post add 8 "深蹲 5x5 挑戰第二週，開始感受到進步了 💪"
SMS4_DB="$DB" $SMS4 post add 9 "今年春夏眼影趨勢：蜜桃色系和大地色系依然主流"
SMS4_DB="$DB" $SMS4 post add 10 "日本京都賞楓攻略，必去景點總整理 🍁"
SMS4_DB="$DB" $SMS4 post add 11 "讀完《挪威的森林》，村上的文字還是那麼令人沉浸"
SMS4_DB="$DB" $SMS4 post add 12 "今天幫一隻黃金獵犬拍寫真，牠的笑容太治癒了 🐕"
SMS4_DB="$DB" $SMS4 post add 13 "新做的皮革零錢包，手縫真的很療癒"
SMS4_DB="$DB" $SMS4 post add 14 "Apple Vision Pro 的空間運算到底是不是下一個風口？"
SMS4_DB="$DB" $SMS4 post add 15 "週末去海邊淨灘，一個小時撿了 5 公斤的垃圾 🌊"
SMS4_DB="$DB" $SMS4 post add 1 "分享一個陽明山秘境：冷水坑往擎天崗的路上"
SMS4_DB="$DB" $SMS4 post add 2 "VS Code 外掛推薦：Rust Analyzer 真的是必裝"
SMS4_DB="$DB" $SMS4 post add 3 "高雄新開的咖啡廳，水泥風格裝潢很適合拍照"
SMS4_DB="$DB" $SMS4 post add 5 "空中瑜珈初體驗，比想像中難好多但很好玩！"
SMS4_DB="$DB" $SMS4 post add 6 "指數投資 vs 主動選股，我的配置心得"
SMS4_DB="$DB" $SMS4 post add 10 "台東池上的稻田真的太美了，台灣的後花園 🌾"
SMS4_DB="$DB" $SMS4 post add 12 "貓咪到底為什麼那麼喜歡紙箱？🐱"
SMS4_DB="$DB" $SMS4 post add 14 "GitHub Copilot 真的讓寫程式效率提升不少"
SMS4_DB="$DB" $SMS4 post add 15 "今天開始挑戰 zero waste 生活，第一週目標減少塑膠"
SMS4_DB="$DB" $SMS4 post add 1 "攝影筆記：清晨的黃金時刻光線最美，推薦大家試試"
SMS4_DB="$DB" $SMS4 post add 2 "為什麼 Rust 的 Option 和 Result 讓錯誤處理這麼優雅？"
SMS4_DB="$DB" $SMS4 post add 3 "台中第二市場的滷肉飯，排隊半小時值得！"

echo ""
echo "=== 4. 回覆貼文 ==="
SMS4_DB="$DB" $SMS4 post reply 1 2 "陽明山現在花況如何？想找時間去走走"
SMS4_DB="$DB" $SMS4 post reply 1 3 "求推薦登山路線！"
SMS4_DB="$DB" $SMS4 post reply 1 5 "建議清晨去，人比較少"
SMS4_DB="$DB" $SMS4 post reply 3 1 "我也在學 Rust，真的很有挑戰性"
SMS4_DB="$DB" $SMS4 post reply 3 3 "在哪裡？想吃！有地址嗎？"
SMS4_DB="$DB" $SMS4 post reply 3 5 "改天一起去吃！"
SMS4_DB="$DB" $SMS4 post reply 7 10 "好美的描述，讓我也想去京都了"
SMS4_DB="$DB" $SMS4 post reply 10 1 "京都是四季都適合去的地方！"
SMS4_DB="$DB" $SMS4 post reply 11 1 "我也很喜歡這本！"
SMS4_DB="$DB" $SMS4 post reply 12 7 "黃金獵犬的笑容真的無敵"

echo ""
echo "=== 5. 建立追蹤關係 ==="
# alice 追蹤 bob, carol, eve, jack, kate
SMS4_DB="$DB" $SMS4 follow add 1 2
SMS4_DB="$DB" $SMS4 follow add 1 3
SMS4_DB="$DB" $SMS4 follow add 1 5
SMS4_DB="$DB" $SMS4 follow add 1 10
SMS4_DB="$DB" $SMS4 follow add 1 11
# bob 追蹤 alice, carol, dave, frank, nick
SMS4_DB="$DB" $SMS4 follow add 2 1
SMS4_DB="$DB" $SMS4 follow add 2 3
SMS4_DB="$DB" $SMS4 follow add 2 4
SMS4_DB="$DB" $SMS4 follow add 2 6
SMS4_DB="$DB" $SMS4 follow add 2 14
# carol 追蹤 alice, bob, grace, iris, may
SMS4_DB="$DB" $SMS4 follow add 3 1
SMS4_DB="$DB" $SMS4 follow add 3 2
SMS4_DB="$DB" $SMS4 follow add 3 7
SMS4_DB="$DB" $SMS4 follow add 3 9
SMS4_DB="$DB" $SMS4 follow add 3 13
# 其他人隨機追蹤
SMS4_DB="$DB" $SMS4 follow add 4 1
SMS4_DB="$DB" $SMS4 follow add 4 2
SMS4_DB="$DB" $SMS4 follow add 5 1
SMS4_DB="$DB" $SMS4 follow add 5 3
SMS4_DB="$DB" $SMS4 follow add 6 2
SMS4_DB="$DB" $SMS4 follow add 6 14
SMS4_DB="$DB" $SMS4 follow add 7 1
SMS4_DB="$DB" $SMS4 follow add 8 1
SMS4_DB="$DB" $SMS4 follow add 8 2
SMS4_DB="$DB" $SMS4 follow add 9 3
SMS4_DB="$DB" $SMS4 follow add 10 1
SMS4_DB="$DB" $SMS4 follow add 10 2
SMS4_DB="$DB" $SMS4 follow add 11 1
SMS4_DB="$DB" $SMS4 follow add 11 10
SMS4_DB="$DB" $SMS4 follow add 12 1
SMS4_DB="$DB" $SMS4 follow add 13 7
SMS4_DB="$DB" $SMS4 follow add 14 2
SMS4_DB="$DB" $SMS4 follow add 14 1
SMS4_DB="$DB" $SMS4 follow add 15 1

echo ""
echo "=== 6. 按讚 ==="
# 貼文 1 (愛麗絲的陽明山) 獲得 5 個讚
SMS4_DB="$DB" $SMS4 like add 2 1
SMS4_DB="$DB" $SMS4 like add 3 1
SMS4_DB="$DB" $SMS4 like add 5 1
SMS4_DB="$DB" $SMS4 like add 10 1
SMS4_DB="$DB" $SMS4 like add 11 1

# 貼文 3 (bob 的 Rust) 獲得 4 個讚
SMS4_DB="$DB" $SMS4 like add 1 3
SMS4_DB="$DB" $SMS4 like add 3 3
SMS4_DB="$DB" $SMS4 like add 14 3
SMS4_DB="$DB" $SMS4 like add 6 3

# 貼文 5 (carol 的甜點) 獲得 3 個讚
SMS4_DB="$DB" $SMS4 like add 1 5
SMS4_DB="$DB" $SMS4 like add 2 5
SMS4_DB="$DB" $SMS4 like add 9 5

# 貼文 7 (dave 的鋼琴曲) 獲得 3 個讚
SMS4_DB="$DB" $SMS4 like add 1 7
SMS4_DB="$DB" $SMS4 like add 2 7
SMS4_DB="$DB" $SMS4 like add 10 7

echo ""
echo "=== 7. 交友資料 (15 筆) ==="
SMS4_DB="$DB" $SMS4 profile set 1  --birthday 1995-03-15 --gender male   --city 台北 --occupation 工程師     --height 175 --looking-for friend --about-me "喜歡爬山和攝影，假日都在戶外"
SMS4_DB="$DB" $SMS4 profile set 2  --birthday 1993-07-20 --gender male   --city 新竹 --occupation 程式設計師   --height 180 --looking-for any   --about-me "咖啡成癮者，Rust 愛好者"
SMS4_DB="$DB" $SMS4 profile set 3  --birthday 1998-12-01 --gender female --city 台北 --occupation 美食部落客   --height 162 --looking-for friend --about-me "吃遍全台美食，喜歡自煮"
SMS4_DB="$DB" $SMS4 profile set 4  --birthday 1990-05-10 --gender male   --city 台中 --occupation 音樂創作人   --height 178 --looking-for date  --about-me "鋼琴和吉他，偶爾寫寫歌"
SMS4_DB="$DB" $SMS4 profile set 5  --birthday 1996-09-03 --gender female --city 高雄 --occupation 瑜伽老師     --height 168 --looking-for friend --about-me "身心靈平衡，推廣正念生活"
SMS4_DB="$DB" $SMS4 profile set 6  --birthday 1988-11-18 --gender male   --city 台北 --occupation 金融分析師   --height 172 --looking-for any   --about-me "價值投資者，喜歡閱讀財報"
SMS4_DB="$DB" $SMS4 profile set 7  --birthday 1997-04-22 --gender female --city 台中 --occupation 插畫家       --height 160 --looking-for friend --about-me "水彩和數位插畫，貓奴一枚"
SMS4_DB="$DB" $SMS4 profile set 8  --birthday 1994-08-14 --gender male   --city 新北 --occupation 健身教練     --height 185 --looking-for any   --about-me "重訓和戶外運動，歡迎交流"
SMS4_DB="$DB" $SMS4 profile set 9  --birthday 1999-02-28 --gender female --city 台北 --occupation 美妝顧問     --height 165 --looking-for date  --about-me "彩妝教學和保養分享"
SMS4_DB="$DB" $SMS4 profile set 10 --birthday 1992-06-07 --gender male   --city 花蓮 --occupation 旅遊作家     --height 176 --looking-for friend --about-me "走遍世界各地，記錄旅途故事"
SMS4_DB="$DB" $SMS4 profile set 11 --birthday 1991-10-30 --gender female --city 台南 --occupation 書評作家     --height 163 --looking-for friend --about-me "一個月讀十本書，寫讀書筆記"
SMS4_DB="$DB" $SMS4 profile set 12 --birthday 2000-01-05 --gender male   --city 屏東 --occupation 寵物攝影師   --height 170 --looking-for any   --about-me "專拍貓狗，動物攝影師"
SMS4_DB="$DB" $SMS4 profile set 13 --birthday 1997-07-15 --gender female --city 台北 --occupation 手作設計師   --height 158 --looking-for friend --about-me "皮革和布藝手作，熱愛 DIY"
SMS4_DB="$DB" $SMS4 profile set 14 --birthday 1994-03-22 --gender male   --city 新竹 --occupation 科技產品經理 --height 177 --looking-for date  --about-me "AI 和區塊鏈，科技趨勢觀察"
SMS4_DB="$DB" $SMS4 profile set 15 --birthday 1996-12-09 --gender female --city 台北 --occupation 環保倡議者   --height 166 --looking-for friend --about-me "零浪費生活實踐者，淨灘志工"

echo ""
echo "=== 8. 興趣標籤 ==="
SMS4_DB="$DB" $SMS4 interest add 1 爬山
SMS4_DB="$DB" $SMS4 interest add 1 攝影
SMS4_DB="$DB" $SMS4 interest add 2 咖啡
SMS4_DB="$DB" $SMS4 interest add 2 程式
SMS4_DB="$DB" $SMS4 interest add 3 美食
SMS4_DB="$DB" $SMS4 interest add 3 烹飪
SMS4_DB="$DB" $SMS4 interest add 4 音樂
SMS4_DB="$DB" $SMS4 interest add 4 鋼琴
SMS4_DB="$DB" $SMS4 interest add 5 瑜珈
SMS4_DB="$DB" $SMS4 interest add 5 冥想
SMS4_DB="$DB" $SMS4 interest add 6 投資
SMS4_DB="$DB" $SMS4 interest add 6 閱讀
SMS4_DB="$DB" $SMS4 interest add 7 插畫
SMS4_DB="$DB" $SMS4 interest add 7 貓
SMS4_DB="$DB" $SMS4 interest add 8 健身
SMS4_DB="$DB" $SMS4 interest add 8 跑步
SMS4_DB="$DB" $SMS4 interest add 9 美妝
SMS4_DB="$DB" $SMS4 interest add 9 時尚
SMS4_DB="$DB" $SMS4 interest add 10 旅行
SMS4_DB="$DB" $SMS4 interest add 10 攝影
SMS4_DB="$DB" $SMS4 interest add 11 閱讀
SMS4_DB="$DB" $SMS4 interest add 11 寫作
SMS4_DB="$DB" $SMS4 interest add 12 寵物
SMS4_DB="$DB" $SMS4 interest add 12 攝影
SMS4_DB="$DB" $SMS4 interest add 13 手作
SMS4_DB="$DB" $SMS4 interest add 13 設計
SMS4_DB="$DB" $SMS4 interest add 14 科技
SMS4_DB="$DB" $SMS4 interest add 14 AI
SMS4_DB="$DB" $SMS4 interest add 15 環保
SMS4_DB="$DB" $SMS4 interest add 15 志工

echo ""
echo "=== 假資料填入完成 ==="
echo ""
echo "  使用者: 15 筆"
echo "  貼文:   30 筆（含 10 則回覆）"
echo "  追蹤:   35 筆"
echo "  按讚:   15 筆"
echo "  交友資料: 15 筆"
echo "  興趣標籤: 30 筆"
echo ""
echo "啟動互動："
echo "  SMS4_DB=$DB cargo run -- post timeline 1"
echo "  SMS4_DB=$DB cargo run -- post get 1"
echo "  SMS4_DB=$DB cargo run -- profile search --city 台北"
echo "  SMS4_DB=$DB cargo run -- profile search --tags 攝影"
