use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppManifest {
    #[serde(rename = "appId")]
    pub app_id: String,
    pub name: String,
    pub version: String,
    pub icon: String,
    pub entry: String,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default = "default_orientation")]
    pub orientation: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default = "default_category")]
    pub category: String,
}

fn default_orientation() -> String {
    "portrait".to_string()
}

fn default_category() -> String {
    "other".to_string()
}

impl AppManifest {
    #[allow(dead_code)]
    pub fn icon_path(&self, base_path: &std::path::Path) -> std::path::PathBuf {
        base_path.join(&self.icon)
    }

    #[allow(dead_code)]
    pub fn entry_path(&self, base_path: &std::path::Path) -> std::path::PathBuf {
        base_path.join(&self.entry)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub manifest: AppManifest,
    pub path: String,
    pub is_builtin: bool,
}
