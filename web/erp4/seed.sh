#!/usr/bin/env bash
# seed.sh — 填入大量測試假資料
set -euo pipefail

ERP4=${ERP4:-cargo run --}
DB="erp4-dev.db"

rm $DB

echo "=== 1. 初始化資料庫 ==="
ERP4_DB="$DB" $ERP4 init

echo ""
echo "=== 2. 建立客戶 (20 筆) ==="
ERP4_DB="$DB" $ERP4 customer add "台北科技公司"       --email info@taipei-tech.tw       --phone 02-25551111 --address "台北市信義區信義路五段7號"
ERP4_DB="$DB" $ERP4 customer add "新竹光電股份"       --email contact@hsinchu-oe.com   --phone 03-5777777  --address "新竹市東區科學園區力行路1號"
ERP4_DB="$DB" $ERP4 customer add "台南紡織工業"       --email service@tainan-textile.tw --phone 06-2688888  --address "台南市東區中華東路三段399號"
ERP4_DB="$DB" $ERP4 customer add "高雄國際物流"       --email info@kh-logistics.tw      --phone 07-3388888  --address "高雄市前鎮區成功二路4號"
ERP4_DB="$DB" $ERP4 customer add "台中精密機械"       --email sales@taichung-machine.tw --phone 04-23599999 --address "台中市西屯區台灣大道四段600號"
ERP4_DB="$DB" $ERP4 customer add "桃園半導體"         --email contact@taoyuan-semi.tw   --phone 03-1234567  --address "桃園市龜山區文化一路88號"
ERP4_DB="$DB" $ERP4 customer add "彰化食品企業"       --email order@changhua-food.tw    --phone 04-7654321  --address "彰化縣彰化市中山路二段100號"
ERP4_DB="$DB" $ERP4 customer add "雲林農業科技"       --email info@yunlin-agri.tw       --phone 05-5333333  --address "雲林縣斗六市大學路三段123號"
ERP4_DB="$DB" $ERP4 customer add "花蓮觀光飯店"       --email booking@hualien-hotel.tw  --phone 03-8889999  --address "花蓮縣花蓮市中美路111號"
ERP4_DB="$DB" $ERP4 customer add "基隆港務公司"       --email service@keelung-port.tw   --phone 02-24201111 --address "基隆市中正區中正路1號"
ERP4_DB="$DB" $ERP4 customer add "新北零售百貨"       --email info@ntpc-retail.tw        --phone 02-29603333 --address "新北市板橋區中山路一段161號"
ERP4_DB="$DB" $ERP4 customer add "宜蘭民宿聯盟"       --email contact@yilan-bnb.tw       --phone 03-9654321  --address "宜蘭縣羅東鎮公正路200號"
ERP4_DB="$DB" $ERP4 customer add "苗栗陶瓷工業"       --email sales@miaoli-ceramic.tw    --phone 03-7222333  --address "苗栗縣公館鄉玉谷村121號"
ERP4_DB="$DB" $ERP4 customer add "南投茶葉合作社"     --email order@nantou-tea.tw        --phone 04-92981234 --address "南投縣鹿谷鄉鹿谷村中正路300號"
ERP4_DB="$DB" $ERP4 customer add "屏東水產養殖"       --email info@pingtung-aqua.tw      --phone 08-8777666  --address "屏東縣東港鎮新生路50號"
ERP4_DB="$DB" $ERP4 customer add "台東有機農場"       --email farm@taitung-organic.tw    --phone 08-9222333  --address "台東縣台東市更生北路500號"
ERP4_DB="$DB" $ERP4 customer add "澎湖海鮮貿易"       --email export@penghu-seafood.tw   --phone 06-9268888  --address "澎湖縣馬公市臨海路36號"
ERP4_DB="$DB" $ERP4 customer add "金門高粱酒廠"       --email info@kinmen-kaoliang.tw    --phone 08-2328888  --address "金門縣金寧鄉桃園路1號"
ERP4_DB="$DB" $ERP4 customer add "連江航運公司"       --email service@matsu-shipping.tw  --phone 08-3625566  --address "連江縣南竿鄉馬祖村101號"
ERP4_DB="$DB" $ERP4 customer add "內湖軟體園區"       --email info@neihu-soft.tw          --phone 02-26551111 --address "台北市內湖區瑞光路333號"

