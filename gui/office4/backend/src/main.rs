use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

// ═══════════════════════════════════════════════════════════
//  SPREADSHEET MODEL
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cell {
    value: String,
    formula: Option<String>,
    #[serde(rename = "type")]
    cell_type: CellType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum CellType { Text, Number, Formula, Empty }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SpreadsheetState {
    cells: HashMap<String, Cell>,
    rows: usize,
    cols: usize,
    name: String,
}

impl SpreadsheetState {
    fn new() -> Self {
        SpreadsheetState { cells: HashMap::new(), rows: 50, cols: 26, name: "試算表".into() }
    }
}

// ═══════════════════════════════════════════════════════════
//  SLIDES MODEL
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TextElement {
    id: String,
    x: f64, y: f64, w: f64, h: f64,
    content: String,
    font_size: u32,
    font_bold: bool,
    font_italic: bool,
    color: String,
    align: String,       // left / center / right
    z_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShapeElement {
    id: String,
    x: f64, y: f64, w: f64, h: f64,
    shape: String,       // rect / ellipse / triangle / line
    fill: String,
    stroke: String,
    stroke_width: f64,
    z_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImageElement {
    id: String,
    x: f64, y: f64, w: f64, h: f64,
    src: String,         // data URL or external URL
    z_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Slide {
    id: String,
    background: String,  // CSS color or gradient
    texts: Vec<TextElement>,
    shapes: Vec<ShapeElement>,
    images: Vec<ImageElement>,
    notes: String,
}

impl Slide {
    fn new(id: String) -> Self {
        Slide {
            id,
            background: "#1a1e2a".into(),
            texts: vec![],
            shapes: vec![],
            images: vec![],
            notes: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PresentationState {
    slides: Vec<Slide>,
    name: String,
    current_slide: usize,
    theme: String,
}

impl PresentationState {
    fn new() -> Self {
        let mut s = Self {
            slides: vec![],
            name: "投影片".into(),
            current_slide: 0,
            theme: "dark".into(),
        };
        // Default first slide
        let mut slide = Slide::new("slide-0".into());
        slide.background = "#0d1117".into();
        slide.texts.push(TextElement {
            id: "t0".into(), x: 80.0, y: 180.0, w: 760.0, h: 100.0,
            content: "Office 4".into(),
            font_size: 56, font_bold: true, font_italic: false,
            color: "#e2e8f0".into(), align: "center".into(), z_index: 1,
        });
        slide.texts.push(TextElement {
            id: "t1".into(), x: 80.0, y: 290.0, w: 760.0, h: 60.0,
            content: "點擊文字開始編輯".into(),
            font_size: 24, font_bold: false, font_italic: false,
            color: "#64748b".into(), align: "center".into(), z_index: 1,
        });
        s.slides.push(slide);
        s
    }
}

// ═══════════════════════════════════════════════════════════
//  CLIENT MESSAGES
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMessage {
    // ── Spreadsheet ──
    UpdateCell { row: usize, col: usize, value: String, formula: Option<String> },
    ClearCell  { row: usize, col: usize },
    ClearAll,
    AddRows    { count: usize },
    RenameSheet{ name: String },
    GetState,

    // ── Slides ──
    GetSlides,
    AddSlide,
    DeleteSlide   { slide_id: String },
    ReorderSlides { order: Vec<String> },
    SetBackground { slide_id: String, background: String },
    RenamePresentation { name: String },

    AddText   { slide_id: String, element: TextElement },
    UpdateText{ slide_id: String, element: TextElement },
    DeleteText{ slide_id: String, element_id: String },

    AddShape   { slide_id: String, element: ShapeElement },
    UpdateShape{ slide_id: String, element: ShapeElement },
    DeleteShape{ slide_id: String, element_id: String },

    AddImage   { slide_id: String, element: ImageElement },
    UpdateImage{ slide_id: String, element: ImageElement },
    DeleteImage{ slide_id: String, element_id: String },

    SetCurrentSlide { index: usize },
    UpdateNotes     { slide_id: String, notes: String },
}

// ═══════════════════════════════════════════════════════════
//  SERVER MESSAGES
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ServerMessage {
    // ── Spreadsheet ──
    StateSnapshot      { state: SpreadsheetState, client_id: String, client_count: usize },
    CellUpdated        { row: usize, col: usize, cell: Cell, client_id: String },
    CellCleared        { row: usize, col: usize, client_id: String },
    AllCleared,
    RowsAdded          { new_rows: usize },
    SheetRenamed       { name: String },

    // ── Slides ──
    SlidesSnapshot     { state: PresentationState, client_id: String, client_count: usize },
    SlideAdded         { slide: Slide },
    SlideDeleted       { slide_id: String },
    SlidesReordered    { order: Vec<String> },
    BackgroundSet      { slide_id: String, background: String },
    PresentationRenamed{ name: String },

    TextAdded    { slide_id: String, element: TextElement },
    TextUpdated  { slide_id: String, element: TextElement },
    TextDeleted  { slide_id: String, element_id: String },

    ShapeAdded   { slide_id: String, element: ShapeElement },
    ShapeUpdated { slide_id: String, element: ShapeElement },
    ShapeDeleted { slide_id: String, element_id: String },

    ImageAdded   { slide_id: String, element: ImageElement },
    ImageUpdated { slide_id: String, element: ImageElement },
    ImageDeleted { slide_id: String, element_id: String },

    CurrentSlideSet { index: usize, client_id: String },
    NotesUpdated    { slide_id: String, notes: String },

    // ── System ──
    ClientJoined { client_count: usize, client_id: String },
    ClientLeft   { client_count: usize },
    Error        { message: String },
}

// ═══════════════════════════════════════════════════════════
//  SHARED STATE
// ═══════════════════════════════════════════════════════════

type Tx      = tokio::sync::mpsc::UnboundedSender<Message>;
type Clients = Arc<Mutex<HashMap<String, Tx>>>;
type Sheet   = Arc<Mutex<SpreadsheetState>>;
type Pres    = Arc<Mutex<PresentationState>>;

// ═══════════════════════════════════════════════════════════
//  FORMULA ENGINE  (unchanged from Celeris)
// ═══════════════════════════════════════════════════════════

fn evaluate_formula(formula: &str, cells: &HashMap<String, Cell>) -> String {
    let formula = formula.trim();
    if !formula.starts_with('=') { return formula.to_string(); }
    let expr = formula[1..].trim().to_uppercase();

    if expr.starts_with("SUM(") && expr.ends_with(')') {
        return eval_range_op(&expr[4..expr.len()-1], cells, |v| v.iter().sum()).to_string();
    }
    if expr.starts_with("AVG(") && expr.ends_with(')') {
        let v = collect_range(&expr[4..expr.len()-1], cells);
        return if v.is_empty() { "0".into() } else { fmt_f(v.iter().sum::<f64>() / v.len() as f64) };
    }
    if expr.starts_with("MAX(") && expr.ends_with(')') {
        return eval_range_op(&expr[4..expr.len()-1], cells, |v| v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)).to_string();
    }
    if expr.starts_with("MIN(") && expr.ends_with(')') {
        return eval_range_op(&expr[4..expr.len()-1], cells, |v| v.iter().cloned().fold(f64::INFINITY, f64::min)).to_string();
    }
    if expr.starts_with("COUNT(") && expr.ends_with(')') {
        return collect_range(&expr[6..expr.len()-1], cells).len().to_string();
    }
    eval_arithmetic(&resolve_refs(&expr, cells))
}

fn eval_range_op(range: &str, cells: &HashMap<String, Cell>, f: impl Fn(&Vec<f64>) -> f64) -> f64 {
    let v = collect_range(range, cells); f(&v)
}

fn collect_range(range: &str, cells: &HashMap<String, Cell>) -> Vec<f64> {
    let mut out = vec![];
    if let Some((a, b)) = range.split_once(':') {
        if let (Some((r1,c1)), Some((r2,c2))) = (parse_ref(a), parse_ref(b)) {
            for r in r1..=r2 { for c in c1..=c2 {
                if let Some(cell) = cells.get(&format!("{}:{}", r, c)) {
                    if let Ok(v) = cell.value.parse::<f64>() { out.push(v); }
                }
            }}
        }
    } else if let Some((r,c)) = parse_ref(range) {
        if let Some(cell) = cells.get(&format!("{}:{}", r, c)) {
            if let Ok(v) = cell.value.parse::<f64>() { out.push(v); }
        }
    }
    out
}

fn parse_ref(s: &str) -> Option<(usize, usize)> {
    let s = s.trim();
    let ci = s.find(|c: char| c.is_ascii_digit())?;
    let col_s = &s[..ci]; let row_s = &s[ci..];
    let mut col: usize = 0;
    for c in col_s.chars() { col = col * 26 + (c as usize - 'A' as usize + 1); }
    let row: usize = row_s.parse::<usize>().ok()?.checked_sub(1)?;
    if col == 0 { return None; }
    Some((row, col - 1))
}

fn resolve_refs(expr: &str, cells: &HashMap<String, Cell>) -> String {
    let chars: Vec<char> = expr.chars().collect();
    let mut out = String::new(); let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_alphabetic() {
            let mut j = i; while j < chars.len() && chars[j].is_ascii_alphabetic() { j += 1; }
            let mut k = j; while k < chars.len() && chars[k].is_ascii_digit()      { k += 1; }
            if k > j {
                let ref_s: String = chars[i..k].iter().collect();
                if let Some((r,c)) = parse_ref(&ref_s) {
                    let v = cells.get(&format!("{}:{}", r, c)).map(|x| x.value.clone()).unwrap_or("0".into());
                    out.push_str(&v); i = k; continue;
                }
            }
            out.extend(chars[i..j].iter()); i = j;
        } else { out.push(chars[i]); i += 1; }
    }
    out
}

fn eval_arithmetic(s: &str) -> String {
    match parse_add(s.trim()) {
        Ok((v, _)) => fmt_f(v),
        Err(_)     => "#ERR".into(),
    }
}
fn fmt_f(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 { format!("{}", v as i64) }
    else { format!("{:.4}", v).trim_end_matches('0').trim_end_matches('.').to_string() }
}

fn parse_add(s: &str) -> Result<(f64, &str), ()> {
    let (mut l, mut r) = parse_mul(s)?; let mut r = r.trim_start();
    while r.starts_with('+') || r.starts_with('-') {
        let op = &r[..1]; r = r[1..].trim_start();
        let (rhs, nr) = parse_mul(r)?;
        l = if op == "+" { l + rhs } else { l - rhs }; r = nr.trim_start();
    }
    Ok((l, r))
}
fn parse_mul(s: &str) -> Result<(f64, &str), ()> {
    let (mut l, mut r) = parse_unary(s)?; let mut r = r.trim_start();
    while r.starts_with('*') || r.starts_with('/') {
        let op = &r[..1]; r = r[1..].trim_start();
        let (rhs, nr) = parse_unary(r)?;
        l = if op == "*" { l * rhs } else if rhs != 0.0 { l / rhs } else { return Err(()); };
        r = nr.trim_start();
    }
    Ok((l, r))
}
fn parse_unary(s: &str) -> Result<(f64, &str), ()> {
    let s = s.trim_start();
    if s.starts_with('-') { let (v, r) = parse_primary(&s[1..])?; return Ok((-v, r)); }
    parse_primary(s)
}
fn parse_primary(s: &str) -> Result<(f64, &str), ()> {
    let s = s.trim_start();
    if s.starts_with('(') {
        let (v, r) = parse_add(&s[1..])?; let r = r.trim_start();
        if r.starts_with(')') { return Ok((v, &r[1..])); } else { return Err(()); }
    }
    let end = s.find(|c: char| !c.is_ascii_digit() && c != '.').unwrap_or(s.len());
    if end == 0 { return Err(()); }
    Ok((s[..end].parse::<f64>().map_err(|_| ())?, &s[end..]))
}

// ═══════════════════════════════════════════════════════════
//  SLIDE HELPERS
// ═══════════════════════════════════════════════════════════

static SLIDE_CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
static EL_CTR:    std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

fn new_slide_id() -> String {
    format!("slide-{}", SLIDE_CTR.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
}
fn new_el_id(prefix: &str) -> String {
    format!("{}-{}", prefix, EL_CTR.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
}

// ═══════════════════════════════════════════════════════════
//  CLIENT HANDLER
// ═══════════════════════════════════════════════════════════

static CLIENT_CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

async fn handle_client(stream: TcpStream, addr: SocketAddr, clients: Clients, sheet: Sheet, pres: Pres) {
    let client_id = format!("client-{}", CLIENT_CTR.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
    println!("[+] {} [{}]", addr, client_id);

    let ws = match accept_async(stream).await {
        Ok(w) => w, Err(e) => { eprintln!("WS error: {e}"); return; }
    };
    let (mut sink, mut src) = ws.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    clients.lock().unwrap().insert(client_id.clone(), tx.clone());
    let client_count = clients.lock().unwrap().len();

    // Send full state snapshots
    {
        let state = sheet.lock().unwrap().clone();
        send_one(&tx, &ServerMessage::StateSnapshot { state, client_id: client_id.clone(), client_count });
        let pstate = pres.lock().unwrap().clone();
        send_one(&tx, &ServerMessage::SlidesSnapshot { state: pstate, client_id: client_id.clone(), client_count });
    }
    broadcast_except(&clients, &client_id, &ServerMessage::ClientJoined { client_count, client_id: client_id.clone() });

    let send_task = tokio::spawn(async move {
        while let Some(m) = rx.recv().await { if sink.send(m).await.is_err() { break; } }
    });

    while let Some(Ok(Message::Text(text))) = src.next().await {
        if let Ok(msg) = serde_json::from_str::<ClientMessage>(&text) {
            dispatch(msg, &client_id, &clients, &sheet, &pres).await;
        }
    }

    send_task.abort();
    clients.lock().unwrap().remove(&client_id);
    let client_count = clients.lock().unwrap().len();
    broadcast_except(&clients, &client_id, &ServerMessage::ClientLeft { client_count });
    println!("[-] {} [{}]", addr, client_id);
}

fn send_one(tx: &Tx, msg: &ServerMessage) {
    let _ = tx.send(Message::Text(serde_json::to_string(msg).unwrap()));
}

fn broadcast_all(clients: &Clients, msg: &ServerMessage) {
    let text = serde_json::to_string(msg).unwrap();
    for tx in clients.lock().unwrap().values() {
        let _ = tx.send(Message::Text(text.clone()));
    }
}

fn broadcast_except(clients: &Clients, exclude: &str, msg: &ServerMessage) {
    let text = serde_json::to_string(msg).unwrap();
    for (id, tx) in clients.lock().unwrap().iter() {
        if id != exclude { let _ = tx.send(Message::Text(text.clone())); }
    }
}

// ═══════════════════════════════════════════════════════════
//  MESSAGE DISPATCH
// ═══════════════════════════════════════════════════════════

async fn dispatch(msg: ClientMessage, cid: &str, clients: &Clients, sheet: &Sheet, pres: &Pres) {
    match msg {

        // ── Spreadsheet ──────────────────────────────────
        ClientMessage::UpdateCell { row, col, value, formula } => {
            let computed = if let Some(ref f) = formula {
                let cells = sheet.lock().unwrap().cells.clone();
                evaluate_formula(f, &cells)
            } else { value.clone() };
            let ct = if formula.is_some() { CellType::Formula }
                     else if value.parse::<f64>().is_ok() { CellType::Number }
                     else if value.is_empty() { CellType::Empty }
                     else { CellType::Text };
            let cell = Cell { value: computed, formula, cell_type: ct };
            sheet.lock().unwrap().cells.insert(format!("{}:{}", row, col), cell.clone());
            broadcast_all(clients, &ServerMessage::CellUpdated { row, col, cell, client_id: cid.into() });
        }
        ClientMessage::ClearCell { row, col } => {
            sheet.lock().unwrap().cells.remove(&format!("{}:{}", row, col));
            broadcast_all(clients, &ServerMessage::CellCleared { row, col, client_id: cid.into() });
        }
        ClientMessage::ClearAll => {
            sheet.lock().unwrap().cells.clear();
            broadcast_all(clients, &ServerMessage::AllCleared);
        }
        ClientMessage::AddRows { count } => {
            let nr = { let mut s = sheet.lock().unwrap(); s.rows += count; s.rows };
            broadcast_all(clients, &ServerMessage::RowsAdded { new_rows: nr });
        }
        ClientMessage::RenameSheet { name } => {
            sheet.lock().unwrap().name = name.clone();
            broadcast_all(clients, &ServerMessage::SheetRenamed { name });
        }
        ClientMessage::GetState => {
            let state = sheet.lock().unwrap().clone();
            let cc = clients.lock().unwrap().len();
            if let Some(tx) = clients.lock().unwrap().get(cid) {
                send_one(tx, &ServerMessage::StateSnapshot { state, client_id: cid.into(), client_count: cc });
            }
        }

        // ── Slides ───────────────────────────────────────
        ClientMessage::GetSlides => {
            let state = pres.lock().unwrap().clone();
            let cc = clients.lock().unwrap().len();
            if let Some(tx) = clients.lock().unwrap().get(cid) {
                send_one(tx, &ServerMessage::SlidesSnapshot { state, client_id: cid.into(), client_count: cc });
            }
        }
        ClientMessage::AddSlide => {
            let slide = Slide::new(new_slide_id());
            let s = slide.clone();
            pres.lock().unwrap().slides.push(slide);
            broadcast_all(clients, &ServerMessage::SlideAdded { slide: s });
        }
        ClientMessage::DeleteSlide { slide_id } => {
            let mut p = pres.lock().unwrap();
            p.slides.retain(|s| s.id != slide_id);
            if p.current_slide >= p.slides.len() && !p.slides.is_empty() {
                p.current_slide = p.slides.len() - 1;
            }
            drop(p);
            broadcast_all(clients, &ServerMessage::SlideDeleted { slide_id });
        }
        ClientMessage::ReorderSlides { order } => {
            let mut p = pres.lock().unwrap();
            let map: HashMap<String,Slide> = p.slides.drain(..).map(|s| (s.id.clone(), s)).collect();
            for id in &order { if let Some(s) = map.get(id) { p.slides.push(s.clone()); } }
            drop(p);
            broadcast_all(clients, &ServerMessage::SlidesReordered { order });
        }
        ClientMessage::SetBackground { slide_id, background } => {
            if let Some(s) = pres.lock().unwrap().slides.iter_mut().find(|s| s.id == slide_id) {
                s.background = background.clone();
            }
            broadcast_all(clients, &ServerMessage::BackgroundSet { slide_id, background });
        }
        ClientMessage::RenamePresentation { name } => {
            pres.lock().unwrap().name = name.clone();
            broadcast_all(clients, &ServerMessage::PresentationRenamed { name });
        }
        ClientMessage::SetCurrentSlide { index } => {
            pres.lock().unwrap().current_slide = index;
            broadcast_all(clients, &ServerMessage::CurrentSlideSet { index, client_id: cid.into() });
        }
        ClientMessage::UpdateNotes { slide_id, notes } => {
            if let Some(s) = pres.lock().unwrap().slides.iter_mut().find(|s| s.id == slide_id) {
                s.notes = notes.clone();
            }
            broadcast_all(clients, &ServerMessage::NotesUpdated { slide_id, notes });
        }

        // text
        ClientMessage::AddText { slide_id, mut element } => {
            element.id = new_el_id("t");
            if let Some(s) = pres.lock().unwrap().slides.iter_mut().find(|s| s.id == slide_id) {
                s.texts.push(element.clone());
            }
            broadcast_all(clients, &ServerMessage::TextAdded { slide_id, element });
        }
        ClientMessage::UpdateText { slide_id, element } => {
            if let Some(s) = pres.lock().unwrap().slides.iter_mut().find(|s| s.id == slide_id) {
                if let Some(t) = s.texts.iter_mut().find(|t| t.id == element.id) { *t = element.clone(); }
            }
            broadcast_all(clients, &ServerMessage::TextUpdated { slide_id, element });
        }
        ClientMessage::DeleteText { slide_id, element_id } => {
            if let Some(s) = pres.lock().unwrap().slides.iter_mut().find(|s| s.id == slide_id) {
                s.texts.retain(|t| t.id != element_id);
            }
            broadcast_all(clients, &ServerMessage::TextDeleted { slide_id, element_id });
        }

        // shape
        ClientMessage::AddShape { slide_id, mut element } => {
            element.id = new_el_id("sh");
            if let Some(s) = pres.lock().unwrap().slides.iter_mut().find(|s| s.id == slide_id) {
                s.shapes.push(element.clone());
            }
            broadcast_all(clients, &ServerMessage::ShapeAdded { slide_id, element });
        }
        ClientMessage::UpdateShape { slide_id, element } => {
            if let Some(s) = pres.lock().unwrap().slides.iter_mut().find(|s| s.id == slide_id) {
                if let Some(sh) = s.shapes.iter_mut().find(|sh| sh.id == element.id) { *sh = element.clone(); }
            }
            broadcast_all(clients, &ServerMessage::ShapeUpdated { slide_id, element });
        }
        ClientMessage::DeleteShape { slide_id, element_id } => {
            if let Some(s) = pres.lock().unwrap().slides.iter_mut().find(|s| s.id == slide_id) {
                s.shapes.retain(|sh| sh.id != element_id);
            }
            broadcast_all(clients, &ServerMessage::ShapeDeleted { slide_id, element_id });
        }

        // image
        ClientMessage::AddImage { slide_id, mut element } => {
            element.id = new_el_id("img");
            if let Some(s) = pres.lock().unwrap().slides.iter_mut().find(|s| s.id == slide_id) {
                s.images.push(element.clone());
            }
            broadcast_all(clients, &ServerMessage::ImageAdded { slide_id, element });
        }
        ClientMessage::UpdateImage { slide_id, element } => {
            if let Some(s) = pres.lock().unwrap().slides.iter_mut().find(|s| s.id == slide_id) {
                if let Some(img) = s.images.iter_mut().find(|img| img.id == element.id) { *img = element.clone(); }
            }
            broadcast_all(clients, &ServerMessage::ImageUpdated { slide_id, element });
        }
        ClientMessage::DeleteImage { slide_id, element_id } => {
            if let Some(s) = pres.lock().unwrap().slides.iter_mut().find(|s| s.id == slide_id) {
                s.images.retain(|img| img.id != element_id);
            }
            broadcast_all(clients, &ServerMessage::ImageDeleted { slide_id, element_id });
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  MAIN
// ═══════════════════════════════════════════════════════════

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:9001";
    let listener = TcpListener::bind(addr).await.expect("bind failed");
    println!("🅾️  Office 4 WebSocket server  ws://{addr}");
    println!("   Modules: Spreadsheet + Slides");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let sheet:   Sheet   = Arc::new(Mutex::new(SpreadsheetState::new()));
    let pres:    Pres    = Arc::new(Mutex::new(PresentationState::new()));

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_client(stream, addr, clients.clone(), sheet.clone(), pres.clone()));
    }
}
