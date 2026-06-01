use eframe::egui;
use std::process::Command;
use crate::buffer::Buffer;

pub struct EditorApp {
    buffers: Vec<Buffer>,
    active: usize,
    cursor_row: usize,
    cursor_col: usize,
    scroll_offset: f32,
    line_number_width: f32,
    show_find: bool,
    find_query: String,
    find_matches: Vec<(usize, usize)>,
    find_index: usize,
    status_msg: String,
}

impl EditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "editor_mono".to_string(),
            egui::FontData::from_static(include_bytes!("/System/Library/Fonts/SFNSMono.ttf")),
        );
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("editor_mono".to_string());
        cc.egui_ctx.set_fonts(fonts);
        Self {
            buffers: vec![Buffer::new()],
            active: 0,
            cursor_row: 0,
            cursor_col: 0,
            scroll_offset: 0.0,
            line_number_width: 0.0,
            show_find: false,
            find_query: String::new(),
            find_matches: Vec::new(),
            find_index: 0,
            status_msg: String::new(),
        }
    }

    fn buf(&self) -> &Buffer { &self.buffers[self.active] }
    fn buf_mut(&mut self) -> &mut Buffer { &mut self.buffers[self.active] }

    fn clamp_cursor(&mut self) {
        let rows = self.buf().line_count();
        if self.cursor_row >= rows { self.cursor_row = rows.saturating_sub(1); }
        let max_col = self.buf().line_len(self.cursor_row);
        if self.cursor_col > max_col { self.cursor_col = max_col; }
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Ok(buf) = Buffer::load(path.to_string_lossy().as_ref()) {
                self.buffers.push(buf);
                self.active = self.buffers.len() - 1;
                self.cursor_row = 0;
                self.cursor_col = 0;
                self.scroll_offset = 0.0;
                self.status_msg = format!("opened: {}", path.display());
            }
        }
    }
    fn save_file(&mut self) {
        if self.buf().filepath().is_some() {
            match self.buf_mut().save() {
                Ok(()) => self.status_msg = "saved".to_string(),
                Err(e) => self.status_msg = format!("save error: {e}"),
            }
        } else { self.save_file_as(); }
    }
    fn save_file_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new().save_file() {
            match self.buf_mut().save_as(path.to_string_lossy().as_ref()) {
                Ok(()) => self.status_msg = format!("saved: {}", path.display()),
                Err(e) => self.status_msg = format!("save error: {e}"),
            }
        }
    }
    fn close_tab(&mut self) {
        if self.buffers.len() > 1 {
            self.buffers.remove(self.active);
            if self.active >= self.buffers.len() { self.active = self.buffers.len() - 1; }
            self.clamp_cursor();
        }
    }
    fn new_tab(&mut self) { self.buffers.push(Buffer::new()); self.active = self.buffers.len() - 1; self.cursor_row = 0; self.cursor_col = 0; self.scroll_offset = 0.0; }
    fn new_terminal_tab(&mut self) { self.buffers.push(Buffer::new_terminal()); self.active = self.buffers.len() - 1; self.cursor_row = 0; self.cursor_col = 0; self.scroll_offset = 0.0; }
    fn find(&mut self) {
        self.find_matches.clear();
        self.find_index = 0;
        if self.find_query.is_empty() { return; }
        let query = self.find_query.to_lowercase();
        let lines: Vec<String> = self.buf().lines.clone();
        for (row, line) in lines.iter().enumerate() {
            let lower = line.to_lowercase();
            let mut col = 0;
            while let Some(pos) = lower[col..].find(&query) {
                self.find_matches.push((row, col + pos));
                col += pos + 1;
            }
        }
        if let Some(&(r,c)) = self.find_matches.first() { self.cursor_row = r; self.cursor_col = c; }
    }
    fn find_next(&mut self) {
        if self.find_matches.is_empty() { return; }
        self.find_index = (self.find_index + 1) % self.find_matches.len();
        let (r,c) = self.find_matches[self.find_index];
        self.cursor_row = r; self.cursor_col = c;
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let line_height = 18.0;
        let line_num_gutter = 50.0;
        self.show_top_menu(ctx);
        self.show_tab_bar(ctx);
        let bottom_height = 24.0;
        let available = ctx.available_rect();
        let editor_rect = available.shrink2(egui::vec2(0.0, bottom_height));
        self.show_editor(ctx, editor_rect, line_height, line_num_gutter);
        self.show_status_bar(ctx, bottom_height);
        if self.show_find { self.show_find_bar(ctx); }
        ctx.request_repaint();
    }
}

