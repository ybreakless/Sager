use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub game_name: String,
    pub app_id: Option<String>,
    pub title: String,
    pub description: String,
    pub unlocked: bool,
    pub unlock_time: Option<i64>,
    pub icon_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementNotification {
    pub game_name: String,
    pub title: String,
    pub description: String,
    pub unlock_time: i64,
}