echo ""
echo "=== 3. 建立產品 (30 筆) ==="
ERP4_DB="$DB" $ERP4 product add "筆記型電腦 Pro"       "NB-PRO-001"  38800 --stock 50  --description "14吋 i7 16G 512G SSD"
ERP4_DB="$DB" $ERP4 product add "無線滑鼠"              "MS-WL-002"   890   --stock 200 --description "2.4GHz 人體工學"
ERP4_DB="$DB" $ERP4 product add "機械鍵盤"              "KB-MECH-003" 2490  --stock 80  --description "青軸 104鍵 RGB"
ERP4_DB="$DB" $ERP4 product add "27吋顯示器"            "MON-27-004"  12800 --stock 30  --description "4K IPS HDR"
ERP4_DB="$DB" $ERP4 product add "USB-C 集線器"          "HUB-USBC-005" 1200 --stock 150 --description "7合1 Type-C"
ERP4_DB="$DB" $ERP4 product add "外接硬碟 2TB"          "HDD-2TB-006" 2800  --stock 60  --description "USB 3.2 2.5吋"
ERP4_DB="$DB" $ERP4 product add "SSD 1TB"               "SSD-1TB-007" 3990  --stock 100 --description "NVMe M.2 PCIe 4.0"
ERP4_DB="$DB" $ERP4 product add "網路攝影機"            "CAM-WEB-008" 1990  --stock 45  --description "1080P 自動對焦"
ERP4_DB="$DB" $ERP4 product add "藍牙耳機"              "BT-EAR-009"  3590  --stock 75  --description "降噪 30小時續航"
ERP4_DB="$DB" $ERP4 product add "不斷電系統 UPS"        "UPS-1000-010" 5200 --stock 5   --description "1000VA 在線互動式"
ERP4_DB="$DB" $ERP4 product add "雷射印表機"            "PRT-LSR-011" 8900  --stock 12  --description "黑白 A4 自動雙面"
ERP4_DB="$DB" $ERP4 product add "無線路由器 WiFi 6"     "RT-WIFI6-012" 3200 --stock 40  --description "AX5400 Mesh"
ERP4_DB="$DB" $ERP4 product add "NAS 2 Bay"             "NAS-2B-013"  6500  --stock 8   --description "2Bay 磁碟陣列"
ERP4_DB="$DB" $ERP4 product add "USB 隨身碟 128G"      "USB-128-014" 499   --stock 300 --description "USB 3.2 金屬殼"
ERP4_DB="$DB" $ERP4 product add "HDMI 線 2M"            "HDMI-2M-015" 299   --stock 500 --description "2.1版 48Gbps"
ERP4_DB="$DB" $ERP4 product add "擴充座 Thunderbolt 4"  "DOCK-TB4-016" 8900 --stock 15  --description "Thunderbolt 4 擴充座"
ERP4_DB="$DB" $ERP4 product add "視訊會議系統"           "VC-SYS-017"  35000 --stock 3   --description "4K 廣角 麥克風陣列"
ERP4_DB="$DB" $ERP4 product add "指紋辨識器 USB"        "FP-USB-018"  1590  --stock 25  --description "Windows Hello"
ERP4_DB="$DB" $ERP4 product add "筆記型電腦 Lite"       "NB-LITE-019" 21800 --stock 35  --description "14吋 i5 8G 256G"
ERP4_DB="$DB" $ERP4 product add "平板電腦"              "TAB-10-020"  14900 --stock 20  --description "10.9吋 64G WiFi"
ERP4_DB="$DB" $ERP4 product add "智慧手錶"              "WATCH-021"   12900 --stock 18  --description "GPS + 心率 + 血氧"
ERP4_DB="$DB" $ERP4 product add "無線充電板"            "CHG-WL-022"  890   --stock 90  --description "15W 三合一"
ERP4_DB="$DB" $ERP4 product add "筆電背包"              "BAG-NB-023"  1980  --stock 60  --description "防潑水 17吋"
ERP4_DB="$DB" $ERP4 product add "立架螢幕支架"          "ARM-MON-024" 1680  --stock 40  --description "氣壓式 32吋適用"
ERP4_DB="$DB" $ERP4 product add "KVM 切換器"            "KVM-2P-025"  2200  --stock 15  --description "2埠 HDMI 4K"
ERP4_DB="$DB" $ERP4 product add "不斷電系統 UPS 2000"   "UPS-2000-026" 9800 --stock 3   --description "2000VA 機架式"
ERP4_DB="$DB" $ERP4 product add "網路交換器 24埠"       "SW-24G-027"  5800  --stock 10  --description "Gigabit 網管型"
ERP4_DB="$DB" $ERP4 product add "SSD 外接盒"            "SSD-ENCL-028" 890  --stock 60  --description "USB 3.2 NVMe"
ERP4_DB="$DB" $ERP4 product add "手寫繪圖板"            "DRAW-TAB-029" 3200 --stock 12  --description "10吋 無線"
ERP4_DB="$DB" $ERP4 product add "會議麥克風"            "MIC-CONF-030" 4500 --stock 7   --description "全向式 USB-C"

