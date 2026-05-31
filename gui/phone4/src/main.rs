use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Vec2};

const C_BLUE: Color32 = Color32::from_rgb(0, 122, 255);
const C_GREEN: Color32 = Color32::from_rgb(52, 199, 89);
const C_RED: Color32 = Color32::from_rgb(255, 59, 48);
const C_ORANGE: Color32 = Color32::from_rgb(255, 149, 0);
const C_DARK: Color32 = Color32::from_rgb(28, 28, 30);
const C_BG: Color32 = Color32::from_rgb(17, 17, 17);

const PHONE_W: f32 = 393.0;
const PHONE_H: f32 = 852.0;
const STATUS_H: f32 = 54.0;
const DOCK_H: f32 = 100.0;
const INDICATOR_H: f32 = 34.0;
const ICON_S: f32 = 60.0;
const CORNER: f32 = 48.0;
const BEZEL: f32 = 10.0;

#[derive(Clone, PartialEq)]
enum Screen {
    Home, Calculator, Settings, Notes, Camera, Messages,
    Music, Weather, Phone, Photos, Maps, Clock, Calendar, Reminders,
}

struct AppInfo {
    name: &'static str,
    icon: &'static str,
    color: Color32,
    screen: Screen,
}

const HOME_APPS: &[AppInfo] = &[
    AppInfo { name: "電話",   icon: "\u{1F4DE}", color: C_GREEN,  screen: Screen::Phone },
    AppInfo { name: "訊息",   icon: "\u{1F4AC}", color: C_GREEN,  screen: Screen::Messages },
    AppInfo { name: "計算機", icon: "\u{1F9EE}", color: C_ORANGE, screen: Screen::Calculator },
    AppInfo { name: "相機",   icon: "\u{1F4F7}", color: C_GREEN,  screen: Screen::Camera },
    AppInfo { name: "設定",   icon: "\u{2699}\u{FE0F}", color: Color32::from_rgb(142,142,147), screen: Screen::Settings },
    AppInfo { name: "音樂",   icon: "\u{1F3B5}", color: C_RED,    screen: Screen::Music },
    AppInfo { name: "天氣",   icon: "\u{1F324}\u{FE0F}", color: C_BLUE, screen: Screen::Weather },
    AppInfo { name: "照片",   icon: "\u{1F5BC}\u{FE0F}", color: Color32::from_rgb(90,200,250), screen: Screen::Photos },
    AppInfo { name: "地圖",   icon: "\u{1F5FA}\u{FE0F}", color: C_GREEN, screen: Screen::Maps },
    AppInfo { name: "時鐘",   icon: "\u{1F550}", color: C_BLUE,   screen: Screen::Clock },
    AppInfo { name: "日曆",   icon: "\u{1F4C5}", color: C_RED,    screen: Screen::Calendar },
    AppInfo { name: "備忘錄", icon: "\u{1F4DD}", color: Color32::from_rgb(255,204,0), screen: Screen::Notes },
    AppInfo { name: "提醒事項",icon: "\u{2705}",  color: C_RED,    screen: Screen::Reminders },
];

const DOCK_APPS: &[AppInfo] = &[
    AppInfo { name: "電話", icon: "\u{1F4DE}", color: C_GREEN, screen: Screen::Phone },
    AppInfo { name: "Safari", icon: "\u{1F310}", color: C_BLUE, screen: Screen::Home },
    AppInfo { name: "訊息", icon: "\u{1F4AC}", color: C_GREEN, screen: Screen::Messages },
    AppInfo { name: "音樂", icon: "\u{1F3B5}", color: C_RED,  screen: Screen::Music },
];

#[derive(Clone)]
struct ChatMsg { me: bool, text: String }

struct Phone4App {
    screen: Screen,
    prev_screens: Vec<Screen>,

    calc_disp: String,
    calc_prev: f64,
    calc_op: Option<char>,
    calc_new: bool,

    wifi: bool, bt: bool, dark: bool, volume: f32, brightness: f32,
    notes: String,
    cam_on: bool,
    msgs: Vec<ChatMsg>,
    music_playing: bool, music_track: usize,
    phone_num: String, phone_call: bool,

    time_str: String,
    date_str: String,
}

impl Default for Phone4App {
    fn default() -> Self {
        Self {
            screen: Screen::Home, prev_screens: vec![],
            calc_disp: "0".into(), calc_prev: 0.0, calc_op: None, calc_new: false,
            wifi: true, bt: true, dark: false, volume: 0.7, brightness: 0.8,
            notes: "你好，世界！\n這是一則測試備忘錄。".into(),
            cam_on: false,
            msgs: vec![
                ChatMsg { me: false, text: "明天幾點吃飯？".into() },
                ChatMsg { me: true, text: "十二點半如何？".into() },
                ChatMsg { me: false, text: "好，到時見！".into() },
            ],
            music_playing: false, music_track: 0,
            phone_num: String::new(), phone_call: false,
            time_str: String::new(), date_str: String::new(),
        }
    }
}

impl Phone4App {
    fn new() -> Self { Self::default() }

    fn update_datetime(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        let secs = d.as_secs();
        let days = secs / 86400;
        let t = secs % 86400;
        let h = t / 3600;
        let m = (t % 3600) / 60;
        let wday = ((days + 4) % 7) as usize;
        let wdays = ["週日","週一","週二","週三","週四","週五","週六"];
        let yday = days as i64 - epoch_days(1970, 1, 1);
        let (_, mo, d) = days_to_ymd(yday);
        self.time_str = format!("{:02}:{:02}", h, m);
        self.date_str = format!("{}月{}日 {}", mo, d, wdays[wday]);
    }

