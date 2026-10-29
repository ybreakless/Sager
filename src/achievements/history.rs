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
}