echo ""
echo "=== 4. 建立供應商 (10 筆) ==="
ERP4_DB="$DB" $ERP4 supplier add "宏碁供應鏈"          --contact-person "陳協理" --email acer_sales@example.com    --phone 02-22345678
ERP4_DB="$DB" $ERP4 supplier add "聯強國際"            --contact-person "吳經理" --email service@syntex.com       --phone 02-25112233
ERP4_DB="$DB" $ERP4 supplier add "台灣積體電路"        --contact-person "蔡副總" --email support@tsmc.com         --phone 03-5631234
ERP4_DB="$DB" $ERP4 supplier add "鴻海精密"            --contact-person "劉課長" --email purchase@honhai.com      --phone 02-22683456
ERP4_DB="$DB" $ERP4 supplier add "廣達電腦"            --contact-person "林專員" --email sales@quantac.com        --phone 03-3287890
ERP4_DB="$DB" $ERP4 supplier add "華碩電腦"            --contact-person "許副理" --email order@asus.com           --phone 02-28943456
ERP4_DB="$DB" $ERP4 supplier add "三星電子台灣"        --contact-person "金代表" --email tw_sales@samsung.com     --phone 02-26567890
ERP4_DB="$DB" $ERP4 supplier add "美光科技"            --contact-person "黃經理" --email tw_support@micron.com    --phone 03-5671234
ERP4_DB="$DB" $ERP4 supplier add "Western Digital"     --contact-person "謝專員" --email hdd_sales@wd.com         --phone 02-27123456
ERP4_DB="$DB" $ERP4 supplier add "羅技電子"            --contact-person "鄭課長" --email logi_tw@logitech.com     --phone 02-27773333

echo ""
echo "=== 5. 建立訂單 (25 筆) ==="

# 訂單 1 — 台北科技，已確認
ERP4_DB="$DB" $ERP4 order create 1 --notes "急件，請優先處理"
ERP4_DB="$DB" $ERP4 order add-item 1 1 2
ERP4_DB="$DB" $ERP4 order add-item 1 2 10
ERP4_DB="$DB" $ERP4 order add-item 1 5 5
ERP4_DB="$DB" $ERP4 order update-status 1 confirmed

# 訂單 2 — 新竹光電，已出貨
ERP4_DB="$DB" $ERP4 order create 2 --notes "請附發票"
ERP4_DB="$DB" $ERP4 order add-item 2 4 3
ERP4_DB="$DB" $ERP4 order add-item 2 7 5
ERP4_DB="$DB" $ERP4 order add-item 2 9 2
ERP4_DB="$DB" $ERP4 order update-status 2 confirmed
ERP4_DB="$DB" $ERP4 order update-status 2 shipped

# 訂單 3 — 台南紡織，已送達
ERP4_DB="$DB" $ERP4 order create 3
ERP4_DB="$DB" $ERP4 order add-item 3 3 8
ERP4_DB="$DB" $ERP4 order add-item 3 6 2
ERP4_DB="$DB" $ERP4 order update-status 3 confirmed
ERP4_DB="$DB" $ERP4 order update-status 3 shipped
ERP4_DB="$DB" $ERP4 order update-status 3 delivered

# 訂單 4 — 高雄物流，待處理
ERP4_DB="$DB" $ERP4 order create 4
ERP4_DB="$DB" $ERP4 order add-item 4 10 1
ERP4_DB="$DB" $ERP4 order add-item 4 16 1

