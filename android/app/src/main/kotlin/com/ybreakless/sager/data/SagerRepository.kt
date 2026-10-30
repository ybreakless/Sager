package com.ybreakless.sager.data

import android.content.Context
import com.ybreakless.sager.model.Achievement
import com.ybreakless.sager.model.GameSummary
import com.ybreakless.sager.model.MobileSyncPayload
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.withContext
import org.json.JSONArray
import org.json.JSONObject
import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import java.net.HttpURLConnection
import java.net.URL

class SagerRepository(private val context: Context) {

    private val prefs = context.getSharedPreferences("sager_prefs", Context.MODE_PRIVATE)
    private val vaultFile = File(context.filesDir, "sager_offline_vault.json")

    private val _payloadState = MutableStateFlow<MobileSyncPayload?>(null)
    val payloadState: StateFlow<MobileSyncPayload?> = _payloadState.asStateFlow()

    init {
        loadCachedVault()
    }

    fun getSavedPcHost(): String {
        return prefs.getString("pc_host", "192.168.0.178:3939") ?: "192.168.0.178:3939"
    }

    fun savePcHost(host: String) {
        prefs.edit().putString("pc_host", host.trim()).apply()
    }

    private fun loadCachedVault() {
        if (!vaultFile.exists()) {
            // Provide default demo/starter payload if completely new
            _payloadState.value = getSamplePayload()
            return
        }
        try {
            val jsonStr = vaultFile.readText()
            val parsed = parseSyncJson(jsonStr)
            _payloadState.value = parsed
        } catch (e: Exception) {
            e.printStackTrace()
            _payloadState.value = getSamplePayload()
        }
    }

    suspend fun syncWithPc(rawHost: String): Result<MobileSyncPayload> = withContext(Dispatchers.IO) {
        val cleanHost = rawHost.replace("http://", "").replace("https://", "").trimEnd('/')
        savePcHost(cleanHost)

        val syncUrl = "http://$cleanHost/api/mobile/sync"
        try {
            val url = URL(syncUrl)
            val conn = url.openConnection() as HttpURLConnection
            conn.connectTimeout = 4000
            conn.readTimeout = 4000
            conn.requestMethod = "GET"

            if (conn.responseCode != 200) {
                return@withContext Result.failure(Exception("HTTP ${conn.responseCode}: ${conn.responseMessage}"))
            }

            val reader = BufferedReader(InputStreamReader(conn.inputStream))
            val sb = StringBuilder()
            var line: String?
            while (reader.readLine().also { line = it } != null) {
                sb.append(line)
            }
            reader.close()

            val jsonStr = sb.toString()
            val payload = parseSyncJson(jsonStr)

            // Cache to offline file
            vaultFile.writeText(jsonStr)
            _payloadState.value = payload

            Result.success(payload)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun importVaultJson(jsonStr: String): Result<Int> = withContext(Dispatchers.IO) {
        try {
            val payload = parseSyncJson(jsonStr)
            vaultFile.writeText(jsonStr)
            _payloadState.value = payload
            Result.success(payload.unlockedAchievementsCount)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    private fun parseSyncJson(jsonStr: String): MobileSyncPayload {
        val root = JSONObject(jsonStr)
        val version = root.optString("sager_version", "0.2.0")
        val pcName = root.optString("pc_name", "Gaming-PC")
        val localIp = root.optString("local_ip", "127.0.0.1")
        val syncTimestamp = root.optLong("sync_timestamp", System.currentTimeMillis() / 1000)
        val launcher = root.optString("preferred_launcher", "Goldberg")

        // Parse unlocked achievements
        val achList = mutableListOf<Achievement>()
        val achArr = root.optJSONArray("unlocked_achievements") ?: root.optJSONArray("achievements") ?: JSONArray()
        for (i in 0 until achArr.length()) {
            val obj = achArr.getJSONObject(i)
            achList.add(
                Achievement(
                    id = obj.optString("id", "ach_$i"),
                    gameName = obj.optString("game_name", "Game"),
                    appId = obj.optString("app_id").takeIf { it.isNotEmpty() },
                    title = obj.optString("title", "Unlocked!"),
                    description = obj.optString("description", ""),
                    unlocked = obj.optBoolean("unlocked", true),
                    unlockTime = obj.optLong("unlock_time").takeIf { it > 0 },
                    launcher = obj.optString("launcher", launcher)
                )
            )
        }

        // Parse games summary
        val gamesList = mutableListOf<GameSummary>()
        val gamesArr = root.optJSONArray("games") ?: JSONArray()
        for (i in 0 until gamesArr.length()) {
            val g = gamesArr.getJSONObject(i)
            gamesList.add(
                GameSummary(
                    gameName = g.optString("game_name", "Game"),
                    appId = g.optString("app_id").takeIf { it.isNotEmpty() },
                    launcher = g.optString("launcher", launcher),
                    totalCount = g.optInt("total_count", 0),
                    unlockedCount = g.optInt("unlocked_count", 0),
                    lockedCount = g.optInt("locked_count", 0),
                    percentage = g.optDouble("percentage", 0.0).toFloat()
                )
            )
        }

        return MobileSyncPayload(
            sagerVersion = version,
            pcName = pcName,
            localIp = localIp,
            syncTimestamp = syncTimestamp,
            totalGames = root.optInt("total_games", gamesList.size),
            unlockedAchievementsCount = root.optInt("unlocked_achievements_count", achList.size),
            games = gamesList,
            unlockedAchievements = achList,
            preferredLauncher = launcher
        )
    }

    private fun getSamplePayload(): MobileSyncPayload {
        val sampleAchs = listOf(
            Achievement(
                id = "Achievement_18_Id",
                gameName = "LEGO Batman: Legacy of the Dark Knight",
                appId = "2215200",
                title = "Dynamic Duo",
                description = "Defeat 50 enemies as Batman and Robin in co-op.",
                unlocked = true,
                unlockTime = System.currentTimeMillis() / 1000 - 86400,
                launcher = "Goldberg"
            ),
            Achievement(
                id = "Achievement_19_Id",
                gameName = "LEGO Batman: Legacy of the Dark Knight",
                appId = "2215200",
                title = "Gotham Guardian",
                description = "Complete 100% of all villain missions in Gotham City.",
                unlocked = true,
                unlockTime = System.currentTimeMillis() / 1000 - 3600,
                launcher = "Goldberg"
            )
        )
        val sampleGames = listOf(
            GameSummary(
                gameName = "LEGO Batman: Legacy of the Dark Knight",
                appId = "2215200",
                launcher = "Goldberg SteamEmu",
                totalCount = 64,
                unlockedCount = 2,
                lockedCount = 62,
                percentage = 3.1f
            )
        )
        return MobileSyncPayload(
            sagerVersion = "0.2.0",
            pcName = "Desktop-PC",
            localIp = "192.168.0.178",
            totalGames = 1,
            unlockedAchievementsCount = 2,
            games = sampleGames,
            unlockedAchievements = sampleAchs
        )
    }
}
