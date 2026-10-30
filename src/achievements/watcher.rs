use super::history::{load_achievements_history, record_unlocked_achievement};
use super::models::{Achievement, AchievementNotification};
use crate::config::SagerConfig;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct AchievementWatcher {
    known_unlocked: Arc<Mutex<HashSet<String>>>,
    notifier_tx: Sender<AchievementNotification>,
}

impl AchievementWatcher {
    pub fn new(notifier_tx: Sender<AchievementNotification>) -> Self {
        let history = load_achievements_history();
        let mut known = HashSet::new();
        for ach in history {
            if ach.unlocked {
                known.insert(format!("{}:{}", ach.game_name, ach.id));
            }
        }

        Self {
            known_unlocked: Arc::new(Mutex::new(known)),
            notifier_tx,
        }
    }

    pub fn start(&self, config_provider: Arc<Mutex<SagerConfig>>) {
        let known_unlocked = Arc::clone(&self.known_unlocked);
        let tx = self.notifier_tx.clone();

        thread::spawn(move || {
            loop {
                let config = {
                    let lock = config_provider.lock().unwrap();
                    lock.clone()
                };

                Self::poll_achievements(&config, &known_unlocked, &tx);
                thread::sleep(Duration::from_secs(3));
            }
        });
    }

    pub fn poll_achievements(
        config: &SagerConfig,
        known_unlocked: &Arc<Mutex<HashSet<String>>>,
        tx: &Sender<AchievementNotification>,
    ) {
        let mut watch_paths = Vec::new();

        // 1. Goldberg Saves Path
        if let Some(goldberg) = &config.goldberg_saves_path {
            if goldberg.exists() {
                watch_paths.push(goldberg.clone());
            }
        }

        // 2. Project Lightning Path
        if let Some(pl) = &config.project_lightning_path {
            if pl.exists() {
                watch_paths.push(pl.clone());
            }
        }

        // 3. Custom Games with achievement paths
        for custom in &config.custom_games {
            if let Some(ach_path) = &custom.achievement_path {
                if ach_path.exists() {
                    watch_paths.push(ach_path.clone());
                }
            }
        }

        for path in watch_paths {
            Self::scan_directory_for_achievements(&path, config, known_unlocked, tx);
        }
    }