# 訂單 5 — 台中精密，已取消
ERP4_DB="$DB" $ERP4 order create 5
ERP4_DB="$DB" $ERP4 order add-item 5 8 3
ERP4_DB="$DB" $ERP4 order update-status 5 cancelled

# 訂單 6 — 桃園半導體，已確認
ERP4_DB="$DB" $ERP4 order create 6
ERP4_DB="$DB" $ERP4 order add-item 6 7 20
ERP4_DB="$DB" $ERP4 order add-item 6 13 2
ERP4_DB="$DB" $ERP4 order update-status 6 confirmed

# 訂單 7 — 彰化食品，已送達
ERP4_DB="$DB" $ERP4 order create 7 --notes "月結客戶"
ERP4_DB="$DB" $ERP4 order add-item 7 14 50
ERP4_DB="$DB" $ERP4 order add-item 7 15 100
ERP4_DB="$DB" $ERP4 order add-item 7 22 10
ERP4_DB="$DB" $ERP4 order update-status 7 confirmed
ERP4_DB="$DB" $ERP4 order update-status 7 shipped
ERP4_DB="$DB" $ERP4 order update-status 7 delivered

# 訂單 8 — 雲林農業，待處理
ERP4_DB="$DB" $ERP4 order create 8
ERP4_DB="$DB" $ERP4 order add-item 8 19 3
ERP4_DB="$DB" $ERP4 order add-item 8 23 5

# 訂單 9 — 花蓮飯店，已出貨
ERP4_DB="$DB" $ERP4 order create 9
ERP4_DB="$DB" $ERP4 order add-item 9 4 5
ERP4_DB="$DB" $ERP4 order add-item 9 20 2
ERP4_DB="$DB" $ERP4 order add-item 9 24 8
ERP4_DB="$DB" $ERP4 order update-status 9 confirmed
ERP4_DB="$DB" $ERP4 order update-status 9 shipped

# 訂單 10 — 基隆港務，已取消
ERP4_DB="$DB" $ERP4 order create 10
ERP4_DB="$DB" $ERP4 order add-item 10 27 3
ERP4_DB="$DB" $ERP4 order update-status 10 cancelled

# 訂單 11 — 新北百貨，待處理
ERP4_DB="$DB" $ERP4 order create 11 --notes "分批出貨"
ERP4_DB="$DB" $ERP4 order add-item 11 14 100
ERP4_DB="$DB" $ERP4 order add-item 11 15 200
ERP4_DB="$DB" $ERP4 order add-item 11 22 30

# 訂單 12 — 宜蘭民宿，已確認
ERP4_DB="$DB" $ERP4 order create 12
ERP4_DB="$DB" $ERP4 order add-item 12 9 5
ERP4_DB="$DB" $ERP4 order add-item 12 22 10
ERP4_DB="$DB" $ERP4 order add-item 12 28 5
ERP4_DB="$DB" $ERP4 order update-status 12 confirmed

# 訂單 13 — 苗栗陶瓷，已送達
ERP4_DB="$DB" $ERP4 order create 13
ERP4_DB="$DB" $ERP4 order add-item 13 3 10
ERP4_DB="$DB" $ERP4 order add-item 13 5 5
ERP4_DB="$DB" $ERP4 order update-status 13 confirmed
ERP4_DB="$DB" $ERP4 order update-status 13 shipped
ERP4_DB="$DB" $ERP4 order update-status 13 delivered

# 訂單 14 — 南投茶葉，待處理
ERP4_DB="$DB" $ERP4 order create 14
ERP4_DB="$DB" $ERP4 order add-item 14 8 2
ERP4_DB="$DB" $ERP4 order add-item 14 19 1

# 訂單 15 — 屏東水產，已出貨
ERP4_DB="$DB" $ERP4 order create 15 --notes "冷鏈配送"
ERP4_DB="$DB" $ERP4 order add-item 15 10 2
ERP4_DB="$DB" $ERP4 order add-item 15 26 1
ERP4_DB="$DB" $ERP4 order update-status 15 confirmed
ERP4_DB="$DB" $ERP4 order update-status 15 shipped

