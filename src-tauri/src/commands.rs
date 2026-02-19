
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use zip::ZipArchive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Addon {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub repo: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledAddon {
    pub id: String,
    pub version: String,
    pub installed_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstalledAddons {
    addons: Vec<InstalledAddon>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    wow_addons_path: Option<String>,
}

fn get_config_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("wow-addon-manager")
        .join("config.json")
}

fn load_config() -> AppConfig {
    let path = get_config_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
    }
    AppConfig { wow_addons_path: None }
}

fn save_config(config: &AppConfig) -> Result<(), String> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

fn get_wow_addons_path() -> Result<PathBuf, String> {
    if let Some(custom) = load_config().wow_addons_path {
        let path = PathBuf::from(&custom);
        if path.exists() {
            return Ok(path);
        }
    }

    #[cfg(target_os = "macos")]
    {
        let paths = vec![
            PathBuf::from("/Applications/World of Warcraft/_retail_/Interface/AddOns"),
            dirs::home_dir()
                .map(|h| h.join("Applications/World of Warcraft/_retail_/Interface/AddOns"))
                .unwrap_or_default(),
        ];

        for path in paths {
            if path.exists() {
                return Ok(path);
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let paths = vec![
            PathBuf::from("C:\\Program Files (x86)\\World of Warcraft\\_retail_\\Interface\\AddOns"),
            PathBuf::from("C:\\Program Files\\World of Warcraft\\_retail_\\Interface\\AddOns"),
        ];

        for path in paths {
            if path.exists() {
                return Ok(path);
            }
        }
    }

    Err("WoW AddOns folder not found. Please set the path manually.".to_string())
}

fn get_installed_file_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("wow-addon-manager")
        .join("installed.json")
}

fn load_installed() -> InstalledAddons {
    let path = get_installed_file_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(data) = serde_json::from_str(&content) {
                return data;
            }
        }
    }
    InstalledAddons { addons: vec![] }
}

fn save_installed(data: &InstalledAddons) -> Result<(), String> {
    let path = get_installed_file_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_wow_path() -> Result<String, String> {
    get_wow_addons_path().map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
pub fn set_wow_path(path: String) -> Result<(), String> {
    let dir = PathBuf::from(&path);
    if !dir.exists() {
        return Err("The selected folder does not exist.".to_string());
    }
    let mut config = load_config();
    config.wow_addons_path = Some(path);
    save_config(&config)
}

#[tauri::command]
pub fn fetch_manifest() -> Vec<Addon> {
    vec![
        Addon {
            id: "RushHour".to_string(),
            name: "RushHour".to_string(),
            description: "World of Warcraft addon".to_string(),
            icon: Some("https://raw.githubusercontent.com/FlorianBx/RushHour/main/icon.png".to_string()),
            repo: "FlorianBx/RushHour".to_string(),
            version: "1.0.0".to_string(),
        },
        Addon {
            id: "craftpad".to_string(),
            name: "CraftPad".to_string(),
            description: "Browse and search housing items with their crafting requirements.".to_string(),
            icon: Some("https://raw.githubusercontent.com/FlorianBx/craftpad/main/icon.png".to_string()),
            repo: "FlorianBx/craftpad".to_string(),
            version: "1.0.0".to_string(),
        },
    ]
}

#[tauri::command]
pub fn get_installed_addons() -> Vec<InstalledAddon> {
    load_installed().addons
}

#[tauri::command]
pub async fn install_addon(addon: Addon) -> Result<(), String> {
    let wow_path = get_wow_addons_path()?;

    let zip_url = format!(
        "https://github.com/{}/archive/refs/heads/main.zip",
        addon.repo
    );

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(|e| e.to_string())?;

    let zip_response = client
        .get(&zip_url)
        .header("User-Agent", "WoW-Addon-Manager")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !zip_response.status().is_success() {
        return Err(format!("Failed to download: {}", zip_response.status()));
    }

    let zip_bytes = zip_response.bytes().await.map_err(|e| e.to_string())?;

    let cursor = Cursor::new(zip_bytes.to_vec());
    let mut archive = ZipArchive::new(cursor).map_err(|e| e.to_string())?;

    let addon_path = wow_path.join(&addon.id);
    if addon_path.exists() {
        fs::remove_dir_all(&addon_path).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(&addon_path).map_err(|e| e.to_string())?;

    let repo_name = addon.repo.split('/').last().unwrap_or(&addon.id);
    let prefix = format!("{}-main/", repo_name);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let name = file.name().to_string();

        if let Some(relative) = name.strip_prefix(&prefix) {
            if relative.is_empty() {
                continue;
            }

            let outpath = addon_path.join(relative);

            if name.ends_with('/') {
                fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
            } else {
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
                std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
            }
        }
    }

    let mut installed = load_installed();
    installed.addons.retain(|a| a.id != addon.id);
    installed.addons.push(InstalledAddon {
        id: addon.id,
        version: addon.version,
        installed_at: chrono::Utc::now().to_rfc3339(),
    });
    save_installed(&installed)?;

    Ok(())
}

#[tauri::command]
pub async fn uninstall_addon(addon_id: String) -> Result<(), String> {
    let wow_path = get_wow_addons_path()?;
    let addon_path = wow_path.join(&addon_id);

    if addon_path.exists() {
        fs::remove_dir_all(&addon_path).map_err(|e| e.to_string())?;
    }

    let mut installed = load_installed();
    installed.addons.retain(|a| a.id != addon_id);
    save_installed(&installed)?;

    Ok(())
}
