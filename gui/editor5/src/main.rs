use dioxus::prelude::*;
use std::fs;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut current_folder = use_signal(|| Option::<String>::None);
    let mut selected_file = use_signal(|| Option::<String>::None);
    let mut files = use_signal(|| Vec::<String>::new());

    let on_open_folder = move |_| {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            let path = folder.to_string_lossy().to_string();
            *current_folder.write() = Some(path.clone());
            let mut file_list = Vec::new();
            if let Ok(entries) = fs::read_dir(&folder) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.path().to_str() {
                        if !entry.path().is_dir() {
                            file_list.push(name.to_string());
                        }
                    }
                }
            }
            file_list.sort();
            *files.write() = file_list;
        }
    };

    let editor_html = include_str!("../assets/index.html");
    let current_files = files.read().clone();

    let file_items: Vec<VNode> = current_files
        .iter()
        .map(|path| {
            let name = path.split('/').last().unwrap_or(path).to_string();
            let fp = path.clone();
            let dn = name;
            let click = move |_| {
                if let Ok(content) = fs::read_to_string(&fp) {
                    *selected_file.write() = Some(fp.clone());
                }
            };
            rsx! {
                div {
                    display: "flex",
                    align_items: "center",
                    padding: "6px 12px",
                    cursor: "pointer",
                    onmousedown: click,
                    span {
                        color: "#9cdcfe",
                        font_size: "13px",
                        "[FILE] {dn}"
                    }
                }
            }
        })
        .filter_map(|r| r.ok())
        .collect();

    rsx! {
        div {
            height: "100vh",
            display: "flex",
            flex_direction: "column",

            div {
                background_color: "#333333",
                padding: "8px 16px",
                display: "flex",
                align_items: "center",
                gap: "12px",

                button {
                    background_color: "#4CAF50",
                    color: "white",
                    border: "none",
                    padding: "8px 16px",
                    border_radius: "4px",
                    cursor: "pointer",
                    onclick: on_open_folder,
                    "Open Folder"
                }

                if let Some(folder) = current_folder.read().as_ref() {
                    span {
                        color: "#cccccc",
                        font_size: "14px",
                        "Folder: {folder}"
                    }
                }
            }

            div {
                flex: "1",
                display: "flex",
                overflow: "hidden",

                if !current_files.is_empty() {
                    div {
                        width: "280px",
                        background_color: "#1e1e1e",
                        border_right: "1px solid #333",
                        overflow_y: "auto",

                        div {
                            padding: "8px 12px",
                            background_color: "#252526",
                            color: "#cccccc",
                            font_size: "12px",
                            font_weight: "bold",
                            "EXPLORER"
                        }

                        for node in file_items {
                            {node}
                        }
                    }
                }

                div {
                    flex: "1",
                    iframe {
                        src: "data:text/html;charset=utf-8,".to_string() + editor_html,
                        width: "100%",
                        height: "100%",
                        border: "none",
                    }
                }
            }
        }
    }
}