# 訂單 16 — 台東有機，已確認
ERP4_DB="$DB" $ERP4 order create 16
ERP4_DB="$DB" $ERP4 order add-item 16 2 20
ERP4_DB="$DB" $ERP4 order add-item 16 12 2
ERP4_DB="$DB" $ERP4 order update-status 16 confirmed

# 訂單 17 — 澎湖海鮮，待處理
ERP4_DB="$DB" $ERP4 order create 17
ERP4_DB="$DB" $ERP4 order add-item 17 1 1
ERP4_DB="$DB" $ERP4 order add-item 17 18 2

# 訂單 18 — 金門酒廠，已送達
ERP4_DB="$DB" $ERP4 order create 18 --notes "貨到付款"
ERP4_DB="$DB" $ERP4 order add-item 18 13 1
ERP4_DB="$DB" $ERP4 order add-item 18 16 2
ERP4_DB="$DB" $ERP4 order add-item 18 27 1
ERP4_DB="$DB" $ERP4 order update-status 18 confirmed
ERP4_DB="$DB" $ERP4 order update-status 18 shipped
ERP4_DB="$DB" $ERP4 order update-status 18 delivered

# 訂單 19 — 連江航運，已取消
ERP4_DB="$DB" $ERP4 order create 19
ERP4_DB="$DB" $ERP4 order add-item 19 20 3
ERP4_DB="$DB" $ERP4 order update-status 19 cancelled

# 訂單 20 — 內湖軟體，待處理
ERP4_DB="$DB" $ERP4 order create 20
ERP4_DB="$DB" $ERP4 order add-item 20 1 5
ERP4_DB="$DB" $ERP4 order add-item 20 7 10
ERP4_DB="$DB" $ERP4 order add-item 20 19 5
ERP4_DB="$DB" $ERP4 order add-item 20 30 2

# 訂單 21 — 台北科技，已送達
ERP4_DB="$DB" $ERP4 order create 1 --notes "第二批訂單"
ERP4_DB="$DB" $ERP4 order add-item 21 11 1
ERP4_DB="$DB" $ERP4 order add-item 21 17 1
ERP4_DB="$DB" $ERP4 order update-status 21 confirmed
ERP4_DB="$DB" $ERP4 order update-status 21 shipped
ERP4_DB="$DB" $ERP4 order update-status 21 delivered

# 訂單 22 — 新竹光電，待處理
ERP4_DB="$DB" $ERP4 order create 2
ERP4_DB="$DB" $ERP4 order add-item 22 4 2
ERP4_DB="$DB" $ERP4 order add-item 22 6 5

# 訂單 23 — 高雄物流，已確認
ERP4_DB="$DB" $ERP4 order create 4 --notes "補貨訂單"
ERP4_DB="$DB" $ERP4 order add-item 23 30 3
ERP4_DB="$DB" $ERP4 order update-status 23 confirmed

# 訂單 24 — 內湖軟體，已出貨
ERP4_DB="$DB" $ERP4 order create 20
ERP4_DB="$DB" $ERP4 order add-item 24 12 5
ERP4_DB="$DB" $ERP4 order add-item 24 25 2
ERP4_DB="$DB" $ERP4 order add-item 24 29 3
ERP4_DB="$DB" $ERP4 order update-status 24 confirmed
ERP4_DB="$DB" $ERP4 order update-status 24 shipped

# 訂單 25 — 彰化食品，待處理
ERP4_DB="$DB" $ERP4 order create 7
ERP4_DB="$DB" $ERP4 order add-item 25 14 200
ERP4_DB="$DB" $ERP4 order add-item 25 15 500

echo ""
echo "=== 6. 建立採購單 (12 筆) ==="

# PO 1 — 供應商 1 (宏碁)，已進貨
ERP4_DB="$DB" $ERP4 purchase-order create 1 --notes "每月定期補貨"
ERP4_DB="$DB" $ERP4 purchase-order add-item 1 2 100
ERP4_DB="$DB" $ERP4 purchase-order add-item 1 5 50
ERP4_DB="$DB" $ERP4 purchase-order update-status 1 approved
ERP4_DB="$DB" $ERP4 purchase-order update-status 1 received

# PO 2 — 供應商 2 (聯強)，待核准
ERP4_DB="$DB" $ERP4 purchase-order create 2
ERP4_DB="$DB" $ERP4 purchase-order add-item 2 1 10
ERP4_DB="$DB" $ERP4 purchase-order add-item 2 7 20

