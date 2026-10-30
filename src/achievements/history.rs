use super::models::Achievement;
use crate::config::get_config_dir;
use std::fs;
use std::path::PathBuf;

pub fn get_history_file_path() -> PathBuf {
    get_config_dir().join("sager_achievements.json")
}

pub fn load_achievements_history() -> Vec<Achievement> {
    let path = get_history_file_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(achievements) = serde_json::from_str::<Vec<Achievement>>(&content) {
                return achievements;
            }
        }
    }
    Vec::new()
}

pub fn save_achievements_history(achievements: &[Achievement]) {
    let path = get_history_file_path();
    if let Ok(json) = serde_json::to_string_pretty(achievements) {
        let _ = fs::write(path, json);
    }
}

pub fn record_unlocked_achievement(ach: &Achievement) {
    let mut history = load_achievements_history();
    // Check if already in history
    if !history.iter().any(|a| a.id == ach.id && a.game_name == ach.game_name) {
        history.insert(0, ach.clone());
        save_achievements_history(&history);
    }
    // Also sync to Goldberg / emulator launcher directory
    sync_achievement_to_launcher(ach);
}

/// Syncs the unlocked achievement to Goldberg emulator and Project Lightning local saves
pub fn sync_achievement_to_launcher(ach: &Achievement) {
    let app_id = match &ach.app_id {
        Some(aid) if !aid.is_empty() && aid != "0" => aid.clone(),
        _ => {
            // Infer from common games
            if ach.game_name.contains("LEGO") || ach.game_name.contains("Batman") {
                "2215200".to_string()
            } else if ach.game_name.contains("F1") {
                "3059520".to_string()
            } else {
                return;
            }
        }
    };

    if let Ok(appdata) = std::env::var("APPDATA") {
        let goldberg_dir = PathBuf::from(&appdata)
            .join("Goldberg SteamEmu Saves")
            .join(&app_id);
        let _ = fs::create_dir_all(&goldberg_dir);
        let ach_file = goldberg_dir.join("achievements.json");

        let mut current_map: serde_json::Map<String, serde_json::Value> = if ach_file.exists() {
            fs::read_to_string(&ach_file)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            serde_json::Map::new()
        };

        let now = ach.unlock_time.unwrap_or_else(|| chrono::Utc::now().timestamp());
        let entry = serde_json::json!({
            "earned": true,
            "earned_time": now
        });
        current_map.insert(ach.id.clone(), entry);

        if let Ok(serialized) = serde_json::to_string_pretty(&current_map) {
            let _ = fs::write(&ach_file, serialized);
        }
    }
}
