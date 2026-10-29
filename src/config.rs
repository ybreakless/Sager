use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomGameConfig {
    pub name: String,
    pub save_path: PathBuf,
    pub achievement_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagerConfig {
    pub backup_directory: PathBuf,
    pub steam_path: Option<PathBuf>,
    pub goldberg_saves_path: Option<PathBuf>,
    pub project_lightning_path: Option<PathBuf>,
    pub custom_games: Vec<CustomGameConfig>,
    pub notifications_enabled: bool,
    pub sound_enabled: bool,
    pub auto_backup_enabled: bool,
    pub auto_backup_interval_minutes: u64,
    pub web_port: u16,
}

impl Default for SagerConfig {
    fn default() -> Self {
        let backup_dir = detect_google_drive_backup_dir();
        let steam_dir = detect_steam_path();
        let goldberg_dir = detect_goldberg_path();
        let pl_dir = detect_project_lightning_path();

        Self {
            backup_directory: backup_dir,
            steam_path: steam_dir,
            goldberg_saves_path: goldberg_dir,
            project_lightning_path: pl_dir,
            custom_games: Vec::new(),
            notifications_enabled: true,
            sound_enabled: true,
            auto_backup_enabled: true,
            auto_backup_interval_minutes: 60,
            web_port: 3939,
        }
    }
}

pub fn get_config_dir() -> PathBuf {
    if let Ok(appdata) = env::var("APPDATA") {
        let path = PathBuf::from(appdata).join("Sager");
        let _ = fs::create_dir_all(&path);
        path
    } else {
        PathBuf::from(".")
    }
}

pub fn get_config_file_path() -> PathBuf {
    get_config_dir().join("sager_config.json")
}

pub fn load_config() -> SagerConfig {
    let path = get_config_file_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str::<SagerConfig>(&content) {
                return config;
            }
        }
    }
    let default = SagerConfig::default();
    save_config(&default);
    default
}

pub fn save_config(config: &SagerConfig) {
    let path = get_config_file_path();
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = fs::write(path, json);
    }
}

/// Detects Google Drive sync directory if available, or falls back to user profile
fn detect_google_drive_backup_dir() -> PathBuf {
    // Check drive letters for Google Drive (e.g. G:\My Drive, H:\My Drive)
    for letter in ['G', 'H', 'F', 'E', 'D'] {
        let candidate = PathBuf::from(format!("{}:\\My Drive\\SagerBackups", letter));
        if let Some(parent) = candidate.parent() {
            if parent.exists() {
                return candidate;
            }
        }
    }

    // Check user profile Google Drive folder
    if let Ok(userprofile) = env::var("USERPROFILE") {
        let gdrive = PathBuf::from(&userprofile).join("Google Drive").join("SagerBackups");
        if let Some(parent) = gdrive.parent() {
            if parent.exists() {
                return gdrive;
            }
        }
        return PathBuf::from(userprofile).join("SagerBackups");
    }

    PathBuf::from("SagerBackups")
}

fn detect_steam_path() -> Option<PathBuf> {
    // Check Program Files (x86)\Steam
    if let Ok(prog_files) = env::var("ProgramFiles(x86)") {
        let candidate = PathBuf::from(prog_files).join("Steam");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    // Check C:\Steam
    let c_steam = PathBuf::from(r"C:\Steam");
    if c_steam.exists() {
        return Some(c_steam);
    }
    None
}

fn detect_goldberg_path() -> Option<PathBuf> {
    if let Ok(appdata) = env::var("APPDATA") {
        let candidate = PathBuf::from(appdata).join("Goldberg SteamEmu Saves");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

fn detect_project_lightning_path() -> Option<PathBuf> {
    if let Ok(local_appdata) = env::var("LOCALAPPDATA") {
        let candidate = PathBuf::from(local_appdata).join("ProjectLightning");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    if let Ok(appdata) = env::var("APPDATA") {
        let candidate = PathBuf::from(appdata).join("Project Lightning");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}