# PO 3 — 供應商 3 (台積電)，已核准
ERP4_DB="$DB" $ERP4 purchase-order create 3 --notes "客製晶片"
ERP4_DB="$DB" $ERP4 purchase-order add-item 3 3 30
ERP4_DB="$DB" $ERP4 purchase-order update-status 3 approved

# PO 4 — 供應商 4 (鴻海)，已進貨
ERP4_DB="$DB" $ERP4 purchase-order create 4
ERP4_DB="$DB" $ERP4 purchase-order add-item 4 1 5
ERP4_DB="$DB" $ERP4 purchase-order add-item 4 4 10
ERP4_DB="$DB" $ERP4 purchase-order update-status 4 approved
ERP4_DB="$DB" $ERP4 purchase-order update-status 4 received

# PO 5 — 供應商 5 (廣達)，待核准
ERP4_DB="$DB" $ERP4 purchase-order create 5
ERP4_DB="$DB" $ERP4 purchase-order add-item 5 19 15
ERP4_DB="$DB" $ERP4 purchase-order add-item 5 20 5

# PO 6 — 供應商 6 (華碩)，已進貨
ERP4_DB="$DB" $ERP4 purchase-order create 6 --notes "促銷活動備貨"
ERP4_DB="$DB" $ERP4 purchase-order add-item 6 4 20
ERP4_DB="$DB" $ERP4 purchase-order add-item 6 9 30
ERP4_DB="$DB" $ERP4 purchase-order update-status 6 approved
ERP4_DB="$DB" $ERP4 purchase-order update-status 6 received

# PO 7 — 供應商 7 (三星)，已核准
ERP4_DB="$DB" $ERP4 purchase-order create 7
ERP4_DB="$DB" $ERP4 purchase-order add-item 7 7 50
ERP4_DB="$DB" $ERP4 purchase-order add-item 7 20 10
ERP4_DB="$DB" $ERP4 purchase-order update-status 7 approved

# PO 8 — 供應商 8 (美光)，待核准
ERP4_DB="$DB" $ERP4 purchase-order create 8
ERP4_DB="$DB" $ERP4 purchase-order add-item 8 7 100
ERP4_DB="$DB" $ERP4 purchase-order add-item 8 28 30

# PO 9 — 供應商 9 (WD)，已進貨
ERP4_DB="$DB" $ERP4 purchase-order create 9
ERP4_DB="$DB" $ERP4 purchase-order add-item 9 6 40
ERP4_DB="$DB" $ERP4 purchase-order update-status 9 approved
ERP4_DB="$DB" $ERP4 purchase-order update-status 9 received

# PO 10 — 供應商 10 (羅技)，待核准
ERP4_DB="$DB" $ERP4 purchase-order create 10
ERP4_DB="$DB" $ERP4 purchase-order add-item 10 2 200
ERP4_DB="$DB" $ERP4 purchase-order add-item 10 8 50

# PO 11 — 供應商 2 (聯強)，已取消
ERP4_DB="$DB" $ERP4 purchase-order create 2 --notes "已取消訂單"
ERP4_DB="$DB" $ERP4 purchase-order add-item 11 11 5
ERP4_DB="$DB" $ERP4 purchase-order update-status 11 cancelled

# PO 12 — 供應商 1 (宏碁)，已核准
ERP4_DB="$DB" $ERP4 purchase-order create 1 --notes "季末補貨"
ERP4_DB="$DB" $ERP4 purchase-order add-item 12 1 10
ERP4_DB="$DB" $ERP4 purchase-order add-item 12 19 10
ERP4_DB="$DB" $ERP4 purchase-order add-item 12 23 20
ERP4_DB="$DB" $ERP4 purchase-order update-status 12 approved

echo ""
echo "=== 7. 建立發票 (20 筆) ==="

# 發票 1-3 對應訂單 1-3
ERP4_DB="$DB" $ERP4 invoice create --customer-id 1 --order-id 1 --due-date "2026-07-15" --amount 87380  --notes "台北科技訂單 #1"
ERP4_DB="$DB" $ERP4 invoice create --customer-id 2 --order-id 2 --due-date "2026-07-20" --amount 67350  --notes "新竹光電訂單 #2"
ERP4_DB="$DB" $ERP4 invoice create --customer-id 3 --order-id 3 --due-date "2026-06-01" --amount 25520  --notes "台南紡織訂單 #3"

