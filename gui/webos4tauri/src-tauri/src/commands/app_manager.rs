use std::path::PathBuf;
use crate::models::app::{AppInfo, AppManifest};

const BUILTIN_APPS: &[(&str, &str, &str, &str, &str, &[&str])] = &[
    ("com.webos.fileexplorer", "File Explorer", "folder-open", "index.html", "File manager & storage browser", &["fs:read", "fs:write"]),
    ("com.webos.terminal",    "Terminal",       "terminal",    "index.html", "Command line shell",            &["shell:exec"]),
    ("com.webos.editor",      "Editor",         "file-code",   "index.html", "Code & text editor",            &["fs:read", "fs:write"]),
    ("com.webos.browser",     "Browser",        "globe",       "index.html", "Web browser",                   &["network"]),
];

fn get_builtin_apps_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.join("src").join("apps")
}

fn get_apps_data_dir(app_handle: &tauri::AppHandle) -> PathBuf {
    let config_dir = tauri::Manager::path(app_handle)
        .app_config_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    config_dir.join("apps")
}

#[tauri::command]
pub fn list_installed_apps(app_handle: tauri::AppHandle) -> Result<Vec<AppInfo>, String> {
    let mut apps: Vec<AppInfo> = Vec::new();

    for (app_id, name, icon, entry, description, permissions) in BUILTIN_APPS {
        let app_path = get_builtin_apps_path().join(app_id.split('.').last().unwrap_or("unknown"));
        let manifest = AppManifest {
            app_id: app_id.to_string(),
            name: name.to_string(),
            version: "0.1.0".to_string(),
            icon: icon.to_string(),
            entry: entry.to_string(),
            permissions: permissions.iter().map(|s| s.to_string()).collect(),
            orientation: "portrait".to_string(),
            description: description.to_string(),
            author: "webOS".to_string(),
            category: "utilities".to_string(),
        };
        apps.push(AppInfo {
            manifest,
            path: app_path.to_string_lossy().to_string(),
            is_builtin: true,
        });
    }

    let apps_dir = get_apps_data_dir(&app_handle);
    if apps_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&apps_dir) {
            for entry in entries.flatten() {
                let app_dir = entry.path();
                if app_dir.is_dir() {
                    let manifest_path = app_dir.join("manifest.json");
                    if manifest_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                            if let Ok(manifest) = serde_json::from_str::<AppManifest>(&content) {
                                apps.push(AppInfo {
                                    manifest,
                                    path: app_dir.to_string_lossy().to_string(),
                                    is_builtin: false,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(apps)
}

#[tauri::command]
pub fn get_app_manifest(app_id: String) -> Result<AppInfo, String> {
    for (aid, name, icon, entry, description, permissions) in BUILTIN_APPS {
        if *aid == app_id {
            let app_path = get_builtin_apps_path().join(aid.split('.').last().unwrap_or("unknown"));
            let manifest = AppManifest {
                app_id: aid.to_string(),
                name: name.to_string(),
                version: "0.1.0".to_string(),
                icon: icon.to_string(),
                entry: entry.to_string(),
                permissions: permissions.iter().map(|s| s.to_string()).collect(),
                orientation: "portrait".to_string(),
                description: description.to_string(),
                author: "webOS".to_string(),
                category: "utilities".to_string(),
            };
            return Ok(AppInfo {
                manifest,
                path: app_path.to_string_lossy().to_string(),
                is_builtin: true,
            });
        }
    }
    Err(format!("App {} not found", app_id))
}

#[tauri::command]
pub fn get_app_entry_url(app_id: String) -> Result<String, String> {
    let manifest_path = get_builtin_apps_path()
        .join(app_id.split('.').last().unwrap_or("unknown"))
        .join("manifest.json");

    if manifest_path.exists() {
        let content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;
        let manifest: AppManifest = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse manifest: {}", e))?;

        let entry_url = format!("/apps/{}/{}",
            app_id.split('.').last().unwrap_or("unknown"),
            manifest.entry.trim_start_matches('/'));
        return Ok(entry_url);
    }

    Ok(format!("/apps/{}/index.html", app_id.split('.').last().unwrap_or("unknown")))
}

#[tauri::command]
pub fn execute_shell(command: String) -> Result<String, String> {
    use std::process::Command as StdCommand;

    let output = StdCommand::new("bash")
        .args(["-c", &command])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(format!("{}{}", stdout, stderr))
    }
}