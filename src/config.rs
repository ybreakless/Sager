use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
    #[serde(default = "default_google_drive_exe")]
    pub google_drive_exe: Option<PathBuf>,
    #[serde(default = "default_backups_per_day")]
    pub backups_per_day: Option<u32>,
    #[serde(default = "default_preferred_launcher")]
    pub preferred_launcher: String,
    #[serde(default = "default_true")]
    pub show_in_launcher: bool,
    #[serde(default = "default_true")]
    pub tray_compact_mode: bool,
}

fn default_preferred_launcher() -> String {
    "Goldberg SteamEmu".to_string()
}

fn default_true() -> bool {
    true
}

fn default_backups_per_day() -> Option<u32> {
    Some(24)
}

impl SagerConfig {
    pub fn get_backups_per_day(&self) -> u32 {
        if let Some(bpd) = self.backups_per_day {
            if bpd > 0 {
                return bpd;
            }
        }
        if self.auto_backup_interval_minutes > 0 {
            (1440 / self.auto_backup_interval_minutes).max(1) as u32
        } else {
            24
        }
    }

    pub fn set_backups_per_day(&mut self, times: u32) {
        let valid_times = times.clamp(1, 144);
        self.backups_per_day = Some(valid_times);
        self.auto_backup_interval_minutes = (1440 / valid_times as u64).max(1);
    }
}

fn default_google_drive_exe() -> Option<PathBuf> {
    detect_google_drive_exe()
}

impl Default for SagerConfig {
    fn default() -> Self {
        let backup_dir = detect_google_drive_backup_dir();
        let steam_dir = detect_steam_path();
        let goldberg_dir = detect_goldberg_path();
        let pl_dir = detect_project_lightning_path();
        let gdrive_exe = detect_google_drive_exe();

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
            google_drive_exe: gdrive_exe,
            backups_per_day: Some(24),
            preferred_launcher: "Goldberg SteamEmu".to_string(),
            show_in_launcher: true,
            tray_compact_mode: true,
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

// ───────────────────────────── Google Drive Detection ─────────────────────────

/// Check if Google Drive (GoogleDriveFS.exe) is currently running
pub fn is_google_drive_running() -> bool {
    match Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq GoogleDriveFS.exe", "/NH"])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains("GoogleDriveFS.exe")
        }
        Err(_) => false,
    }
}

/// Launch Google Drive desktop app
pub fn launch_google_drive(config: &SagerConfig) -> Result<(), String> {
    // Try configured path first
    if let Some(exe) = &config.google_drive_exe {
        if exe.exists() {
            return Command::new(exe)
                .spawn()
                .map(|_| ())
                .map_err(|e| format!("Failed to launch Google Drive: {}", e));
        }
    }
    // Try well-known install locations
    if let Some(exe) = detect_google_drive_exe() {
        return Command::new(&exe)
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("Failed to launch Google Drive: {}", e));
    }
    Err("Google Drive executable not found. Install Google Drive for desktop.".to_string())
}

/// Check if the configured backup directory exists and is accessible
pub fn is_backup_dir_accessible(config: &SagerConfig) -> bool {
    config.backup_directory.exists() || {
        // Try creating it
        fs::create_dir_all(&config.backup_directory).is_ok()
    }
}

/// Returns a status summary for display
pub fn get_drive_status(config: &SagerConfig) -> DriveStatus {
    let gdrive_running = is_google_drive_running();
    let backup_dir_exists = config.backup_directory.exists();
    let backup_dir_is_gdrive = is_path_on_google_drive(&config.backup_directory);

    DriveStatus {
        google_drive_running: gdrive_running,
        google_drive_installed: detect_google_drive_exe().is_some(),
        backup_dir: config.backup_directory.clone(),
        backup_dir_exists,
        backup_dir_is_google_drive: backup_dir_is_gdrive,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveStatus {
    pub google_drive_running: bool,
    pub google_drive_installed: bool,
    pub backup_dir: PathBuf,
    pub backup_dir_exists: bool,
    pub backup_dir_is_google_drive: bool,
}

/// Detect Google Drive sync directory if available, or falls back to user profile
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
    if let Ok(prog_files) = env::var("ProgramFiles(x86)") {
        let candidate = PathBuf::from(prog_files).join("Steam");
        if candidate.exists() {
            return Some(candidate);
        }
    }
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

/// Detect the Google Drive for Desktop executable
fn detect_google_drive_exe() -> Option<PathBuf> {
    // Standard install path
    if let Ok(prog_files) = env::var("ProgramFiles") {
        let candidate = PathBuf::from(&prog_files).join("Google").join("Drive").join("GoogleDriveFS.exe");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    // Sometimes under Program Files (x86)
    if let Ok(prog_files) = env::var("ProgramFiles(x86)") {
        let candidate = PathBuf::from(&prog_files).join("Google").join("Drive").join("GoogleDriveFS.exe");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    // Check user-level install
    if let Ok(local_appdata) = env::var("LOCALAPPDATA") {
        let candidate = PathBuf::from(&local_appdata).join("Google").join("Drive").join("GoogleDriveFS.exe");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

/// Check if a path is on a Google Drive mount (e.g. G:\My Drive\...)
fn is_path_on_google_drive(path: &PathBuf) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    if path_str.contains("my drive") || path_str.contains("google drive") {
        return true;
    }
    // Check if it's on a Google Drive FS mount point (virtual drive letter)
    if path_str.len() >= 2 {
        let drive_letter = &path_str[0..1];
        for letter in ['g', 'h', 'f', 'e', 'd'] {
            if drive_letter == letter.to_string() {
                let my_drive = format!("{}:\\my drive", letter);
                if path_str.starts_with(&my_drive) {
                    return true;
                }
            }
        }
    }
    false
}