# 發票 4-6 逾期未付
ERP4_DB="$DB" $ERP4 invoice create --customer-id 4 --order-id 4 --due-date "2026-04-15" --amount 42000  --notes "高雄物流逾期 #4"
ERP4_DB="$DB" $ERP4 invoice create --customer-id 8 --order-id 8 --due-date "2026-05-01" --amount 78900  --notes "雲林農業逾期 #8"
ERP4_DB="$DB" $ERP4 invoice create --customer-id 14 --order-id 14 --due-date "2026-05-20" --amount 21600 --notes "南投茶業逾期 #14"

# 發票 7-9 已付款
ERP4_DB="$DB" $ERP4 invoice create --customer-id 6 --order-id 6 --due-date "2026-06-30" --amount 89800 --notes "桃園半導體 #6"
ERP4_DB="$DB" $ERP4 invoice update-status 7 paid
ERP4_DB="$DB" $ERP4 invoice create --customer-id 7 --order-id 7 --due-date "2026-06-25" --amount 79850 --notes "彰化食品 #7"
ERP4_DB="$DB" $ERP4 invoice update-status 8 paid
ERP4_DB="$DB" $ERP4 invoice create --customer-id 12 --order-id 12 --due-date "2026-07-10" --amount 37900 --notes "宜蘭民宿 #12"
ERP4_DB="$DB" $ERP4 invoice update-status 9 paid

# 發票 10-12 待付款
ERP4_DB="$DB" $ERP4 invoice create --customer-id 11 --order-id 11 --due-date "2026-08-15" --amount 118600 --notes "新北百貨 #11"
ERP4_DB="$DB" $ERP4 invoice create --customer-id 16 --order-id 16 --due-date "2026-08-20" --amount 28800 --notes "台東有機 #16"
ERP4_DB="$DB" $ERP4 invoice create --customer-id 20 --order-id 20 --due-date "2026-09-01" --amount 263800 --notes "內湖軟體 #20"

# 發票 13-15 對應訂單 9, 13, 15
ERP4_DB="$DB" $ERP4 invoice create --customer-id 9 --order-id 9 --due-date "2026-07-05" --amount 81200 --notes "花蓮飯店 #9"
ERP4_DB="$DB" $ERP4 invoice create --customer-id 13 --order-id 13 --due-date "2026-07-30" --amount 30900 --notes "苗栗陶瓷 #13"
ERP4_DB="$DB" $ERP4 invoice create --customer-id 15 --order-id 15 --due-date "2026-08-05" --amount 20200 --notes "屏東水產 #15"

# 發票 16-17 已取消
ERP4_DB="$DB" $ERP4 invoice create --customer-id 5 --order-id 5 --due-date "2026-08-10" --amount 5970 --notes "台中精密已取消 #5"
ERP4_DB="$DB" $ERP4 invoice update-status 16 cancelled
ERP4_DB="$DB" $ERP4 invoice create --customer-id 10 --order-id 10 --due-date "2026-08-12" --amount 17400 --notes "基隆港務已取消 #10"
ERP4_DB="$DB" $ERP4 invoice update-status 17 cancelled

# 發票 18-20 逾期未付
ERP4_DB="$DB" $ERP4 invoice create --customer-id 17 --order-id 17 --due-date "2026-05-15" --amount 39800 --notes "澎湖海鮮逾期 #17"
ERP4_DB="$DB" $ERP4 invoice create --customer-id 18 --order-id 18 --due-date "2026-06-01" --amount 29800 --notes "金門酒廠逾期 #18"
ERP4_DB="$DB" $ERP4 invoice create --customer-id 1 --order-id 21 --due-date "2026-06-01" --amount 43900 --notes "台北科技逾期 #21"

echo ""
echo "=== 假資料填入完成 ==="
echo ""
echo "  客戶: 20 筆"
echo "  產品: 30 筆"
echo "  供應商: 10 筆"
echo "  訂單: 25 筆（含明細）"
echo "  採購單: 12 筆（含明細）"
echo "  發票: 20 筆"
echo ""
echo "啟動開發伺服器："
echo "  ERP4_DB=$DB cargo run -- web --dev"
