use bcrypt::{hash, DEFAULT_COST};
use rand::seq::SliceRandom;
use rand::Rng;
use uuid::Uuid;

#[derive(Debug)]
struct User {
    id: String,
    username: String,
    email: String,
    password_hash: String,
    role: String,
}

#[derive(Debug)]
struct Category {
    id: String,
    name: String,
    parent_id: Option<String>,
}

#[derive(Debug)]
struct Product {
    id: String,
    seller_id: String,
    category_id: String,
    name: String,
    description: String,
    price: f64,
    stock: i32,
    image_url: String,
    rating: f64,
    sold_count: i32,
}

struct TestData {
    users: Vec<User>,
    categories: Vec<Category>,
    products: Vec<Product>,
}

type ProductTemplate = (&'static str, &'static str, &'static str, f64);
type CategoryImageMap = std::collections::HashMap<&'static str, Vec<&'static str>>;

fn get_category_images() -> CategoryImageMap {
    let mut map = std::collections::HashMap::new();
    map.insert("cat-phones", vec![
        "https://images.unsplash.com/photo-1511707171634-5f897ff02aa9?w=400",
        "https://images.unsplash.com/photo-1512054502232-10a0a035d672?w=400",
        "https://images.unsplash.com/photo-1544244015-0df4b3ffc6b0?w=400",
        "https://images.unsplash.com/photo-1598327105666-5b89351aff70?w=400",
        "https://images.unsplash.com/photo-1605236453806-6ff36851218e?w=400",
    ]);
    map.insert("cat-laptops", vec![
        "https://images.unsplash.com/photo-1496181133206-80ce9b88a853?w=400",
        "https://images.unsplash.com/photo-1525547719571-a2d4ac8945e2?w=400",
        "https://images.unsplash.com/photo-1517336714731-489689fd1ca8?w=400",
        "https://images.unsplash.com/photo-1611186871348-b1ce696e52c9?w=400",
        "https://images.unsplash.com/photo-1541807084-5c52b6b3adef?w=400",
    ]);
    map.insert("cat-accessories", vec![
        "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=400",
        "https://images.unsplash.com/photo-1585386959984-a4155224a1ad?w=400",
        "https://images.unsplash.com/photo-1572569511254-d8f925b5f5e2?w=400",
        "https://images.unsplash.com/photo-1606220588913-b3aacb4d2f46?w=400",
        "https://images.unsplash.com/photo-1590658268037-6bf12f032f55?w=400",
    ]);
    map.insert("cat-mens", vec![
        "https://images.unsplash.com/photo-1576566588028-4147f3842f27?w=400",
        "https://images.unsplash.com/photo-1618354691373-d851c5c3a990?w=400",
        "https://images.unsplash.com/photo-1594938298603-c8148c4dae35?w=400",
        "https://images.unsplash.com/photo-1591047139829-d91aecb6caea?w=400",
        "https://images.unsplash.com/photo-1552374196-1ab2a1c593e8?w=400",
    ]);
    map.insert("cat-womens", vec![
        "https://images.unsplash.com/photo-1434389677669-e08b4cac3105?w=400",
        "https://images.unsplash.com/photo-1525507119028-ed4c629a60a3?w=400",
        "https://images.unsplash.com/photo-1515886657613-9f3515b0c78f?w=400",
        "https://images.unsplash.com/photo-1594938298603-c8148c4dae35?w=400",
        "https://images.unsplash.com/photo-1483985988355-763728e1935b?w=400",
    ]);
    map.insert("cat-sneakers", vec![
        "https://images.unsplash.com/photo-1542291026-7eec264c27ff?w=400",
        "https://images.unsplash.com/photo-1460353581641-37baddab0fa2?w=400",
        "https://images.unsplash.com/photo-1553062407-98eeb64c6a62?w=400",
        "https://images.unsplash.com/photo-1549298916-b41d501d3772?w=400",
        "https://images.unsplash.com/photo-1600185365926-3a2ce3cdb9eb?w=400",
    ]);
    map.insert("cat-furniture", vec![
        "https://images.unsplash.com/photo-1555041469-a586c61ea9bc?w=400",
        "https://images.unsplash.com/photo-1506439773649-6e0eb8cfb237?w=400",
        "https://images.unsplash.com/photo-1524758631624-e2822e304c36?w=400",
        "https://images.unsplash.com/photo-1550226891-ef816aed4a98?w=400",
        "https://images.unsplash.com/photo-1493663284031-b7e3aefcae8e?w=400",
    ]);
    map.insert("cat-kitchen", vec![
        "https://images.unsplash.com/photo-1556909114-f6e7ad7d3136?w=400",
        "https://images.unsplash.com/photo-1571175443880-49e1d25b2bc5?w=400",
        "https://images.unsplash.com/photo-1585659722983-3a675dabf23d?w=400",
        "https://images.unsplash.com/photo-1556909114-44e3e9699e2b?w=400",
        "https://images.unsplash.com/photo-1565538810643-b5bdb714032a?w=400",
    ]);
    map.insert("cat-skincare", vec![
        "https://images.unsplash.com/photo-1556228720-195a672e8a03?w=400",
        "https://images.unsplash.com/photo-1570194065650-d99fb4b8ccb0?w=400",
        "https://images.unsplash.com/photo-1596462502278-27bfdc403348?w=400",
        "https://images.unsplash.com/photo-1556228578-0d85b1a4d571?w=400",
        "https://images.unsplash.com/photo-1620916566398-39f1143ab7be?w=400",
    ]);
    map.insert("cat-makeup", vec![
        "https://images.unsplash.com/photo-1512496015851-a90fb38ba796?w=400",
        "https://images.unsplash.com/photo-1586495777744-4e6232bf2171?w=400",
        "https://images.unsplash.com/photo-1631730486572-226d1f595b68?w=400",
        "https://images.unsplash.com/photo-1599733594230-6b823276abcc?w=400",
        "https://images.unsplash.com/photo-1522335789203-aabd1fc54bc9?w=400",
    ]);
    map.insert("cat-snacks", vec![
        "https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=400",
        "https://images.unsplash.com/photo-1558961363-fa8fdf82db35?w=400",
        "https://images.unsplash.com/photo-1607082348824-0a96f2a4b9da?w=400",
        "https://images.unsplash.com/photo-1621939514649-280e2ee25f60?w=400",
        "https://images.unsplash.com/photo-1563729784474-d77dbb933a9e?w=400",
    ]);
    map.insert("cat-drinks", vec![
        "https://images.unsplash.com/photo-1544145945-f90425340c7e?w=400",
        "https://images.unsplash.com/photo-1551538827-9c037cb4f32a?w=400",
        "https://images.unsplash.com/photo-1534353473418-4cfa6c56fd38?w=400",
        "https://images.unsplash.com/photo-1560023907-5f339617ea30?w=400",
        "https://images.unsplash.com/photo-1600271886742-f049cd451bba?w=400",
    ]);
    map
}

fn generate_users() -> Vec<User> {
    let names = vec![
        ("王小明", "wangxiaoming"),
        ("李小華", "lixiaohua"),
        ("陳大頭", "chendatou"),
        ("張美麗", "zhangmeili"),
        ("林志偉", "linzhiwei"),
        ("黃小倩", "huangxiaoqian"),
        ("周杰倫", "zhoujielun"),
        ("吳怡君", "wuyijun"),
        ("徐子涵", "xuzihan"),
        ("孫雅筑", "sunYazhu"),
    ];

    names
        .into_iter()
        .enumerate()
        .map(|(i, (name, username))| {
            let id = format!("user-{:03}", i + 1);
            let email = format!("{}@shop4.com", username);
            let hash = hash("password123", DEFAULT_COST).unwrap();
            User {
                id,
                username: name.to_string(),
                email,
                password_hash: hash,
                role: if i == 0 { "admin" } else { "user" }.to_string(),
            }
        })
        .collect()
}

fn generate_categories() -> Vec<Category> {
    vec![
        Category { id: "cat-electronics".to_string(), name: "電子產品".to_string(), parent_id: None },
        Category { id: "cat-phones".to_string(), name: "手機與平板".to_string(), parent_id: Some("cat-electronics".to_string()) },
        Category { id: "cat-laptops".to_string(), name: "筆電與桌機".to_string(), parent_id: Some("cat-electronics".to_string()) },
        Category { id: "cat-accessories".to_string(), name: "3C配件".to_string(), parent_id: Some("cat-electronics".to_string()) },
        Category { id: "cat-clothing".to_string(), name: "服飾".to_string(), parent_id: None },
        Category { id: "cat-mens".to_string(), name: "男裝".to_string(), parent_id: Some("cat-clothing".to_string()) },
        Category { id: "cat-womens".to_string(), name: "女裝".to_string(), parent_id: Some("cat-clothing".to_string()) },
        Category { id: "cat-shoes".to_string(), name: "鞋類".to_string(), parent_id: None },
        Category { id: "cat-sneakers".to_string(), name: "運動鞋".to_string(), parent_id: Some("cat-shoes".to_string()) },
        Category { id: "cat-home".to_string(), name: "家居生活".to_string(), parent_id: None },
        Category { id: "cat-furniture".to_string(), name: "傢俱".to_string(), parent_id: Some("cat-home".to_string()) },
        Category { id: "cat-kitchen".to_string(), name: "廚房用品".to_string(), parent_id: Some("cat-home".to_string()) },
        Category { id: "cat-beauty".to_string(), name: "美妝保養".to_string(), parent_id: None },
        Category { id: "cat-skincare".to_string(), name: "護膚品".to_string(), parent_id: Some("cat-beauty".to_string()) },
        Category { id: "cat-makeup".to_string(), name: "彩妝".to_string(), parent_id: Some("cat-beauty".to_string()) },
        Category { id: "cat-food".to_string(), name: "美食伴手禮".to_string(), parent_id: None },
        Category { id: "cat-snacks".to_string(), name: "零食".to_string(), parent_id: Some("cat-food".to_string()) },
        Category { id: "cat-drinks".to_string(), name: "飲料".to_string(), parent_id: Some("cat-food".to_string()) },
    ]
}

fn generate_products(users: &[User]) -> Vec<Product> {
    let mut rng = rand::thread_rng();
    let mut products = Vec::new();
    let category_images = get_category_images();

    let product_templates: Vec<(&str, Vec<ProductTemplate>)> = vec![
        ("cat-phones", vec![
            ("iPhone 15 Pro Max 256GB", "A17 Pro 晶片鈦金屬邊框相機控制按鍵旗艦手機", "phone", 44900.0),
            ("iPhone 15 128GB", "A16 晶片雙相機系統動態島設計", "phone", 29900.0),
            ("Samsung Galaxy S24 Ultra", "Snapdragon 8 Gen 3 200MP相機S Pen", "phone", 43900.0),
            ("Samsung Galaxy Z Flip5", "Snapdragon 8 Gen 2 折疊式設計", "phone", 32990.0),
            ("Sony Xperia 1 V", "4K HDR OLED 120Hz 還原真實色彩", "phone", 36990.0),
            ("Google Pixel 8 Pro", "Tensor G3 人工智慧相機", "phone", 29900.0),
            ("Xiaomi 13 Ultra", "Leica專屬鏡頭 1吋感光元件", "phone", 27990.0),
            ("ASUS ROG Phone 8", "電競旗艦 165Hz AMOLED", "phone", 24990.0),
        ]),
        ("cat-laptops", vec![
            ("MacBook Pro 14吋 M3 Pro", "M3 Pro 晶片 18核CPU 36GB RAM", "laptop", 82900.0),
            ("MacBook Air 15吋 M3", "M3 晶片 超薄設計 長效續航", "laptop", 54900.0),
            ("Dell XPS 15 9530", "Intel Core i9 13代 4K+ OLED觸控", "laptop", 66900.0),
            ("ThinkPad X1 Carbon Gen 11", "Intel Core i7 13代 輕薄商務", "laptop", 52900.0),
            ("ASUS ROG Zephyrus G14", "AMD Ryzen 9 7940HS RTX 4070", "laptop", 72900.0),
            ("MSI Creator Z17 HX Studio", "Intel i9-13950HX RTX 4070 創作本", "laptop", 79900.0),
            ("HP Pavilion Plus 16", "Intel Core i7 13代 2.8K OLED", "laptop", 39900.0),
            ("Acer Swift Edge 16", "AMD Ryzen 7 7840U 輕薄大螢幕", "laptop", 32900.0),
        ]),
        ("cat-accessories", vec![
            ("AirPods Pro 2", "主動式降噪 USB-C充電", "accessory", 7490.0),
            ("Sony WH-1000XM5", "業界最佳降噪耳罩耳機", "accessory", 9990.0),
            ("Samsung Galaxy Watch6", "健康監測 NFC 支付", "accessory", 9900.0),
            ("Apple Watch Ultra 2", "鈦金屬錶殼 雙頻GPS", "accessory", 27900.0),
            ("Logitech MX Master 3S", "安靜極致安靜精確滾輪", "accessory", 3690.0),
            ("Anker 735 GaN Prime 65W", "氮化鎵 三連接埠 迷你充电器", "accessory", 1290.0),
            ("JBL Flip 6 藍牙喇叭", "IP67防水 12小時續航", "accessory", 2990.0),
            ("Nintendo Switch OLED", "白色主機 7吋OLED螢幕", "accessory", 10480.0),
        ]),
        ("cat-mens", vec![
            ("UNIQLO 男裝特級極輕羽絨外套", "保暖輕便時尚 黑色 M碼", "mens", 1990.0),
            ("GU 男子法蘭絨襯衫", "格紋設計 多色可選 L碼", "mens", 590.0),
            ("Lativ 精梳棉圓領T恤", "基本款白T 3件組 XL碼", "mens", 399.0),
            ("NET 高領針織毛衣", "喀什米爾混紡 深藍 M碼", "mens", 1590.0),
            ("OB紳士休閒短褲", "彈性腰頭 卡其色 32腰", "mens", 790.0),
            ("WCloset 正式西裝外套", "修身版型 碳灰色 L碼", "mens", 2590.0),
        ]),
        ("cat-womens", vec![
            ("UNIQLO 女裝刷毛外套", "保暖連帽 奶茶色 S碼", "womens", 1490.0),
            ("GU 慢跑短褲", "涼感面料 黑色 M碼", "womens", 499.0),
            ("Lativ 法式復古洋裝", "荷葉邊設計 藍色 S碼", "womens", 1290.0),
            ("OB針織開衫", "薄款外套 杏色 M碼", "womens", 1190.0),
            ("Catherine 蕾絲上衣", "約會款 白色 S碼", "womens", 890.0),
            ("WCloset 高腰牛仔褲", "彈性顯瘦 淺藍 26腰", "womens", 1390.0),
        ]),
        ("cat-sneakers", vec![
            ("Nike Air Max 90", "經典氣墊 白色 男女尺寸", "sneaker", 3800.0),
            ("Adidas Ultraboost Light", "回彈緩震 黑色 27cm", "sneaker", 5800.0),
            ("New Balance 5740", "經典復古 灰色 26.5cm", "sneaker", 3280.0),
            ("Converse Chuck 70", "高筒帆布鞋 黑白 26cm", "sneaker", 2680.0),
            ("Vans Old Skool", "經典滑板鞋 黑白 27cm", "sneaker", 2180.0),
            ("ASICS Gel-Kayano 30", "支撐跑鞋 藍色 27.5cm", "sneaker", 4680.0),
        ]),
        ("cat-furniture", vec![
            ("IKEA KALLAX書架", "4x4格層架 白色 77x147cm", "furniture", 2490.0),
            ("IKEA MALM化妝桌", "附抽屜 白色的 120x41cm", "furniture", 3990.0),
            ("特力和樂 人體工學辦公椅", "可調高度 透氣網布 黑色", "furniture", 4990.0),
            ("宜得利 北歐風沙發", "3人座 淺灰色 可拆洗", "furniture", 19990.0),
            ("特力屋 實木餐桌", "6人份 140x80cm 原木色", "furniture", 8990.0),
            ("HOLA 抗菌防蟎床墊", "單人加大 10cm厚", "furniture", 3990.0),
        ]),
        ("cat-kitchen", vec![
            ("氣炸鍋 5.5L", "多功能 無油烹饪 不沾內鍋", "kitchen", 2990.0),
            ("大同電鍋 10人份", "不鏽鋼內鍋 經典款 TAC-10T", "kitchen", 1980.0),
            ("Electrolux 掃地機器人", "雷射導航 APP控制", "kitchen", 12990.0),
            ("Instant Pot 慢燉壓力鍋", "6公升 多功能 8合1", "kitchen", 3990.0),
            ("象印電子鍋 10人份", "NP-HBC18 美味煮", "kitchen", 6980.0),
            ("WMF 刀具組", "德國製 8件組 消毒箱", "kitchen", 3990.0),
        ]),
        ("cat-skincare", vec![
            ("kiehls 金盞花化妝水", "500ml 舒緩保濕 無酒精", "skincare", 1450.0),
            ("La Mer 乳霜", "60ml 經典款 海洋拉帕", "skincare", 9800.0),
            ("Shiseido 百优精華液", "50ml 抗老修護", "skincare", 4200.0),
            ("Dr.Wu 玻尿酸保濕面膜", "5片裝 深層補水", "skincare", 699.0),
            ("Aesop 香芹籽抗氧化精華", "100ml 輕盈保濕", "skincare", 3200.0),
            ("Curél 潤浸保濕乳", "40ml 乾燥敏弱肌適用", "skincare", 680.0),
        ]),
        ("cat-makeup", vec![
            ("YSL 恆久完美粉底", "25ml B10 健康膚色", "makeup", 2100.0),
            ("Dior 癮誘色料唇膏", "3.5g 999 經典紅", "makeup", 1550.0),
            ("M.A.C 柔礦腮紅", "8g Warm Soul 蜜桃色", "makeup", 980.0),
            ("NARS 零觸感粉餅", "10g 透明光澤", "makeup", 1200.0),
            ("植村秀 武士刀眉筆", "05 深棕 自動筆", "makeup", 780.0),
            ("Canmake 彩妝盤", "5色 甜蜜煉瓦 打造好氣色", "makeup", 580.0),
        ]),
        ("cat-snacks", vec![
            ("乖乖 奶油椰子口味", "20包入 經典零食", "snack", 280.0),
            ("旺旺 雪餅/仙貝組", "24包入 禮盒", "snack", 350.0),
            ("卡迪那 德州薯條", "4包入 原始鹹味", "snack", 199.0),
            ("義美 小泡芙", "草莓口味 12入", "snack", 89.0),
            ("77乳加巧克力", "12入 經典款", "snack", 120.0),
            ("老楊 鹹蛋黃麵", "5包入 聯名款", "snack", 249.0),
            ("星巴克 咖啡豆", "1磅 耶加雪菲", "snack", 680.0),
            ("日出茶室 蛋黃酥", "6入 經典口味", "snack", 420.0),
        ]),
        ("cat-drinks", vec![
            ("iFit 抹茶拿鐵", "10包入 無加糖", "drink", 399.0),
            ("劈柴頭 虎咬獅", "600ml 咖啡風味", "drink", 60.0),
            ("Macro 龍潭茉莉綠茶", "600mlx24瓶 箱裝", "drink", 360.0),
            ("維他露P 每日C", "350mlx24瓶 箱裝", "drink", 399.0),
            ("悅氏 運動飲料", "600mlx24瓶 低鈉配方", "drink", 456.0),
            ("茶裏王 現萃茶", "4入 伯爵奶茶", "drink", 85.0),
        ]),
    ];

    let sellers: Vec<&str> = users.iter().filter(|u| u.role == "user").map(|u| u.id.as_str()).collect();
    if sellers.is_empty() {
        return products;
    }

    for (cat_id, items) in product_templates {
        let images = category_images.get(cat_id).cloned().unwrap_or_else(|| {
            vec!["https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=400"]
        });

        for (name, desc, _type_key, price) in items {
            let id = Uuid::new_v4().to_string();
            let seller_id = sellers.choose(&mut rng).unwrap().to_string();
            let stock = rng.gen_range(10..200);
            let rating = (rng.gen_range(35..50) as f64) / 10.0;
            let sold_count = rng.gen_range(0..500);
            let image = images.choose(&mut rng).unwrap().to_string();

            products.push(Product {
                id,
                seller_id,
                category_id: cat_id.to_string(),
                name: name.to_string(),
                description: desc.to_string(),
                price,
                stock,
                image_url: image,
                rating,
                sold_count,
            });
        }
    }

    products
}

fn create_test_data() -> TestData {
    let users = generate_users();
    let categories = generate_categories();
    let products = generate_products(&users);

    TestData {
        users,
        categories,
        products,
    }
}

async fn init_database(pool: &sqlx::Pool<sqlx::Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'user',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            parent_id TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (parent_id) REFERENCES categories(id)
        );

        CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            seller_id TEXT NOT NULL,
            category_id TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            price REAL NOT NULL,
            stock INTEGER NOT NULL DEFAULT 0,
            image_url TEXT,
            rating REAL DEFAULT 0.0,
            sold_count INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (seller_id) REFERENCES users(id),
            FOREIGN KEY (category_id) REFERENCES categories(id)
        );

        CREATE TABLE IF NOT EXISTS cart_items (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            product_id TEXT NOT NULL,
            quantity INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (product_id) REFERENCES products(id),
            UNIQUE(user_id, product_id)
        );

        CREATE TABLE IF NOT EXISTS orders (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            total_amount REAL NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            shipping_address TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );

        CREATE TABLE IF NOT EXISTS order_items (
            id TEXT PRIMARY KEY,
            order_id TEXT NOT NULL,
            product_id TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            price REAL NOT NULL,
            FOREIGN KEY (order_id) REFERENCES orders(id),
            FOREIGN KEY (product_id) REFERENCES products(id)
        );

        CREATE TABLE IF NOT EXISTS reviews (
            id TEXT PRIMARY KEY,
            product_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            rating INTEGER NOT NULL,
            comment TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (product_id) REFERENCES products(id),
            FOREIGN KEY (user_id) REFERENCES users(id),
            UNIQUE(product_id, user_id)
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "/Users/Shared/ccc/project/computer4/web/shop4/shop4.db";
    std::fs::remove_file(db_path).ok();
    std::fs::File::create(db_path).expect("Failed to create database file");

    let database_url = format!("sqlite:{}", db_path);
    let pool = sqlx::SqlitePool::connect(&database_url).await?;

    println!("Initializing database...");
    init_database(&pool).await?;

    let data = create_test_data();
    let now = chrono::Utc::now().to_rfc3339();

    println!("Inserting {} users...", data.users.len());
    for user in &data.users {
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.role)
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await?;
    }

    println!("Inserting {} categories...", data.categories.len());
    for cat in &data.categories {
        sqlx::query(
            "INSERT INTO categories (id, name, parent_id, created_at) VALUES (?, ?, ?, ?)",
        )
        .bind(&cat.id)
        .bind(&cat.name)
        .bind(&cat.parent_id)
        .bind(&now)
        .execute(&pool)
        .await?;
    }

    println!("Inserting {} products...", data.products.len());
    for product in &data.products {
        sqlx::query(
            "INSERT INTO products (id, seller_id, category_id, name, description, price, stock, image_url, rating, sold_count, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&product.id)
        .bind(&product.seller_id)
        .bind(&product.category_id)
        .bind(&product.name)
        .bind(&product.description)
        .bind(&product.price)
        .bind(&product.stock)
        .bind(&product.image_url)
        .bind(&product.rating)
        .bind(&product.sold_count)
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await?;
    }

    println!("\n=== Test data built successfully! ===");
    println!("Users: {} (login: wangxiaoming@shop4.com / password123)", data.users.len());
    println!("Categories: {}", data.categories.len());
    println!("Products: {}", data.products.len());

    Ok(())
}