impl EditorApp {
    fn show_top_menu(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Tab (Ctrl+N)").clicked() { self.new_tab(); ui.close_menu(); }
                    if ui.button("New Terminal (Ctrl+T)").clicked() { self.new_terminal_tab(); ui.close_menu(); }
                    if ui.button("Open... (Ctrl+O)").clicked() { self.open_file(); ui.close_menu(); }
                    if ui.button("Save (Ctrl+S)").clicked() { self.save_file(); ui.close_menu(); }
                    if ui.button("Save As... (Ctrl+Shift+S)").clicked() { self.save_file_as(); ui.close_menu(); }
                    if ui.button("Close Tab (Ctrl+W)").clicked() { self.close_tab(); ui.close_menu(); }
                    if ui.button("Quit").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); ui.close_menu(); }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Find... (Ctrl+F)").clicked() { self.show_find = true; ui.close_menu(); }
                });
            });
        });
    }
    fn show_tab_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut switch_to = None;
                let mut close_idx = None;
                for (i, buf) in self.buffers.iter().enumerate() {
                    let label = if buf.is_modified() { format!(" {}* ", buf.filename()) } else { format!(" {} ", buf.filename()) };
                    let is_active = i == self.active;
                    let response = ui.selectable_label(is_active, &label);
                    if response.clicked() { switch_to = Some(i); }
                    if response.hovered() && ui.input(|r| r.pointer.secondary_clicked()) { close_idx = Some(i); }
                }
                if let Some(idx) = switch_to { self.active = idx; self.clamp_cursor(); }
                if let Some(idx) = close_idx {
                    self.buffers.remove(idx);
                    if self.buffers.is_empty() { self.buffers.push(Buffer::new()); }
                    if self.active >= self.buffers.len() { self.active = self.buffers.len() - 1; }
                    self.clamp_cursor();
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(" + ").clicked() { self.new_tab(); }
                });
            });
        });
    }
    fn show_editor(&mut self, ctx: &egui::Context, rect: egui::Rect, line_height: f32, line_num_gutter: f32) {
        let line_count = self.buf().line_count();
        let total_height = line_count as f32 * line_height;
        let visible_lines = ((rect.height()) / line_height).ceil() as usize;
        let max_scroll = (total_height - rect.height()).max(0.0);
        // movement
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) { if self.cursor_row + 1 < line_count { self.cursor_row += 1; } self.clamp_cursor(); }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) { self.cursor_row = self.cursor_row.saturating_sub(1); self.clamp_cursor(); }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) { if self.cursor_col > 0 { self.cursor_col -= 1; } else if self.cursor_row > 0 { self.cursor_row -= 1; self.cursor_col = self.buf().line_len(self.cursor_row); } }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) { let max_col = self.buf().line_len(self.cursor_row); if self.cursor_col < max_col { self.cursor_col += 1; } else if self.cursor_row + 1 < line_count { self.cursor_row += 1; self.cursor_col = 0; } }
        if ctx.input(|i| i.key_pressed(egui::Key::Home)) { self.cursor_col = 0; }
        if ctx.input(|i| i.key_pressed(egui::Key::End)) { self.cursor_col = self.buf().line_len(self.cursor_row); }
        if ctx.input(|i| i.key_pressed(egui::Key::PageUp)) { self.cursor_row = self.cursor_row.saturating_sub(visible_lines); self.clamp_cursor(); }
        if ctx.input(|i| i.key_pressed(egui::Key::PageDown)) { self.cursor_row = (self.cursor_row + visible_lines).min(line_count - 1); self.clamp_cursor(); }
        // shortcuts
        let ctrl = ctx.input(|i| i.modifiers.ctrl);
        let shift = ctx.input(|i| i.modifiers.shift);
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::S)) { if shift { self.save_file_as(); } else { self.save_file(); } }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::O)) { self.open_file(); }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::N)) { self.new_tab(); }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::W)) { self.close_tab(); }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::T)) { self.new_terminal_tab(); }
        if ctrl && ctx.input(|i| i.key_pressed(egui::Key::F)) { self.show_find = !self.show_find; if self.show_find { self.find_query.clear(); self.find_matches.clear(); } }
        if self.show_find && ctx.input(|i| i.key_pressed(egui::Key::Enter)) { self.find_next(); }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) { self.show_find = false; }
        // scrolling
        let cursor_screen_y = self.cursor_row as f32 * line_height;
        if cursor_screen_y < self.scroll_offset { self.scroll_offset = cursor_screen_y; }
        if cursor_screen_y > self.scroll_offset + rect.height() - line_height { self.scroll_offset = cursor_screen_y - rect.height() + line_height; }
        self.scroll_offset = self.scroll_offset.clamp(0.0, max_scroll);
        let scroll_delta = ctx.input(|i| i.raw_scroll_delta.y);
        self.scroll_offset = (self.scroll_offset - scroll_delta).clamp(0.0, max_scroll);
        let first_visible = (self.scroll_offset / line_height) as usize;
        // drawing
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Middle, egui::Id::new("editor_canvas")));
        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(30,30,30));
        let line_num_area = egui::Rect::from_min_max(rect.min, egui::pos2(rect.min.x + line_num_gutter, rect.max.y));
        painter.rect_filled(line_num_area, 0.0, egui::Color32::from_rgb(40,40,40));
        let font_id = egui::FontId::monospace(14.0);
        let char_width = 8.4;
        for i in 0..=visible_lines {
            let row = first_visible + i;
            if row >= line_count { break; }
            let y = rect.min.y + (row as f32) * line_height - self.scroll_offset;
            let line_num = format!("{:>4}", row + 1);
            painter.text(egui::pos2(rect.min.x + 4.0, y), egui::Align2::LEFT_TOP, &line_num, font_id.clone(), egui::Color32::from_rgb(120,120,120));
            let line = self.buf().line(row);
            let text_x = rect.min.x + line_num_gutter;
            painter.text(egui::pos2(text_x, y), egui::Align2::LEFT_TOP, line, font_id.clone(), egui::Color32::from_rgb(212,212,212));
        }
        // cursor
        if first_visible <= self.cursor_row && self.cursor_row <= first_visible + visible_lines {
            let y = rect.min.y + self.cursor_row as f32 * line_height - self.scroll_offset;
            let x = rect.min.x + line_num_gutter + self.cursor_col as f32 * char_width;
            if ctx.input(|i| i.time) % 1.0 < 0.5 {
                painter.line_segment([egui::pos2(x,y), egui::pos2(x, y+line_height)], egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(200,200,200)));
            }
            let hl = egui::Rect::from_min_max(egui::pos2(rect.min.x + line_num_gutter, y), egui::pos2(rect.max.x, y+line_height));
            painter.rect_filled(hl, 0.0, egui::Color32::from_rgba_premultiplied(40,40,80,30));
        }
        // find highlights
        for &(r,c) in &self.find_matches {
            if r >= first_visible && r <= first_visible + visible_lines {
                let y = rect.min.y + r as f32 * line_height - self.scroll_offset;
                let x = rect.min.x + line_num_gutter + c as f32 * char_width;
                let w = self.find_query.chars().count() as f32 * char_width;
                let hl = egui::Rect::from_min_max(egui::pos2(x,y), egui::pos2(x+w, y+line_height));
                painter.rect_filled(hl, 0.0, egui::Color32::from_rgba_premultiplied(255,200,0,40));
            }
        }
        // event handling when not in find mode
        if !self.show_find {
            let events = ctx.input(|i| i.events.clone());
            for ev in &events {
                match ev {
                    egui::Event::Key { key: egui::Key::Backspace, pressed: true, .. } => {
                        let row = self.cursor_row;
let col = self.cursor_col;
if let Some((r,c)) = self.buf_mut().backspace(row, col) { self.cursor_row = r; self.cursor_col = c; }
                    }
                    egui::Event::Key { key: egui::Key::Delete, pressed: true, .. } => {
                        let row = self.cursor_row;
let col = self.cursor_col;
if let Some((r,c)) = self.buf_mut().delete(row, col) { self.cursor_row = r; self.cursor_col = c; }
                    }
                    egui::Event::Key { key: egui::Key::Enter, pressed: true, modifiers, .. } => {
                        if !modifiers.ctrl {
                            if self.buf().is_terminal() {
                                let prompt = ">>> ";
                                let cmd_line = self.buf().line(self.cursor_row).to_string();
                                let command = if cmd_line.starts_with(prompt) { &cmd_line[prompt.len()..] } else { &cmd_line[..] };
                                let output = Command::new("sh").arg("-c").arg(command).output();
                                let mut insert_idx = self.cursor_row + 1;
                                if let Ok(out) = output {
                                    let stdout = String::from_utf8_lossy(&out.stdout);
                                    let stderr = String::from_utf8_lossy(&out.stderr);
                                    for line in stdout.lines() { self.buf_mut().lines.insert(insert_idx, line.to_string()); insert_idx += 1; }
                                    for line in stderr.lines() { self.buf_mut().lines.insert(insert_idx, line.to_string()); insert_idx += 1; }
                                } else {
                                    self.buf_mut().lines.insert(insert_idx, "Failed to run command".to_string()); insert_idx += 1;
                                }
                                self.buf_mut().lines.insert(insert_idx, prompt.to_string());
                                self.cursor_row = insert_idx;
                                self.cursor_col = prompt.len();
                            } else {
                                let row = self.cursor_row;
let col = self.cursor_col;
self.buf_mut().insert_newline(row, col);
                                self.cursor_row += 1; self.cursor_col = 0;
                            }
                        }
                    }
                    egui::Event::Text(text) => {
                        let row = self.cursor_row;
                        let col = self.cursor_col;
                        let (r, c) = self.buf_mut().insert_text(text, row, col);
                        self.cursor_row = r; self.cursor_col = c;
                    }
                    _ => {}
                }
            }
        }
        // gutter separator line
        let gutter_sep = egui::pos2(rect.min.x + line_num_gutter - 2.0, rect.min.y);
        let gutter_end = egui::pos2(rect.min.x + line_num_gutter - 2.0, rect.max.y);
        painter.line_segment([gutter_sep, gutter_end], egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(60,60,60)));
    }
    fn show_status_bar(&self, ctx: &egui::Context, height: f32) {
        egui::TopBottomPanel::bottom("status_bar").exact_height(height).show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let info = format!("Ln {}, Col {}  |  {}  |  {}",
                    self.cursor_row + 1,
                    self.cursor_col + 1,
                    self.buf().filename(),
                    if self.status_msg.is_empty() { "Ready" } else { &self.status_msg }
                );
                ui.label(egui::RichText::new(info).monospace().size(12.0));
            });
        });
    }
    fn show_find_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("find_bar").exact_height(32.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Find:");
                let response = ui.add(egui::TextEdit::singleline(&mut self.find_query).desired_width(200.0).font(egui::FontId::monospace(14.0)));
                if ui.button("Next").clicked() || response.lost_focus() { self.find(); }
                if ui.button("Prev").clicked() {
                    self.find();
                    if !self.find_matches.is_empty() {
                        if self.find_index == 0 { self.find_index = self.find_matches.len() - 1; } else { self.find_index -= 1; }
                        let (r,c) = self.find_matches[self.find_index]; self.cursor_row = r; self.cursor_col = c;
                    }
                }
                let count = if self.find_matches.is_empty() { "No matches".to_string() } else { format!("{}/{}", self.find_index+1, self.find_matches.len()) };
                ui.label(&count);
                if ui.button("X").clicked() { self.show_find = false; self.find_matches.clear(); }
            });
        });
    }
}