    fn go_home(&mut self) { self.screen = Screen::Home; self.prev_screens.clear(); }
    fn open_app(&mut self, s: &Screen) {
        if *s == Screen::Home { return; }
        self.prev_screens.push(self.screen.clone());
        self.screen = s.clone();
    }
    fn go_back(&mut self) {
        if let Some(prev) = self.prev_screens.pop() { self.screen = prev; }
        else { self.go_home(); }
    }
}

fn epoch_days(y: i64, mo: i64, d: i64) -> i64 {
    fn is_leap(y: i64) -> bool { (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 }
    let mdays = [31, if is_leap(y) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut total = 0i64;
    for y0 in 1970..y { total += if is_leap(y0) { 366 } else { 365 }; }
    for m0 in 0..mo-1 { total += mdays[m0 as usize]; }
    total + d - 1
}

fn days_to_ymd(mut n: i64) -> (i64, i64, i64) {
    fn is_leap(y: i64) -> bool { (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 }
    let mut y = 1970i64;
    loop {
        let ylen = if is_leap(y) { 366 } else { 365 };
        if n < ylen { break; }
        n -= ylen; y += 1;
    }
    let mdays = [31, if is_leap(y) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for (i, &md) in mdays.iter().enumerate() {
        if n < md { return (y, (i+1) as i64, n+1); }
        n -= md;
    }
    (y, 12, 31)
}

// ── 繪圖輔助 ──
fn fill_rect(p: &egui::Painter, r: Rect, cr: f32, fill: Color32) {
    p.rect(r, Rounding::same(cr), fill, egui::Stroke::new(0.0_f32, Color32::TRANSPARENT));
}

fn stroke_rect(p: &egui::Painter, r: Rect, cr: f32, fill: Color32, sw: f32, sc: Color32) {
    p.rect(r, Rounding::same(cr), fill, egui::Stroke::new(sw, sc));
}

fn draw_icon_bg(p: &egui::Painter, center: Pos2, s: f32, color: Color32) -> Rect {
    let r = Rect::from_center_size(center, Vec2::splat(s));
    fill_rect(p, r, s * 0.22, color);
    r
}

fn pointer_in(rect: Rect, ctx: &egui::Context) -> bool {
    ctx.input(|i| i.pointer.interact_pos()).map_or(false, |p| rect.contains(p))
}

// ═══════════════════════════════════════════════
//  App 畫面（全部使用 global 座標，ctx + Rect）
// ═══════════════════════════════════════════════

fn calculator_ui(app: &mut Phone4App, ctx: &egui::Context, r: Rect) {
    let p = ctx.debug_painter();
    fill_rect(&p, r, 0.0, C_DARK);
    let disp_h = r.height() * 0.28;
    let eq = app.calc_op.map(|op| format!("{} {} ", app.calc_prev, op)).unwrap_or_default();
    let txt = format!("{}{}", eq, app.calc_disp);
    p.text(Pos2::new(r.right()-16.0, r.top()+disp_h-12.0), egui::Align2::RIGHT_BOTTOM, &txt,
           egui::FontId::proportional(46.0), Color32::WHITE);

    struct CalcBtn { label: &'static str, span: usize }
    let rows: [&[CalcBtn]; 5] = [
        &[CalcBtn{label:"AC",span:1},CalcBtn{label:"±",span:1},CalcBtn{label:"%",span:1},CalcBtn{label:"÷",span:1}],
        &[CalcBtn{label:"7",span:1},CalcBtn{label:"8",span:1},CalcBtn{label:"9",span:1},CalcBtn{label:"×",span:1}],
        &[CalcBtn{label:"4",span:1},CalcBtn{label:"5",span:1},CalcBtn{label:"6",span:1},CalcBtn{label:"−",span:1}],
        &[CalcBtn{label:"1",span:1},CalcBtn{label:"2",span:1},CalcBtn{label:"3",span:1},CalcBtn{label:"+",span:1}],
        &[CalcBtn{label:"0",span:2},CalcBtn{label:".",span:1},CalcBtn{label:"=",span:1}],
    ];

    let cols = 4;
    let area = Rect::from_min_size(r.left_top()+Vec2::new(0.0,disp_h+8.0), Vec2::new(r.width(), r.height()-disp_h-8.0));
    let bw = area.width() / cols as f32;
    let bh = area.height() / rows.len() as f32;

    for (ri, row) in rows.iter().enumerate() {
        let mut ci = 0;
        for btn in *row {
            let is_op = matches!(btn.label, "÷"|"×"|"−"|"+"|"=");
            let is_fn = matches!(btn.label, "AC"|"±"|"%");
            let bc = if is_op { C_ORANGE } else if is_fn { Color32::from_gray(165) } else { Color32::from_gray(51) };
            let tc = if is_fn { Color32::BLACK } else { Color32::WHITE };
            let w = bw * btn.span as f32 - 8.0;
            let br = Rect::from_min_size(Pos2::new(area.left()+ci as f32*bw+4.0, area.top()+ri as f32*bh+4.0),
                                          Vec2::new(w, bh-8.0));
            fill_rect(&p, br, br.height()/2.0, bc);
            p.text(br.center(), egui::Align2::CENTER_CENTER, btn.label,
                   egui::FontId::proportional(if btn.span==2{28.0}else if is_fn{20.0}else{28.0}), tc);
            if pointer_in(br, ctx) && ctx.input(|i| i.pointer.any_click()) { calc_handle(app, btn.label); }
            ci += btn.span;
        }
    }
}

fn calc_handle(app: &mut Phone4App, label: &str) {
    match label {
        "AC" => { app.calc_disp = "0".into(); app.calc_prev = 0.0; app.calc_op = None; app.calc_new = false; }
        "±" => { app.calc_disp = format!("{}", -app.calc_disp.parse::<f64>().unwrap_or(0.0)); }
        "%" => { app.calc_disp = format!("{}", app.calc_disp.parse::<f64>().unwrap_or(0.0)/100.0); }
        "+"|"−"|"×"|"÷" => {
            let v = app.calc_disp.parse::<f64>().unwrap_or(0.0);
            let op_c = match label { "+"=>'+', "−"=>'-', "×"=>'*', _=>'/' };
            if let Some(op) = app.calc_op { app.calc_prev = calc_exec(app.calc_prev, v, op); }
            else { app.calc_prev = v; }
            app.calc_op = Some(op_c); app.calc_new = true;
        }
        "=" => {
            let v = app.calc_disp.parse::<f64>().unwrap_or(0.0);
            if let Some(op) = app.calc_op { app.calc_disp = format!("{}", calc_exec(app.calc_prev, v, op)); app.calc_op = None; app.calc_new = true; }
        }
        "." => { if app.calc_new { app.calc_disp = "0.".into(); app.calc_new = false; } else if !app.calc_disp.contains('.') { app.calc_disp.push('.'); } }
        _ => { if app.calc_disp=="0"||app.calc_new { app.calc_disp=label.into(); app.calc_new=false; } else { app.calc_disp.push_str(label); } }
    }
}

fn calc_exec(a: f64, b: f64, op: char) -> f64 {
    match op { '+'=>a+b, '-'=>a-b, '*'=>a*b, _=>a/b }
}

fn settings_ui(app: &mut Phone4App, ctx: &egui::Context, r: Rect) {
    let p = ctx.debug_painter();
    let fg = if app.dark { Color32::WHITE } else { Color32::BLACK };
    let bg = if app.dark { C_DARK } else { Color32::from_rgb(242,242,247) };
    let cell = if app.dark { Color32::from_gray(40) } else { Color32::WHITE };
    fill_rect(&p, r, 0.0, bg);

    let mut y = r.top()+16.0; let lx = r.left()+20.0;
    let toggles: [(&str, &mut bool); 3] = [("Wi-Fi",&mut app.wifi),("藍牙",&mut app.bt),("深色模式",&mut app.dark)];
    for (name, val) in toggles {
        let ir = Rect::from_min_size(Pos2::new(r.left(),y), Vec2::new(r.width(),44.0));
        fill_rect(&p, ir, 0.0, cell);
        p.text(Pos2::new(lx,y+22.0), egui::Align2::LEFT_CENTER, name, egui::FontId::proportional(16.0), fg);
        let tog_r = Rect::from_min_size(Pos2::new(ir.right()-56.0,y+10.0), Vec2::new(44.0,24.0));
        fill_rect(&p, tog_r, 12.0, if *val{C_GREEN}else{Color32::from_gray(180)});
        let kx = if *val{tog_r.right()-18.0}else{tog_r.left()+6.0};
        p.circle(Pos2::new(kx+6.0,tog_r.center().y), 9.0, Color32::WHITE, egui::Stroke::new(0.0_f32,Color32::TRANSPARENT));
        if pointer_in(tog_r, ctx) && ctx.input(|i| i.pointer.any_click()) { *val = !*val; }
        y += 44.0;
    }

    y += 16.0;
    let sliders: [(&str, &mut f32); 2] = [("音量",&mut app.volume),("亮度",&mut app.brightness)];
    for (label, val) in sliders {
        let ir = Rect::from_min_size(Pos2::new(r.left(),y), Vec2::new(r.width(),56.0));
        fill_rect(&p, ir, 0.0, cell);
        p.text(Pos2::new(lx,y+18.0), egui::Align2::LEFT_CENTER, label, egui::FontId::proportional(16.0), fg);
        let sl = Rect::from_min_size(Pos2::new(lx+80.0,y+24.0), Vec2::new(r.width()-120.0,6.0));
        fill_rect(&p, sl, 3.0, Color32::from_gray(160));
        let fw = sl.width()**val;
        fill_rect(&p, Rect::from_min_size(sl.left_top(),Vec2::new(fw,sl.height())), 3.0, C_BLUE);
        p.circle(Pos2::new(sl.left()+fw,sl.center().y), 7.0, Color32::WHITE, egui::Stroke::new(1.0_f32,C_BLUE));
        if pointer_in(sl, ctx) && ctx.input(|i| i.pointer.any_down()) {
            if let Some(m) = ctx.input(|i| i.pointer.interact_pos()) { *val = ((m.x-sl.left())/sl.width()).clamp(0.0,1.0); }
        }
        y += 56.0;
    }
}

fn notes_ui(app: &mut Phone4App, ctx: &egui::Context, r: Rect) {
    let p = ctx.debug_painter();
    fill_rect(&p, r, 0.0, C_DARK);
    let editor_r = r.shrink(16.0);
    // 使用 Area 來承載 TextEdit widget
    egui::Area::new(egui::Id::new("notes_editor")).fixed_pos(editor_r.left_top()).show(ctx, |ui| {
        ui.set_clip_rect(editor_r);
        ui.add_sized(editor_r.size(), egui::TextEdit::multiline(&mut app.notes)
            .font(egui::TextStyle::Body)
            .desired_width(f32::INFINITY));
    });
}

fn camera_ui(app: &mut Phone4App, ctx: &egui::Context, r: Rect) {
    let p = ctx.debug_painter();
    fill_rect(&p, r, 0.0, Color32::from_rgb(10,10,12));
    p.text(r.center(), egui::Align2::CENTER_CENTER, "📷", egui::FontId::proportional(80.0), Color32::WHITE);
    let btn_c = r.center()+Vec2::new(0.0,r.height()*0.3);
    p.circle(btn_c, 30.0, Color32::WHITE.linear_multiply(0.2), egui::Stroke::new(4.0_f32,Color32::WHITE));
    p.circle(btn_c, 26.0, Color32::WHITE, egui::Stroke::new(0.0_f32,Color32::TRANSPARENT));
    if pointer_in(Rect::from_center_size(btn_c,Vec2::splat(60.0)), ctx) && ctx.input(|i| i.pointer.any_click()) {
        app.cam_on = !app.cam_on;
    }
    if app.cam_on {
        fill_rect(&p, r, 0.0, Color32::from_rgb(200,230,50).linear_multiply(0.15));
        p.text(r.center(), egui::Align2::CENTER_CENTER, "📸 已拍照！", egui::FontId::proportional(20.0), Color32::WHITE);
    }
}

fn messages_ui(app: &mut Phone4App, ctx: &egui::Context, r: Rect, scale: f32) {
    let p = ctx.debug_painter();
    fill_rect(&p, r, 0.0, Color32::from_rgb(20,20,22));
    let mut y = r.top()+8.0*scale;
    for msg in &app.msgs {
        let tw = msg.text.len() as f32 * 7.5 * scale;
        let max_w = r.width()*0.7;
        let bw = (tw+24.0*scale).min(max_w);
        let bx = if msg.me { r.right()-bw-12.0*scale } else { r.left()+12.0*scale };
        let br = Rect::from_min_size(Pos2::new(bx,y), Vec2::new(bw,32.0*scale));
        fill_rect(&p, br, 14.0*scale, if msg.me{C_GREEN.linear_multiply(0.85)}else{Color32::from_gray(40)});
        p.text(Pos2::new(bx+12.0*scale,y+8.0*scale), egui::Align2::LEFT_TOP, &msg.text, egui::FontId::proportional(15.0*scale), Color32::WHITE);
        y += 38.0*scale;
    }

    let btn_w = (r.width()-40.0*scale)/3.0;
    let btn_y = r.bottom()-48.0*scale;
    for (i, t) in ["好","收到","👍"].iter().enumerate() {
        let br = Rect::from_min_size(Pos2::new(r.left()+10.0*scale+i as f32*(btn_w+10.0*scale),btn_y), Vec2::new(btn_w,32.0*scale));
        fill_rect(&p, br, 16.0*scale, C_BLUE.linear_multiply(0.8));
        p.text(br.center(), egui::Align2::CENTER_CENTER, *t, egui::FontId::proportional(14.0*scale), Color32::WHITE);
        if pointer_in(br, ctx) && ctx.input(|i| i.pointer.any_click()) {
            app.msgs.push(ChatMsg { me: true, text: t.to_string() });
        }
    }
}

fn music_ui(app: &mut Phone4App, ctx: &egui::Context, r: Rect, scale: f32) {
    let tracks: &[&str] = &["夏日午後","星空下","城市節奏","清晨漫步","海洋之聲"];
    let artists: &[&str] = &["林小美","陳大中","樂團A","鋼琴家B","自然錄音"];
    let p = ctx.debug_painter();
    fill_rect(&p, r, 0.0, C_DARK);
    let c = r.center();

    let cover = Rect::from_center_size(c-Vec2::new(0.0,100.0*scale), Vec2::splat(160.0*scale));
    fill_rect(&p, cover, 16.0*scale, C_RED.linear_multiply(0.6));
    p.text(cover.center(), egui::Align2::CENTER_CENTER, "🎵", egui::FontId::proportional(50.0*scale), Color32::WHITE.linear_multiply(0.8));

    let idx = app.music_track;
    p.text(Pos2::new(c.x,cover.bottom()+24.0*scale), egui::Align2::CENTER_CENTER, tracks[idx], egui::FontId::proportional(22.0*scale), Color32::WHITE);
    p.text(Pos2::new(c.x,cover.bottom()+48.0*scale), egui::Align2::CENTER_CENTER, artists[idx], egui::FontId::proportional(14.0*scale), Color32::GRAY);

    let pr = Rect::from_center_size(Pos2::new(c.x,cover.bottom()+80.0*scale), Vec2::new(240.0*scale,4.0*scale));
    fill_rect(&p, pr, 2.0*scale, Color32::from_gray(60));
    fill_rect(&p, Rect::from_min_size(pr.left_top(),Vec2::new(pr.width()*0.35,pr.height())), 2.0*scale, Color32::WHITE);

    let by = cover.bottom()+130.0*scale;
    for (dx, lbl) in [(-100.0*scale,"⏮"),(-40.0*scale,if app.music_playing{"⏸"}else{"▶️"}),(40.0*scale,"⏭"),(100.0*scale,"🔀")] {
        let br = Rect::from_center_size(Pos2::new(c.x+dx,by), Vec2::splat(44.0*scale));
        p.circle(br.center(), 22.0*scale, Color32::TRANSPARENT, egui::Stroke::new(1.5_f32*scale,Color32::GRAY));
        p.text(br.center(), egui::Align2::CENTER_CENTER, lbl, egui::FontId::proportional(20.0*scale), Color32::WHITE);
        if pointer_in(br, ctx) && ctx.input(|i| i.pointer.any_click()) {
            match lbl { "▶️"|"⏸" => app.music_playing = !app.music_playing,
                         "⏮" => { if app.music_track>0 { app.music_track-=1; } }
                         "⏭" => { if app.music_track<tracks.len()-1 { app.music_track+=1; } }
                         _ => app.music_track = (app.music_track+2)%tracks.len() }
        }
    }
}

fn weather_ui(_app: &mut Phone4App, ctx: &egui::Context, r: Rect) {
    let p = ctx.debug_painter();
    let top_c = Color32::from_rgb(80,150,220); let bot_c = Color32::from_rgb(40,80,160);
    for i in 0..20 {
        let t = i as f32/20.0; let y0 = r.top()+r.height()*t; let y1 = r.top()+r.height()*(t+1.0/20.0);
        let c = Color32::from_rgb((top_c[0]as f32*(1.0-t)+bot_c[0]as f32*t) as u8,
                                   (top_c[1]as f32*(1.0-t)+bot_c[1]as f32*t) as u8,
                                   (top_c[2]as f32*(1.0-t)+bot_c[2]as f32*t) as u8);
        fill_rect(&p, Rect::from_min_size(Pos2::new(r.left(),y0),Vec2::new(r.width(),y1-y0)), 0.0, c);
    }
    p.text(r.center()+Vec2::new(0.0,-60.0), egui::Align2::CENTER_CENTER, "🌤", egui::FontId::proportional(60.0), Color32::WHITE);
    p.text(r.center()+Vec2::new(0.0,-10.0), egui::Align2::CENTER_CENTER, "27°", egui::FontId::proportional(60.0), Color32::WHITE);
    p.text(r.center()+Vec2::new(0.0,30.0), egui::Align2::CENTER_CENTER, "晴時多雲", egui::FontId::proportional(20.0), Color32::WHITE.linear_multiply(0.9));
    p.text(r.center()+Vec2::new(0.0,56.0), egui::Align2::CENTER_CENTER, "高30° 低22°", egui::FontId::proportional(15.0), Color32::WHITE.linear_multiply(0.7));

    let hours = ["現在","14","15","16","17","18"];
    let temps = ["27°","28°","28°","26°","24°","22°"];
    let icons = ["🌤","🌤","⛅","☁️","🌧","🌧"];
    let hw = r.width()/hours.len() as f32; let hy = r.center().y+80.0;
    for i in 0..hours.len() {
        let cx = r.left()+(i as f32+0.5)*hw;
        p.text(Pos2::new(cx,hy), egui::Align2::CENTER_CENTER, icons[i], egui::FontId::proportional(22.0), Color32::WHITE);
        p.text(Pos2::new(cx,hy+28.0), egui::Align2::CENTER_CENTER, temps[i], egui::FontId::proportional(15.0), Color32::WHITE);
        p.text(Pos2::new(cx,hy+48.0), egui::Align2::CENTER_CENTER, hours[i], egui::FontId::proportional(12.0), Color32::WHITE.linear_multiply(0.7));
    }
}

fn phone_ui(app: &mut Phone4App, ctx: &egui::Context, r: Rect, scale: f32) {
    let p = ctx.debug_painter();
    fill_rect(&p, r, 0.0, C_DARK);
    let c = r.center();
    p.text(Pos2::new(c.x,r.top()+40.0*scale), egui::Align2::CENTER_CENTER,
           if app.phone_num.is_empty(){"撥打號碼"}else{&app.phone_num},
           egui::FontId::proportional(32.0*scale), if app.phone_num.is_empty(){Color32::GRAY}else{Color32::WHITE});

    let keys: &[&[&str]] = &[&["1","2","3"],&["4","5","6"],&["7","8","9"],&["*","0","#"]];
    let kw=66.0*scale; let kh=66.0*scale; let gap=10.0*scale;
    let sx=c.x-(kw*3.0+gap*2.0)/2.0; let sy=r.top()+90.0*scale;
    for (ri,row) in keys.iter().enumerate() {
        for (ci,&k) in row.iter().enumerate() {
            let kr = Rect::from_center_size(Pos2::new(sx+ci as f32*(kw+gap)+kw/2.0,sy+ri as f32*(kh+gap)+kh/2.0), Vec2::splat(kw));
            p.circle(kr.center(), kw/2.0, Color32::from_gray(40), egui::Stroke::new(0.0_f32,Color32::TRANSPARENT));
            p.text(kr.center(), egui::Align2::CENTER_CENTER, k, egui::FontId::proportional(26.0*scale), Color32::WHITE);
            if pointer_in(kr, ctx) && ctx.input(|i| i.pointer.any_click()) { app.phone_num.push_str(k); }
        }
    }

    let cy = sy+4.0*(kh+gap);
    let call_r = Rect::from_center_size(Pos2::new(c.x,cy), Vec2::splat(72.0*scale));
    p.circle(call_r.center(), 36.0*scale, C_GREEN, egui::Stroke::new(0.0_f32,Color32::TRANSPARENT));
    p.text(call_r.center(), egui::Align2::CENTER_CENTER, if app.phone_call{"📵"}else{"📞"}, egui::FontId::proportional(28.0*scale), Color32::WHITE);
    if pointer_in(call_r, ctx) && ctx.input(|i| i.pointer.any_click()) && !app.phone_num.is_empty() { app.phone_call = !app.phone_call; }

    if !app.phone_num.is_empty() {
        let dr = Rect::from_center_size(Pos2::new(c.x+90.0*scale,cy), Vec2::splat(44.0*scale));
        p.circle(dr.center(), 22.0*scale, Color32::from_gray(60), egui::Stroke::new(0.0_f32,Color32::TRANSPARENT));
        p.text(dr.center(), egui::Align2::CENTER_CENTER, "⌫", egui::FontId::proportional(22.0*scale), Color32::WHITE);
        if pointer_in(dr, ctx) && ctx.input(|i| i.pointer.any_click()) { app.phone_num.pop(); }
    }
}

fn placeholder_ui(name: &str, icon: &str, ctx: &egui::Context, r: Rect) {
    let p = ctx.debug_painter();
    fill_rect(&p, r, 0.0, C_DARK);
    p.text(r.center(), egui::Align2::CENTER_CENTER, icon, egui::FontId::proportional(64.0), Color32::WHITE.linear_multiply(0.8));
    p.text(r.center()+Vec2::new(0.0,48.0), egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(20.0), Color32::WHITE.linear_multiply(0.6));
}

// ═══════════════════════════════════════════════
//  Phone4App 實作
// ═══════════════════════════════════════════════

impl Phone4App {
    fn render_home(&self, ctx: &egui::Context, content_r: Rect, scale: f32) {
        let p = ctx.debug_painter();
        let cols = 4;
        let icon_total = ICON_S * scale;
        let h_gap = (content_r.width()-icon_total*cols as f32)/(cols as f32+1.0);
        let start_y = content_r.top()+24.0*scale;

        for (i, app) in HOME_APPS.iter().enumerate() {
            let col = i%cols; let row = i/cols;
            let cx = content_r.left()+h_gap+(col as f32+0.5)*(icon_total+h_gap);
            let cy = start_y+row as f32*(icon_total+36.0*scale);
            let ic = Pos2::new(cx,cy);
            draw_icon_bg(&p, ic, icon_total, app.color);
            p.text(ic, egui::Align2::CENTER_CENTER, app.icon, egui::FontId::proportional(icon_total*0.45), Color32::WHITE);
            p.text(Pos2::new(cx,cy+icon_total/2.0+8.0*scale), egui::Align2::CENTER_CENTER, app.name,
                   egui::FontId::proportional(11.0*scale), Color32::WHITE.linear_multiply(0.9));
        }
    }

    fn hit_home_icon(&mut self, ctx: &egui::Context, content_r: Rect, scale: f32) -> bool {
        let cols = 4;
        let icon_total = ICON_S * scale;
        let h_gap = (content_r.width()-icon_total*cols as f32)/(cols as f32+1.0);
        let start_y = content_r.top()+24.0*scale;

        if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
            if !ctx.input(|i| i.pointer.any_click()) { return false; }
            for (i, app) in HOME_APPS.iter().enumerate() {
                let col = i%cols; let row = i/cols;
                let cx = content_r.left()+h_gap+(col as f32+0.5)*(icon_total+h_gap);
                let cy = start_y+row as f32*(icon_total+36.0*scale);
                let hit_r = Rect::from_center_size(Pos2::new(cx,cy), Vec2::new(icon_total+h_gap*0.5, icon_total+36.0*scale));
                if hit_r.contains(pos) { self.open_app(&app.screen); return true; }
            }
        }
        false
    }

    fn render_dock(&self, ctx: &egui::Context, dock_r: Rect, scale: f32) {
        let p = ctx.debug_painter();
        fill_rect(&p, dock_r, dock_r.height()*0.3, Color32::from_black_alpha(120));
        let icon_total = ICON_S*scale*0.85;
        let count = DOCK_APPS.len();
        let total_w = count as f32*icon_total+(count-1) as f32*12.0*scale;
        let start_x = dock_r.center().x-total_w/2.0;
        for (i, app) in DOCK_APPS.iter().enumerate() {
            let cx = start_x+(i as f32+0.5)*icon_total+i as f32*12.0*scale;
            let cy = dock_r.center().y; let ic = Pos2::new(cx,cy);
            draw_icon_bg(&p, ic, icon_total, app.color);
            p.text(ic, egui::Align2::CENTER_CENTER, app.icon, egui::FontId::proportional(icon_total*0.4), Color32::WHITE);
        }
    }

    fn hit_dock_icon(&mut self, ctx: &egui::Context, dock_r: Rect, scale: f32) -> bool {
        let icon_total = ICON_S*scale*0.85;
        let count = DOCK_APPS.len();
        let total_w = count as f32*icon_total+(count-1) as f32*12.0*scale;
        let start_x = dock_r.center().x-total_w/2.0;
        if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
            if !ctx.input(|i| i.pointer.any_click()) { return false; }
            for (i, app) in DOCK_APPS.iter().enumerate() {
                let cx = start_x+(i as f32+0.5)*icon_total+i as f32*12.0*scale;
                let hit_r = Rect::from_center_size(Pos2::new(cx,dock_r.center().y), Vec2::splat(icon_total+10.0*scale));
                if hit_r.contains(pos) && app.screen != Screen::Home { self.open_app(&app.screen); return true; }
            }
        }
        false
    }

    fn render_app_ui(&mut self, ctx: &egui::Context, nav_r: Rect, app_r: Rect, scale: f32, screen: &Screen) {
        let p = ctx.debug_painter();
        fill_rect(&p, nav_r, 0.0, C_DARK);

        let back_r = Rect::from_min_size(nav_r.left_top()+Vec2::new(4.0*scale,(nav_r.height()-24.0*scale)/2.0), Vec2::new(50.0*scale,24.0*scale));
        p.text(back_r.center(), egui::Align2::CENTER_CENTER, "‹ 返回", egui::FontId::proportional(15.0*scale), C_BLUE);
        let _ = back_r;

        let title = match screen {
            Screen::Calculator=>"計算機",Screen::Settings=>"設定",Screen::Notes=>"備忘錄",
            Screen::Camera=>"相機",Screen::Messages=>"訊息",Screen::Music=>"音樂",
            Screen::Weather=>"天氣",Screen::Phone=>"電話",Screen::Photos=>"照片",
            Screen::Maps=>"地圖",Screen::Clock=>"時鐘",Screen::Calendar=>"日曆",
            Screen::Reminders=>"提醒事項", _=>"",
        };
        p.text(Pos2::new(nav_r.center().x,nav_r.center().y), egui::Align2::CENTER_CENTER, title,
               egui::FontId::proportional(16.0*scale), Color32::WHITE);

        match screen {
            Screen::Calculator => calculator_ui(self, ctx, app_r),
            Screen::Settings => settings_ui(self, ctx, app_r),
            Screen::Notes => notes_ui(self, ctx, app_r),
            Screen::Camera => camera_ui(self, ctx, app_r),
            Screen::Messages => messages_ui(self, ctx, app_r, scale),
            Screen::Music => music_ui(self, ctx, app_r, scale),
            Screen::Weather => weather_ui(self, ctx, app_r),
            Screen::Phone => phone_ui(self, ctx, app_r, scale),
            Screen::Photos => placeholder_ui("照片","🖼️",ctx,app_r),
            Screen::Maps => placeholder_ui("地圖","🗺️",ctx,app_r),
            Screen::Clock => placeholder_ui("時鐘","🕐",ctx,app_r),
            Screen::Calendar => placeholder_ui("日曆","📅",ctx,app_r),
            Screen::Reminders => placeholder_ui("提醒事項","✅",ctx,app_r),
            _ => {}
        }
    }

}

// ═══════════════════════════════════════════════
//  eframe::App
// ═══════════════════════════════════════════════

impl eframe::App for Phone4App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_datetime();
        let screen_r = ctx.screen_rect();
        let max_h = screen_r.height()*0.88;
        let max_w = screen_r.width()*0.38;
        let ph = max_h.min(PHONE_H/PHONE_W*max_w);
        let pw = ph*PHONE_W/PHONE_H;
        let scale = pw/PHONE_W;

        let phone_origin = Pos2::new(screen_r.center().x-pw/2.0, screen_r.center().y-ph/2.0);
        let phone_rect = Rect::from_min_size(phone_origin, Vec2::new(pw, ph));
        let screen_area = phone_rect.shrink(BEZEL*scale);

        let p = ctx.debug_painter();

        // 桌面背景
        fill_rect(&p, screen_r, 0.0, C_BG);

        // 手機外殼
        fill_rect(&p, phone_rect, CORNER*scale, Color32::from_rgb(40,40,42));
        stroke_rect(&p, phone_rect, CORNER*scale, Color32::TRANSPARENT, 1.0_f32, Color32::WHITE.linear_multiply(0.15));

        // 側邊按鈕
        let bw=3.0_f32*scale; let bh=60.0_f32*scale; let bc=Color32::from_rgb(80,80,85);
        fill_rect(&p, Rect::from_min_size(Pos2::new(phone_rect.right()+1.0,phone_rect.top()+120.0*scale),Vec2::new(bw,bh)), 1.5, bc);
        for i in 0..2 {
            fill_rect(&p, Rect::from_min_size(Pos2::new(phone_rect.left()-bw-1.0,phone_rect.top()+(160.0+i as f32*80.0)*scale),Vec2::new(bw,bh*0.7)), 1.5, bc);
        }

        // 螢幕背景漸層
        {
            let r = screen_area;
            let top=Color32::from_rgb(50,70,110); let bot=Color32::from_rgb(30,40,70);
            for i in 0..10 {
                let t=i as f32/10.0; let y0=r.top()+r.height()*t; let y1=r.top()+r.height()*(t+1.0/10.0);
                let c=Color32::from_rgb((top[0]as f32*(1.0-t)+bot[0]as f32*t)as u8,
                                        (top[1]as f32*(1.0-t)+bot[1]as f32*t)as u8,
                                        (top[2]as f32*(1.0-t)+bot[2]as f32*t)as u8);
                fill_rect(&p, Rect::from_min_size(Pos2::new(r.left(),y0),Vec2::new(r.width(),y1-y0)), 0.0, c);
            }
        }

        // 狀態列
        let status_r = Rect::from_min_size(screen_area.left_top(), Vec2::new(screen_area.width(), STATUS_H*scale));
        self.draw_status_bar(&p, status_r, scale);

        // ===== 手機螢幕內容（全部 global 座標） =====
        let content_r = Rect::from_min_size(
            Pos2::new(screen_area.left(), screen_area.top()+STATUS_H*scale),
            Vec2::new(screen_area.width(), screen_area.height()-(STATUS_H+DOCK_H+INDICATOR_H)*scale),
        );

        match self.screen {
            Screen::Home => {
                self.render_home(ctx, content_r, scale);
                let dock_r = Rect::from_min_size(
                    Pos2::new(screen_area.left(), content_r.bottom()),
                    Vec2::new(screen_area.width(), DOCK_H*scale),
                );
                self.render_dock(ctx, dock_r, scale);
            }
            _ => {
                let nav_h = 36.0 * scale;
                let nav_r = Rect::from_min_size(content_r.left_top(), Vec2::new(content_r.width(), nav_h));
                let app_r = Rect::from_min_size(
                    Pos2::new(content_r.left(), content_r.top()+nav_h),
                    Vec2::new(content_r.width(), content_r.height()-nav_h),
                );
                self.render_app_ui(ctx, nav_r, app_r, scale, &self.screen.clone());
            }
        }

        // Dynamic Island
        let island_y = phone_rect.top()+BEZEL*scale+10.0*scale;
        let island_r = Rect::from_center_size(Pos2::new(phone_rect.center().x, island_y), Vec2::new(126.0,36.0));
        fill_rect(&p, island_r, island_r.height()/2.0, Color32::from_black_alpha(120));
        p.circle(island_r.center(), 5.0, Color32::from_rgb(60,40,100), egui::Stroke::new(0.0_f32,Color32::TRANSPARENT));
        p.circle(island_r.center(), 2.0, Color32::from_rgb(80,60,120), egui::Stroke::new(0.0_f32,Color32::TRANSPARENT));

        // 主畫面指示器
        let ind_y = phone_rect.bottom()-BEZEL*scale-8.0*scale;
        let ind_r = Rect::from_center_size(Pos2::new(phone_rect.center().x, ind_y), Vec2::new(134.0,5.0));
        fill_rect(&p, ind_r, ind_r.height()/2.0, Color32::WHITE.linear_multiply(0.8));

        // ===== 點擊處理 =====
        // Home 指示器
        let hit_r = Rect::from_center_size(Pos2::new(phone_rect.center().x, ind_y), Vec2::new(134.0*scale,20.0*scale));
        if pointer_in(hit_r, ctx) && ctx.input(|i| i.pointer.any_click()) { self.go_home(); return; }

        // App 圖示（主畫面）
        if self.screen == Screen::Home {
            let dock_r = Rect::from_min_size(
                Pos2::new(screen_area.left(), content_r.bottom()),
                Vec2::new(screen_area.width(), DOCK_H*scale),
            );
            if self.hit_home_icon(ctx, content_r, scale) { return; }
            if self.hit_dock_icon(ctx, dock_r, scale) { return; }
        } else {
            let nav_h = 36.0 * scale;
            let nav_r = Rect::from_min_size(content_r.left_top(), Vec2::new(content_r.width(), nav_h));
            let back_r = Rect::from_min_size(nav_r.left_top()+Vec2::new(4.0*scale,(nav_h-24.0*scale)/2.0), Vec2::new(50.0*scale,24.0*scale));
            if pointer_in(back_r, ctx) && ctx.input(|i| i.pointer.any_click()) {
                self.go_back(); return;
            }
        }
    }
}

impl Phone4App {
    fn draw_status_bar(&self, p: &egui::Painter, status_r: Rect, _scale: f32) {
        let fg = Color32::WHITE;
        p.text(status_r.left_center()+Vec2::new(20.0,0.0), egui::Align2::LEFT_CENTER, &self.time_str,
               egui::FontId::proportional(14.0), fg);

        // 電池
        let bat_c = status_r.right_top()+Vec2::new(-28.0,18.0);
        let bat_w=22.0; let bat_h=10.0;
        let bat_r = Rect::from_center_size(bat_c, Vec2::new(bat_w,bat_h));
        stroke_rect(p, bat_r, 2.0, Color32::TRANSPARENT, 1.5, fg);
        let fw = (bat_w-4.0)*0.75;
        fill_rect(p, Rect::from_min_size(bat_r.left_top()+Vec2::new(2.0,2.0), Vec2::new(fw,bat_h-4.0)), 1.0, fg);
        fill_rect(p, Rect::from_min_size(bat_c+Vec2::new(bat_w/2.0+1.0,-2.0), Vec2::new(3.0,4.0)), 1.0, fg);

        // 訊號
        for i in 0..4 {
            let bar_h = 3.0+i as f32*2.5;
            let x = status_r.right()-60.0+i as f32*7.0;
            fill_rect(p, Rect::from_min_size(Pos2::new(x,bat_c.y-bar_h),Vec2::new(4.0,bar_h)), 1.0,
                      fg.linear_multiply(if i<3{1.0}else{0.35}));
        }
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 860.0])
            .with_title("Phone4 — iPhone 模擬器")
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native("Phone4", options, Box::new(|cc| {
        let mut fonts = egui::FontDefinitions::default();
        let font_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("win4/font/font.ttf");
        if let Ok(data) = std::fs::read(&font_path) {
            fonts.font_data.insert("cjk".into(), egui::FontData::from_owned(data).into());
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "cjk".into());
            fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().insert(0, "cjk".into());
        }
        cc.egui_ctx.set_fonts(fonts);
        Ok(Box::new(Phone4App::new()))
    })).expect("Phone4 啟動失敗");
}
