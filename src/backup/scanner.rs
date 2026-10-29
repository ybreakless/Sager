use crate::config::SagerConfig;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredGame {
    pub name: String,
    pub source_path: PathBuf,
    pub save_category: String,
    pub total_size_bytes: u64,
    pub files_count: usize,
    pub last_modified: Option<i64>,
}

pub struct SaveScanner;

impl SaveScanner {
    pub fn scan_all(config: &SagerConfig) -> Vec<DiscoveredGame> {
        let mut results = Vec::new();

        // 1. Saved Games (%USERPROFILE%/Saved Games)
        if let Ok(userprofile) = env::var("USERPROFILE") {
            let saved_games_dir = PathBuf::from(&userprofile).join("Saved Games");
            Self::scan_folder(&saved_games_dir, "Saved Games", &mut results);

            // 2. Documents / My Games
            let my_games_dir = PathBuf::from(&userprofile).join("Documents").join("My Games");
            Self::scan_folder(&my_games_dir, "Documents/My Games", &mut results);
        }

        // 3. AppData / Roaming
        if let Ok(appdata) = env::var("APPDATA") {
            let appdata_path = PathBuf::from(&appdata);
            // Scan Goldberg emulator saves explicitly
            let goldberg = appdata_path.join("Goldberg SteamEmu Saves");
            if goldberg.exists() {
                Self::scan_subfolders_with_prefix(&goldberg, "Goldberg SteamEmu", &mut results);
            }
        }

        // 4. LocalAppData (Local & LocalLow)
        if let Ok(local_appdata) = env::var("LOCALAPPDATA") {
            let local_path = PathBuf::from(&local_appdata);
            let local_low = local_path.parent().map(|p| p.join("LocalLow"));
            if let Some(low) = local_low {
                if low.exists() {
                    Self::scan_folder(&low, "LocalLow", &mut results);
                }
            }
        }

        // 5. Steam Userdata Cloud Saves
        if let Some(steam_path) = &config.steam_path {
            let userdata = steam_path.join("userdata");
            if userdata.exists() {
                Self::scan_steam_userdata(&userdata, &mut results);
            }
        }

        // 6. Custom games from config
        for custom in &config.custom_games {
            if custom.save_path.exists() {
                let (size, count, mtime) = calculate_dir_stats(&custom.save_path);
                results.push(DiscoveredGame {
                    name: custom.name.clone(),
                    source_path: custom.save_path.clone(),
                    save_category: "Custom".to_string(),
                    total_size_bytes: size,
                    files_count: count,
                    last_modified: mtime,
                });
            }
        }

        // Deduplicate by source_path
        results.sort_by(|a, b| a.name.cmp(&b.name));
        results.dedup_by(|a, b| a.source_path == b.source_path);

        results
    }

    fn scan_folder(parent: &Path, category: &str, results: &mut Vec<DiscoveredGame>) {
        if !parent.exists() || !parent.is_dir() {
            return;
        }

        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let folder_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let (size, count, mtime) = calculate_dir_stats(&path);
                    if count > 0 {
                        results.push(DiscoveredGame {
                            name: folder_name,
                            source_path: path,
                            save_category: category.to_string(),
                            total_size_bytes: size,
                            files_count: count,
                            last_modified: mtime,
                        });
                    }
                }
            }
        }
    }

    fn scan_subfolders_with_prefix(parent: &Path, category: &str, results: &mut Vec<DiscoveredGame>) {
        if !parent.exists() || !parent.is_dir() {
            return;
        }

        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let folder_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let (size, count, mtime) = calculate_dir_stats(&path);
                    if count > 0 {
                        results.push(DiscoveredGame {
                            name: format!("{} ({})", category, folder_name),
                            source_path: path,
                            save_category: category.to_string(),
                            total_size_bytes: size,
                            files_count: count,
                            last_modified: mtime,
                        });
                    }
                }
            }
        }
    }

    fn scan_steam_userdata(userdata_dir: &Path, results: &mut Vec<DiscoveredGame>) {
        if let Ok(user_entries) = fs::read_dir(userdata_dir) {
            for user in user_entries.flatten() {
                let user_path = user.path();
                if user_path.is_dir() {
                    if let Ok(app_entries) = fs::read_dir(&user_path) {
                        for app in app_entries.flatten() {
                            let app_path = app.path();
                            let remote_dir = app_path.join("remote");
                            if remote_dir.exists() && remote_dir.is_dir() {
                                let app_id = app_path.file_name().unwrap_or_default().to_string_lossy().to_string();
                                let (size, count, mtime) = calculate_dir_stats(&remote_dir);
                                if count > 0 {
                                    results.push(DiscoveredGame {
                                        name: format!("Steam AppID {}", app_id),
                                        source_path: remote_dir,
                                        save_category: "Steam Cloud".to_string(),
                                        total_size_bytes: size,
                                        files_count: count,
                                        last_modified: mtime,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn calculate_dir_stats(dir: &Path) -> (u64, usize, Option<i64>) {
    let mut total_size = 0u64;
    let mut files_count = 0usize;
    let mut newest_time = None;

    for entry in WalkDir::new(dir).into_iter().flatten() {
        if entry.file_type().is_file() {
            files_count += 1;
            if let Ok(meta) = entry.metadata() {
                total_size += meta.len();
                if let Ok(modified) = meta.modified() {
                    if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                        let secs = duration.as_secs() as i64;
                        match newest_time {
                            Some(current) if secs > current => newest_time = Some(secs),
                            None => newest_time = Some(secs),
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    (total_size, files_count, newest_time)
}
