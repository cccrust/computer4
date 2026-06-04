mod commands;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::app_manager::list_installed_apps,
            commands::app_manager::get_app_manifest,
            commands::app_manager::get_app_entry_url,
            commands::app_manager::execute_shell,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}