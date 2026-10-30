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
    #[serde(default)]
    pub launcher: Option<String>,
    #[serde(default)]
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementNotification {
    pub game_name: String,
    pub title: String,
    pub description: String,
    pub unlock_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAchievementSummary {
    pub game_name: String,
    pub app_id: Option<String>,
    pub launcher: String,
    pub total_count: usize,
    pub unlocked_count: usize,
    pub locked_count: usize,
    pub percentage: f32,
    pub last_unlocked: Option<i64>,
    pub unlocked: Vec<Achievement>,
    pub locked: Vec<Achievement>,
}
