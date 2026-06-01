use eframe::egui;

mod buffer;
mod editor;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("editor4"),
        ..Default::default()
    };
    eframe::run_native(
        "editor4",
        options,
        Box::new(|cc| {
            Ok(Box::new(editor::EditorApp::new(cc)))
        }),
    )
}