    fn scan_directory_for_achievements(
        dir: &Path,
        config: &SagerConfig,
        known_unlocked: &Arc<Mutex<HashSet<String>>>,
        tx: &Sender<AchievementNotification>,
    ) {
        if !dir.is_dir() {
            if dir.file_name().map_or(false, |f| f == "achievements.json") {
                Self::process_achievement_file(dir, config, known_unlocked, tx);
            }
            return;
        }

        // Check if directly in this directory
        let direct_file = dir.join("achievements.json");
        if direct_file.exists() {
            Self::process_achievement_file(&direct_file, config, known_unlocked, tx);
        }

        // Check subdirectories (e.g. %APPDATA%/Goldberg SteamEmu Saves/<appid>/achievements.json)
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let sub_path = entry.path();
                if sub_path.is_dir() {
                    let ach_file = sub_path.join("achievements.json");
                    if ach_file.exists() {
                        Self::process_achievement_file(&ach_file, config, known_unlocked, tx);
                    }
                }
            }
        }
    }

    fn process_achievement_file(
        file_path: &Path,
        config: &SagerConfig,
        known_unlocked: &Arc<Mutex<HashSet<String>>>,
        tx: &Sender<AchievementNotification>,
    ) {
        let Ok(content) = fs::read_to_string(file_path) else {
            return;
        };

        // Determine Game Name or AppID
        let (mut game_name, app_id) = Self::infer_game_info(file_path);

        // Try resolving full catalog and game title
        let mut catalog_map = std::collections::HashMap::new();
        if let Some(aid) = &app_id {
            if let Some(parent) = file_path.parent() {
                let (resolved_title, catalog_opt) =
                    super::catalog::AchievementCatalog::resolve_game_and_catalog(aid, parent, config);
                game_name = resolved_title;
                if let Some(defs) = catalog_opt {
                    for d in defs {
                        catalog_map.insert(d.name.clone(), d);
                    }
                }
            }
        }

        // Try parsing JSON content
        let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&content) else {
            return;
        };

        let mut achievements = Vec::new();

        if let Some(map) = json_val.as_object() {
            for (ach_key, ach_data) in map {
                let mut unlocked = false;
                let mut unlock_time = None;
                let mut display_name = ach_key.clone();
                let mut description = String::new();

                if let Some(is_bool) = ach_data.as_bool() {
                    unlocked = is_bool;
                } else if let Some(is_num) = ach_data.as_i64() {
                    unlocked = is_num > 0;
                } else if let Some(obj) = ach_data.as_object() {
                    // Check "earned", "achieved", or "unlocked"
                    if let Some(e) = obj.get("earned").and_then(|v| v.as_bool()) {
                        unlocked = e;
                    } else if let Some(a) = obj.get("achieved").and_then(|v| v.as_i64()) {
                        unlocked = a > 0;
                    } else if let Some(u) = obj.get("unlocked").and_then(|v| v.as_bool()) {
                        unlocked = u;
                    }

                    if let Some(t) = obj.get("earned_time").and_then(|v| v.as_i64()) {
                        unlock_time = Some(t);
                    } else if let Some(t) = obj.get("unlock_time").and_then(|v| v.as_i64()) {
                        unlock_time = Some(t);
                    }

                    if let Some(dn) = obj.get("displayName").and_then(|v| v.as_str()) {
                        display_name = dn.to_string();
                    } else if let Some(n) = obj.get("name").and_then(|v| v.as_str()) {
                        display_name = n.to_string();
                    }

                    if let Some(desc) = obj.get("description").and_then(|v| v.as_str()) {
                        description = desc.to_string();
                    }
                }

                // Check catalog for richer title & description if not present
                if let Some(def) = catalog_map.get(ach_key) {
                    if display_name == *ach_key {
                        if let Some(dn) = &def.displayName {
                            display_name = dn.clone();
                        }
                    }
                    if description.is_empty() {
                        if let Some(desc) = &def.description {
                            description = desc.clone();
                        }
                    }
                }

                if unlocked {
                    achievements.push(Achievement {
                        id: ach_key.clone(),
                        game_name: game_name.clone(),
                        app_id: app_id.clone(),
                        title: display_name,
                        description,
                        unlocked: true,
                        unlock_time,
                        icon_path: None,
                        launcher: Some(config.preferred_launcher.clone()),
                        hidden: false,
                    });
                }
            }
        } else if let Some(arr) = json_val.as_array() {
            for item in arr {
                if let Some(obj) = item.as_object() {
                    let mut unlocked = false;
                    let mut id = String::new();
                    let mut description = String::new();
                    let mut unlock_time = None;

                    if let Some(n) = obj.get("name").and_then(|v| v.as_str()) {
                        id = n.to_string();
                    }
                    let display_name = if let Some(dn) = obj.get("displayName").and_then(|v| v.as_str()) {
                        dn.to_string()
                    } else {
                        id.clone()
                    };
                    if let Some(desc) = obj.get("description").and_then(|v| v.as_str()) {
                        description = desc.to_string();
                    }
                    if let Some(e) = obj.get("earned").and_then(|v| v.as_bool()) {
                        unlocked = e;
                    } else if let Some(a) = obj.get("achieved").and_then(|v| v.as_i64()) {
                        unlocked = a > 0;
                    }
                    if let Some(t) = obj.get("unlock_time").and_then(|v| v.as_i64()) {
                        unlock_time = Some(t);
                    }

                    if unlocked && !id.is_empty() {
                        achievements.push(Achievement {
                            id,
                            game_name: game_name.clone(),
                            app_id: app_id.clone(),
                            title: display_name,
                            description,
                            unlocked: true,
                            unlock_time,
                            icon_path: None,
                            launcher: Some(config.preferred_launcher.clone()),
                            hidden: false,
                        });
                    }
                }
            }
        }

        let now = chrono::Utc::now().timestamp();
        for ach in achievements {
            let unique_key = format!("{}:{}", ach.game_name, ach.id);
            let mut known = known_unlocked.lock().unwrap();

            if !known.contains(&unique_key) {
                known.insert(unique_key);

                // Record to history file
                let mut ach_to_save = ach.clone();
                if ach_to_save.unlock_time.is_none() {
                    ach_to_save.unlock_time = Some(now);
                }
                record_unlocked_achievement(&ach_to_save);

                // Send notification
                let notif = AchievementNotification {
                    game_name: ach.game_name.clone(),
                    title: ach.title.clone(),
                    description: ach.description.clone(),
                    unlock_time: ach_to_save.unlock_time.unwrap_or(now),
                };
                let _ = tx.send(notif);
            }
        }
    }

    fn infer_game_info(file_path: &Path) -> (String, Option<String>) {
        let parent = file_path.parent();
        let mut app_id = None;
        let mut game_name = "Unknown Game".to_string();

        if let Some(p) = parent {
            let folder_name = p.file_name().unwrap_or_default().to_string_lossy().to_string();

            // Check if folder name is all digits (Steam AppID)
            if folder_name.chars().all(|c| c.is_ascii_digit()) && !folder_name.is_empty() {
                app_id = Some(folder_name.clone());
                // Try to find game title from app_name.txt or steam_settings
                let app_name_file = p.join("app_name.txt");
                if let Ok(name) = fs::read_to_string(&app_name_file) {
                    let trimmed = name.trim().to_string();
                    if !trimmed.is_empty() {
                        game_name = trimmed;
                        return (game_name, app_id);
                    }
                }
                if let Some(known) = super::catalog::get_known_app_name(&folder_name) {
                    game_name = known.to_string();
                    return (game_name, app_id);
                }
                game_name = format!("Game (AppID {})", folder_name);
            } else if !folder_name.is_empty() {
                game_name = folder_name;
            }
        }

        (game_name, app_id)
    }
}
