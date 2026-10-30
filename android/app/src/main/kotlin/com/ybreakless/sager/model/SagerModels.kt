package com.ybreakless.sager.model

data class Achievement(
    val id: String,
    val gameName: String,
    val appId: String? = null,
    val title: String,
    val description: String,
    val unlocked: Boolean = true,
    val unlockTime: Long? = null,
    val launcher: String? = null
)

data class GameSummary(
    val gameName: String,
    val appId: String? = null,
    val launcher: String,
    val totalCount: Int,
    val unlockedCount: Int,
    val lockedCount: Int,
    val percentage: Float,
    val unlocked: List<Achievement> = emptyList(),
    val locked: List<Achievement> = emptyList()
)

data class MobileSyncPayload(
    val sagerVersion: String = "0.2.0",
    val pcName: String = "Sager-PC",
    val localIp: String = "127.0.0.1",
    val syncTimestamp: Long = System.currentTimeMillis() / 1000,
    val totalGames: Int = 0,
    val unlockedAchievementsCount: Int = 0,
    val games: List<GameSummary> = emptyList(),
    val unlockedAchievements: List<Achievement> = emptyList(),
    val preferredLauncher: String = "Goldberg"
)
