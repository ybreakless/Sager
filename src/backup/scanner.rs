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

            // 2. Documents / My Games & Documents game folders
            let docs = PathBuf::from(&userprofile).join("Documents");
            let my_games_dir = docs.join("My Games");
            Self::scan_folder(&my_games_dir, "Documents/My Games", &mut results);
            Self::scan_folder(&docs, "Documents", &mut results);
        }

        // 3. AppData / Roaming
        if let Ok(appdata) = env::var("APPDATA") {
            let appdata_path = PathBuf::from(&appdata);
            // Scan Goldberg emulator saves explicitly
            let goldberg = appdata_path.join("Goldberg SteamEmu Saves");
            if goldberg.exists() {
                Self::scan_goldberg_saves(&goldberg, config, &mut results);
            }

            // Scan Goldberg Uplay emulator saves
            let uplay = appdata_path.join("Goldberg UplayEmu Saves");
            if uplay.exists() {
                Self::scan_uplay_saves(&uplay, &mut results);
            }

            // Scan GSE Saves
            let gse = appdata_path.join("GSE Saves");
            if gse.exists() {
                Self::scan_subfolders_with_prefix(&gse, "GSE", &mut results);
            }

            // Scan TRX (Tomb Raider)
            let trx = appdata_path.join("TRX");
            if trx.exists() {
                let (size, count, mtime) = calculate_dir_stats(&trx);
                if count > 0 {
                    results.push(DiscoveredGame {
                        name: "Tomb Raider (TRX)".to_string(),
                        source_path: trx,
                        save_category: "Roaming AppData".to_string(),
                        total_size_bytes: size,
                        files_count: count,
                        last_modified: mtime,
                    });
                }
            }
        }

        // 4. LocalAppData (Deep Game Save Discovery)
        if let Ok(local_appdata) = env::var("LOCALAPPDATA") {
            let local_path = PathBuf::from(&local_appdata);
            Self::scan_local_appdata(&local_path, &mut results);

            // LocalLow
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
                Self::scan_steam_userdata(&userdata, config, &mut results);
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
                    let folder_name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    // Skip non-game folders
                    if is_ignored_folder_name(&folder_name) {
                        continue;
                    }

                    let (size, count, mtime) = calculate_dir_stats(&path);
                    if count > 0 && size > 100 {
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

    fn scan_goldberg_saves(
        goldberg_dir: &Path,
        config: &SagerConfig,
        results: &mut Vec<DiscoveredGame>,
    ) {
        if let Ok(entries) = fs::read_dir(goldberg_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let app_id = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    let name = resolve_steam_app_name(&app_id, config)
                        .map(|n| format!("{} (Goldberg)", n))
                        .unwrap_or_else(|| format!("Goldberg SteamEmu ({})", app_id));

                    let (size, count, mtime) = calculate_dir_stats(&path);
                    if count > 0 {
                        results.push(DiscoveredGame {
                            name,
                            source_path: path,
                            save_category: "Goldberg SteamEmu".to_string(),
                            total_size_bytes: size,
                            files_count: count,
                            last_modified: mtime,
                        });
                    }
                }
            }
        }
    }

    fn scan_uplay_saves(uplay_dir: &Path, results: &mut Vec<DiscoveredGame>) {
        if let Ok(entries) = fs::read_dir(uplay_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let id = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let name = match id.as_str() {
                        "8006" => "Assassin's Creed Shadows (Uplay)".to_string(),
                        _ => format!("Goldberg Uplay ({})", id),
                    };

                    let (size, count, mtime) = calculate_dir_stats(&path);
                    if count > 0 {
                        results.push(DiscoveredGame {
                            name,
                            source_path: path,
                            save_category: "Goldberg Uplay".to_string(),
                            total_size_bytes: size,
                            files_count: count,
                            last_modified: mtime,
                        });
                    }
                }
            }
        }
    }

    fn scan_local_appdata(local_dir: &Path, results: &mut Vec<DiscoveredGame>) {
        if !local_dir.exists() || !local_dir.is_dir() {
            return;
        }

        // 1. Direct known publishers / titles
        let wb = local_dir
            .join("Warner Bros. Interactive Entertainment")
            .join("LEGO Batman - Legacy of the Dark Knight")
            .join("SaveGames");
        if wb.exists() {
            let (size, count, mtime) = calculate_dir_stats(&wb);
            results.push(DiscoveredGame {
                name: "LEGO Batman: Legacy of the Dark Knight".to_string(),
                source_path: wb,
                save_category: "Local AppData".to_string(),
                total_size_bytes: size,
                files_count: count,
                last_modified: mtime,
            });
        }

        let inv = local_dir.join("TheInvincible").join("Saved").join("SaveGames");
        if inv.exists() {
            let (size, count, mtime) = calculate_dir_stats(&inv);
            results.push(DiscoveredGame {
                name: "The Invincible".to_string(),
                source_path: inv,
                save_category: "Local AppData".to_string(),
                total_size_bytes: size,
                files_count: count,
                last_modified: mtime,
            });
        }

        let exp33 = local_dir.join("Sandfall").join("Saved").join("SaveGames");
        if exp33.exists() {
            let (size, count, mtime) = calculate_dir_stats(&exp33);
            results.push(DiscoveredGame {
                name: "Clair Obscur: Expedition 33".to_string(),
                source_path: exp33,
                save_category: "Local AppData".to_string(),
                total_size_bytes: size,
                files_count: count,
                last_modified: mtime,
            });
        }

        let f1 = local_dir.join("F1 25").join("savegame");
        if f1.exists() {
            let (size, count, mtime) = calculate_dir_stats(&f1);
            results.push(DiscoveredGame {
                name: "F1 25".to_string(),
                source_path: f1,
                save_category: "Local AppData".to_string(),
                total_size_bytes: size,
                files_count: count,
                last_modified: mtime,
            });
        }

        let bodycam = local_dir.join("Bodycam").join("Saved").join("SaveGames");
        if bodycam.exists() {
            let (size, count, mtime) = calculate_dir_stats(&bodycam);
            results.push(DiscoveredGame {
                name: "Bodycam".to_string(),
                source_path: bodycam,
                save_category: "Local AppData".to_string(),
                total_size_bytes: size,
                files_count: count,
                last_modified: mtime,
            });
        }

        let acs = local_dir
            .join("Ubisoft")
            .join("Assassin's Creed Shadows")
            .join("save");
        if acs.exists() {
            let (size, count, mtime) = calculate_dir_stats(&acs);
            results.push(DiscoveredGame {
                name: "Assassin's Creed Shadows".to_string(),
                source_path: acs,
                save_category: "Ubisoft".to_string(),
                total_size_bytes: size,
                files_count: count,
                last_modified: mtime,
            });
        }

        let ac2 = local_dir.join("AC2").join("Saved");
        if ac2.exists() {
            let (size, count, mtime) = calculate_dir_stats(&ac2);
            results.push(DiscoveredGame {
                name: "Assetto Corsa Competizione".to_string(),
                source_path: ac2,
                save_category: "Local AppData".to_string(),
                total_size_bytes: size,
                files_count: count,
                last_modified: mtime,
            });
        }
    }

    fn scan_subfolders_with_prefix(
        parent: &Path,
        category: &str,
        results: &mut Vec<DiscoveredGame>,
    ) {
        if !parent.exists() || !parent.is_dir() {
            return;
        }

        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let folder_name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
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

    fn scan_steam_userdata(
        userdata_dir: &Path,
        config: &SagerConfig,
        results: &mut Vec<DiscoveredGame>,
    ) {
        if let Ok(user_entries) = fs::read_dir(userdata_dir) {
            for user in user_entries.flatten() {
                let user_path = user.path();
                if user_path.is_dir() {
                    if let Ok(app_entries) = fs::read_dir(&user_path) {
                        for app in app_entries.flatten() {
                            let app_path = app.path();
                            let remote_dir = app_path.join("remote");
                            if remote_dir.exists() && remote_dir.is_dir() {
                                let app_id = app_path
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string();

                                // Ignore Steam internal app id 7
                                if app_id == "7" || app_id == "0" {
                                    continue;
                                }

                                let (size, count, mtime) = calculate_dir_stats(&remote_dir);
                                if count > 0 {
                                    let resolved = resolve_steam_app_name(&app_id, config)
                                        .map(|n| format!("{} (Steam Cloud)", n))
                                        .unwrap_or_else(|| format!("Steam AppID {}", app_id));

                                    results.push(DiscoveredGame {
                                        name: resolved,
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

pub fn resolve_steam_app_name(app_id: &str, config: &SagerConfig) -> Option<String> {
    // 1. Check known dictionary
    if let Some(known) = crate::achievements::get_known_app_name(app_id) {
        return Some(known.to_string());
    }

    // 2. Check Steam appmanifest_<appid>.acf
    if let Some(steam_path) = &config.steam_path {
        let manifest = steam_path
            .join("steamapps")
            .join(format!("appmanifest_{}.acf", app_id));
        if manifest.exists() {
            if let Ok(content) = fs::read_to_string(&manifest) {
                if let Some(name) = crate::achievements::extract_acf_key(&content, "name") {
                    return Some(crate::achievements::clean_game_name(&name));
                }
            }
        }
    }

    None
}

fn is_ignored_folder_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    matches!(
        lower.as_str(),
        "microsoft"
            | "google"
            | "mozilla"
            | "temp"
            | "package cache"
            | "audacity"
            | "crashreportclient"
            | "docker"
            | "programs"
            | "pip"
            | "npm"
            | "yarn"
            | "d3dscache"
            | "nvidia"
            | "amd"
            | "intel"
            | "asus"
            | "kimi"
            | "codex"
            | "studienkolleg"
    )
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
