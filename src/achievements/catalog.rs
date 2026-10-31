use super::history::load_achievements_history;
use super::models::{Achievement, GameAchievementSummary};
use crate::config::SagerConfig;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawAchievementDef {
    pub name: String,
    #[serde(default)]
    pub displayName: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub hidden: Option<serde_json::Value>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub icongray: Option<String>,
}

pub struct AchievementCatalog;

impl AchievementCatalog {
    /// Discovers all tracked games and computes their overall achievement progress (unlocked vs locked)
    pub fn get_overall_summary(config: &SagerConfig) -> Vec<GameAchievementSummary> {
        let mut summaries: Vec<GameAchievementSummary> = Vec::new();
        let history = load_achievements_history();

        // Map of (game_key) -> HashMap of ach_id -> (Achievement, unlock_time)
        let mut history_by_game: HashMap<String, Vec<Achievement>> = HashMap::new();
        for ach in history {
            let key = if let Some(app_id) = &ach.app_id {
                app_id.clone()
            } else {
                ach.game_name.clone()
            };
            history_by_game.entry(key).or_default().push(ach);
        }

        // 1. Scan Goldberg SteamEmu Saves
        if let Some(goldberg_dir) = &config.goldberg_saves_path {
            if goldberg_dir.exists() && goldberg_dir.is_dir() {
                if let Ok(entries) = fs::read_dir(goldberg_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            let app_id_str = path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();

                            // Read earned achievements file in this folder
                            let earned_file = path.join("achievements.json");
                            let earned_map = Self::parse_earned_file(&earned_file);

                            // Resolve game title and catalog of all achievements (both locked and unlocked)
                            let (game_title, catalog) =
                                Self::resolve_game_and_catalog(&app_id_str, &path, config);

                            let mut unlocked = Vec::new();
                            let mut locked = Vec::new();
                            let mut latest_unlock: Option<i64> = None;

                            if let Some(cat) = catalog {
                                for def in cat {
                                    let is_earned = earned_map.contains_key(&def.name);
                                    let unlock_time = earned_map.get(&def.name).copied().flatten();

                                    if let Some(t) = unlock_time {
                                        match latest_unlock {
                                            Some(prev) if t > prev => latest_unlock = Some(t),
                                            None => latest_unlock = Some(t),
                                            _ => {}
                                        }
                                    }

                                    let title = def
                                        .displayName
                                        .clone()
                                        .unwrap_or_else(|| def.name.clone());
                                    let description = def.description.clone().unwrap_or_default();
                                    let is_hidden = def
                                        .hidden
                                        .as_ref()
                                        .map(|v| v.as_str() == Some("1") || v.as_i64() == Some(1))
                                        .unwrap_or(false);

                                    let ach = Achievement {
                                        id: def.name.clone(),
                                        game_name: game_title.clone(),
                                        app_id: Some(app_id_str.clone()),
                                        title,
                                        description,
                                        unlocked: is_earned,
                                        unlock_time,
                                        icon_path: if is_earned {
                                            def.icon.clone()
                                        } else {
                                            def.icongray.clone()
                                        },
                                        launcher: Some("Goldberg SteamEmu".to_string()),
                                        hidden: is_hidden,
                                    };

                                    if is_earned {
                                        unlocked.push(ach);
                                    } else {
                                        locked.push(ach);
                                    }
                                }
                            } else {
                                // No full catalog file found: create entries for the earned achievements
                                for (ach_id, u_time) in &earned_map {
                                    if let Some(t) = *u_time {
                                        match latest_unlock {
                                            Some(prev) if t > prev => latest_unlock = Some(t),
                                            None => latest_unlock = Some(t),
                                            _ => {}
                                        }
                                    }
                                    unlocked.push(Achievement {
                                        id: ach_id.clone(),
                                        game_name: game_title.clone(),
                                        app_id: Some(app_id_str.clone()),
                                        title: ach_id.clone(),
                                        description: "Earned achievement".to_string(),
                                        unlocked: true,
                                        unlock_time: *u_time,
                                        icon_path: None,
                                        launcher: Some("Goldberg SteamEmu".to_string()),
                                        hidden: false,
                                    });
                                }
                            }

                            // Sort unlocked by unlock_time descending
                            unlocked.sort_by(|a, b| b.unlock_time.cmp(&a.unlock_time));
                            locked.sort_by(|a, b| a.title.cmp(&b.title));

                            let total = unlocked.len() + locked.len();
                            let unlocked_count = unlocked.len();
                            let locked_count = locked.len();
                            let percentage = if total > 0 {
                                ((unlocked_count as f32 / total as f32) * 1000.0).round() / 10.0
                            } else {
                                0.0
                            };

                            summaries.push(GameAchievementSummary {
                                game_name: game_title,
                                app_id: Some(app_id_str.clone()),
                                launcher: "Goldberg SteamEmu".to_string(),
                                total_count: total,
                                unlocked_count,
                                locked_count,
                                percentage,
                                last_unlocked: latest_unlock,
                                unlocked,
                                locked,
                            });
                        }
                    }
                }
            }
        }

        // 2. Incorporate custom / non-Goldberg games from history (e.g. Project Lightning or manual triggers)
        let existing_app_ids: HashSet<String> = summaries
            .iter()
            .filter_map(|s| s.app_id.clone())
            .collect();

        for (game_key, achs) in history_by_game {
            if existing_app_ids.contains(&game_key) {
                continue;
            }

            let first = achs.first();
            let game_name = first
                .map(|a| a.game_name.clone())
                .unwrap_or_else(|| game_key.clone());
            let app_id = first.and_then(|a| a.app_id.clone());

            let mut unlocked = Vec::new();
            let mut latest_unlock: Option<i64> = None;

            for ach in achs {
                if let Some(t) = ach.unlock_time {
                    match latest_unlock {
                        Some(prev) if t > prev => latest_unlock = Some(t),
                        None => latest_unlock = Some(t),
                        _ => {}
                    }
                }
                unlocked.push(ach);
            }

            unlocked.sort_by(|a, b| b.unlock_time.cmp(&a.unlock_time));
            let u_count = unlocked.len();

            let launcher_label = if game_name.to_lowercase().contains("lightning") {
                "Project Lightning".to_string()
            } else {
                config.preferred_launcher.clone()
            };

            summaries.push(GameAchievementSummary {
                game_name,
                app_id,
                launcher: launcher_label,
                total_count: u_count,
                unlocked_count: u_count,
                locked_count: 0,
                percentage: 100.0,
                last_unlocked: latest_unlock,
                unlocked,
                locked: Vec::new(),
            });
        }

        summaries.sort_by(|a, b| {
            // Put games with active achievements and recent unlocks first
            b.unlocked_count
                .cmp(&a.unlocked_count)
                .then_with(|| a.game_name.cmp(&b.game_name))
        });

        summaries
    }

    /// Parses an earned achievements JSON file (e.g. %APPDATA%\Goldberg SteamEmu Saves\<appid>\achievements.json)
    pub fn parse_earned_file(path: &Path) -> HashMap<String, Option<i64>> {
        let mut map = HashMap::new();
        if !path.exists() {
            return map;
        }

        let Ok(content) = fs::read_to_string(path) else {
            return map;
        };

        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(obj) = val.as_object() {
                for (k, v) in obj {
                    let mut earned = false;
                    let mut time: Option<i64> = None;

                    if let Some(b) = v.as_bool() {
                        earned = b;
                    } else if let Some(n) = v.as_i64() {
                        earned = n > 0;
                    } else if let Some(o) = v.as_object() {
                        if let Some(e) = o.get("earned").and_then(|x| x.as_bool()) {
                            earned = e;
                        } else if let Some(a) = o.get("achieved").and_then(|x| x.as_i64()) {
                            earned = a > 0;
                        }
                        if let Some(t) = o.get("earned_time").and_then(|x| x.as_i64()) {
                            time = Some(t);
                        } else if let Some(t) = o.get("unlock_time").and_then(|x| x.as_i64()) {
                            time = Some(t);
                        }
                    }

                    if earned {
                        map.insert(k.clone(), time);
                    }
                }
            }
        }

        map
    }

    /// Resolves the human-readable game title and locates the full achievement catalog
    pub fn resolve_game_and_catalog(
        app_id: &str,
        goldberg_save_folder: &Path,
        config: &SagerConfig,
    ) -> (String, Option<Vec<RawAchievementDef>>) {
        let mut game_title: Option<String> = None;
        let mut catalog: Option<Vec<RawAchievementDef>> = None;

        // 1. Check known AppID dictionary first (cleanest curated names)
        if let Some(known) = get_known_app_name(app_id) {
            game_title = Some(known.to_string());
        }

        // 2. Check app_name.txt inside goldberg save folder
        if game_title.is_none() {
            let app_name_file = goldberg_save_folder.join("app_name.txt");
            if let Ok(name) = fs::read_to_string(&app_name_file) {
                let trimmed = name.trim().to_string();
                if !trimmed.is_empty() {
                    game_title = Some(trimmed);
                }
            }
        }

        // 3. Check Steam appmanifest_<appid>.acf
        let mut steam_installdir: Option<String> = None;
        if let Some(steam_path) = &config.steam_path {
            let manifest_path = steam_path
                .join("steamapps")
                .join(format!("appmanifest_{}.acf", app_id));
            if manifest_path.exists() {
                if let Ok(content) = fs::read_to_string(&manifest_path) {
                    if game_title.is_none() {
                        if let Some(name) = extract_acf_key(&content, "name") {
                            game_title = Some(clean_game_name(&name));
                        }
                    }
                    if let Some(dir) = extract_acf_key(&content, "installdir") {
                        steam_installdir = Some(dir);
                    }
                }
            }
        }

        let resolved_title = game_title.unwrap_or_else(|| format!("Game (AppID {})", app_id));

        // 4. Locate full achievement definitions catalog:
        // A) Directly inside goldberg save folder steam_settings
        let local_steam_settings = goldberg_save_folder.join("steam_settings").join("achievements.json");
        if local_steam_settings.exists() {
            catalog = Self::load_raw_catalog(&local_steam_settings);
        }

        // B) In Steam common installdir if available
        if catalog.is_none() {
            if let Some(steam_path) = &config.steam_path {
                let common_dir = steam_path.join("steamapps").join("common");
                if let Some(inst_dir) = steam_installdir {
                    let game_folder = common_dir.join(inst_dir);
                    if game_folder.exists() {
                        catalog = Self::find_catalog_in_game_folder(&game_folder);
                    }
                }

                // C) Search common folder for matching game folder name
                if catalog.is_none() {
                    let search_name = resolved_title
                        .split([':', '-', '('])
                        .next()
                        .unwrap_or(&resolved_title)
                        .trim();
                    if let Ok(entries) = fs::read_dir(&common_dir) {
                        for entry in entries.flatten() {
                            let p = entry.path();
                            if p.is_dir() {
                                let folder_str = p.file_name().unwrap_or_default().to_string_lossy();
                                if folder_str.to_lowercase().contains(&search_name.to_lowercase()) {
                                    catalog = Self::find_catalog_in_game_folder(&p);
                                    if catalog.is_some() {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        (resolved_title, catalog)
    }

    /// Scans a game installation directory for steam_settings\achievements.json
    pub fn find_catalog_in_game_folder(game_dir: &Path) -> Option<Vec<RawAchievementDef>> {
        for entry in WalkDir::new(game_dir).max_depth(8).into_iter().flatten() {
            if entry.file_type().is_file() && entry.file_name() == "achievements.json" {
                let path = entry.path();
                // Ensure it's inside a steam_settings or settings directory
                if let Some(parent) = path.parent() {
                    let parent_name = parent.file_name().unwrap_or_default().to_string_lossy();
                    if parent_name == "steam_settings" || parent_name == "settings" {
                        if let Some(defs) = Self::load_raw_catalog(path) {
                            if !defs.is_empty() {
                                return Some(defs);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Loads and parses a raw achievements.json catalog array
    pub fn load_raw_catalog(path: &Path) -> Option<Vec<RawAchievementDef>> {
        let Ok(content) = fs::read_to_string(path) else {
            return None;
        };

        if let Ok(defs) = serde_json::from_str::<Vec<RawAchievementDef>>(&content) {
            return Some(defs);
        }

        None
    }
}

pub fn extract_acf_key(acf_content: &str, key: &str) -> Option<String> {
    for line in acf_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("\"{}\"", key)) {
            let parts: Vec<&str> = trimmed.split('"').collect();
            if parts.len() >= 4 {
                return Some(parts[3].to_string());
            }
        }
    }
    None
}

pub fn clean_game_name(raw: &str) -> String {
    let mut cleaned = String::new();
    for c in raw.chars() {
        if c.is_ascii_alphanumeric() || c.is_ascii_whitespace() || ":-–—_.'\"!&(),".contains(c) {
            cleaned.push(c);
        }
    }
    cleaned
        .replace("LEGOAr ", "LEGO ")
        .replace("F1Ar ", "F1 ")
        .replace("  ", " ")
        .trim()
        .to_string()
}

pub fn get_known_app_name(app_id: &str) -> Option<&'static str> {
    match app_id {
        "2215200" => Some("LEGO Batman: Legacy of the Dark Knight"),
        "431960" => Some("Wallpaper Engine"),
        "3059520" => Some("F1 25"),
        "228980" => Some("Steamworks Shared Components"),
        "8870" => Some("BioShock Infinite"),
        "620" => Some("Portal 2"),
        "221040" => Some("Resident Evil 6"),
        "524220" => Some("NieR:Automata"),
        "1659420" => Some("UNCHARTED: Legacy of Thieves Collection"),
        "1888930" => Some("The Last of Us Part I"),
        "1903340" => Some("Clair Obscur: Expedition 33"),
        "731040" => Some("The Invincible"),
        "2478970" => Some("Bodycam"),
        "242050" => Some("Assassin's Creed IV: Black Flag"),
        "273" => Some("Assassin's Creed IV: Black Flag"),
        _ => None,
    }
}
