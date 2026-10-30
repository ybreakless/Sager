package com.ybreakless.sager.ui.main

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.filled.Share
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import com.ybreakless.sager.model.Achievement
import java.text.SimpleDateFormat
import java.util.*

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScreen(
    modifier: Modifier = Modifier,
    viewModel: MainScreenViewModel = viewModel()
) {
    val payload by viewModel.payloadState.collectAsStateWithLifecycle()
    val isSyncing by viewModel.isSyncing.collectAsStateWithLifecycle()
    val syncMessage by viewModel.syncMessage.collectAsStateWithLifecycle()
    val activeFilter by viewModel.activeFilter.collectAsStateWithLifecycle()
    val selectedGame by viewModel.selectedGameFilter.collectAsStateWithLifecycle()

    var showSettingsDialog by remember { mutableStateOf(false) }
    var selectedAchievementForShowoff by remember { mutableStateOf<Achievement?>(null) }
    var pcHostInput by remember { mutableStateOf(viewModel.getSavedPcHost()) }

    val snackbarHostState = remember { SnackbarHostState() }

    LaunchedEffect(syncMessage) {
        syncMessage?.let {
            snackbarHostState.showSnackbar(it)
            viewModel.clearMessage()
        }
    }

    Scaffold(
        modifier = modifier.fillMaxSize(),
        snackbarHost = { SnackbarHost(snackbarHostState) },
        topBar = {
            TopAppBar(
                title = {
                    Column {
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            Text("🎮 SAGER", fontWeight = FontWeight.ExtraBold, fontSize = 18.sp, color = Color(0xFF66C0F4))
                            Spacer(Modifier.width(6.dp))
                            Surface(
                                shape = RoundedCornerShape(4.dp),
                                color = Color(0x2666C0F4)
                            ) {
                                Text("POCKET", fontSize = 10.sp, fontWeight = FontWeight.Bold, color = Color(0xFF66C0F4), modifier = Modifier.padding(horizontal = 4.dp, vertical = 2.dp))
                            }
                        }
                        Text("Offline Trophy Vault", fontSize = 11.sp, color = Color(0xFF8F98A0))
                    }
                },
                actions = {
                    IconButton(onClick = { viewModel.syncNow() }, enabled = !isSyncing) {
                        if (isSyncing) {
                            CircularProgressIndicator(modifier = Modifier.size(20.dp), strokeWidth = 2.dp, color = Color(0xFF66C0F4))
                        } else {
                            Icon(Icons.Default.Refresh, contentDescription = "Sync over Wi-Fi", tint = Color(0xFF66C0F4))
                        }
                    }
                    IconButton(onClick = { showSettingsDialog = true }) {
                        Icon(Icons.Default.Settings, contentDescription = "Wi-Fi Settings", tint = Color(0xFF8F98A0))
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = Color(0xFF171A21)
                )
            )
        },
        containerColor = Color(0xFF0F141C)
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .padding(innerPadding)
                .fillMaxSize()
        ) {
            // Home Network Sync Status Banner
            Surface(
                color = Color(0xFF1B2838),
                modifier = Modifier.fillMaxWidth()
            ) {
                Row(
                    modifier = Modifier.padding(horizontal = 16.dp, vertical = 10.dp),
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Box(
                            modifier = Modifier
                                .size(8.dp)
                                .clip(CircleShape)
                                .background(Color(0xFF10B981))
                        )
                        Spacer(Modifier.width(8.dp))
                        Column {
                            Text(
                                text = "Home Wi-Fi: ${payload?.pcName ?: "Desktop PC"}",
                                fontSize = 12.sp,
                                fontWeight = FontWeight.Bold,
                                color = Color.White
                            )
                            Text(
                                text = "Tap Sync when home to pull trophies",
                                fontSize = 10.sp,
                                color = Color(0xFF8F98A0)
                            )
                        }
                    }
                    OutlinedButton(
                        onClick = { viewModel.syncNow() },
                        modifier = Modifier.height(32.dp),
                        contentPadding = PaddingValues(horizontal = 10.dp, vertical = 2.dp),
                        colors = ButtonDefaults.outlinedButtonColors(contentColor = Color(0xFF66C0F4)),
                        border = ButtonDefaults.outlinedButtonBorder.copy(brush = Brush.linearGradient(listOf(Color(0xFF66C0F4), Color(0xFF38BDF8))))
                    ) {
                        Text("⚡ Sync Now", fontSize = 11.sp, fontWeight = FontWeight.Bold)
                    }
                }
            }

            // Overview Stats Card
            val totalUnlocked = payload?.unlockedAchievementsCount ?: 0
            val totalGames = payload?.totalGames ?: 0
            Card(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(14.dp),
                shape = RoundedCornerShape(12.dp),
                colors = CardDefaults.cardColors(containerColor = Color(0xFF171A21)),
                border = ButtonDefaults.outlinedButtonBorder.copy(brush = Brush.linearGradient(listOf(Color(0xFF2A475E), Color(0xFF1B2838))))
            ) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(16.dp),
                    horizontalArrangement = Arrangement.SpaceAround,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Text("🏆 $totalUnlocked", fontSize = 22.sp, fontWeight = FontWeight.ExtraBold, color = Color(0xFFFBBF24))
                        Text("Trophies Unlocked", fontSize = 11.sp, color = Color(0xFF8F98A0))
                    }
                    Divider(modifier = Modifier.height(30.dp).width(1.dp), color = Color(0xFF2A475E))
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Text("🎮 $totalGames", fontSize = 22.sp, fontWeight = FontWeight.ExtraBold, color = Color(0xFF66C0F4))
                        Text("Games Tracked", fontSize = 11.sp, color = Color(0xFF8F98A0))
                    }
                    Divider(modifier = Modifier.height(30.dp).width(1.dp), color = Color(0xFF2A475E))
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Text("100%", fontSize = 22.sp, fontWeight = FontWeight.ExtraBold, color = Color(0xFF10B981))
                        Text("Offline Vault", fontSize = 11.sp, color = Color(0xFF8F98A0))
                    }
                }
            }

            // Filter Pills
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 14.dp, vertical = 4.dp),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                FilterChip(
                    selected = activeFilter == "all",
                    onClick = { viewModel.setFilter("all") },
                    label = { Text("All Trophies ($totalUnlocked)", fontSize = 12.sp) }
                )
                FilterChip(
                    selected = activeFilter == "unlocked",
                    onClick = { viewModel.setFilter("unlocked") },
                    label = { Text("🏆 Earned", fontSize = 12.sp) }
                )
            }

            // Trophy Showcase Feed
            val achievements = payload?.unlockedAchievements ?: emptyList()
            if (achievements.isEmpty()) {
                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .weight(1f),
                    contentAlignment = Alignment.Center
                ) {
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Text("🏆", fontSize = 42.sp)
                        Spacer(Modifier.height(8.dp))
                        Text("No trophies cached yet", fontWeight = FontWeight.Bold, color = Color.White)
                        Text("Connect to your PC over Wi-Fi and tap Sync!", fontSize = 12.sp, color = Color(0xFF8F98A0))
                    }
                }
            } else {
                LazyColumn(
                    modifier = Modifier.weight(1f),
                    contentPadding = PaddingValues(horizontal = 14.dp, vertical = 8.dp),
                    verticalArrangement = Arrangement.spacedBy(10.dp)
                ) {
                    items(achievements) { ach ->
                        AchievementCard(
                            achievement = ach,
                            onShowOffClick = { selectedAchievementForShowoff = ach }
                        )
                    }
                }
            }
        }
    }

    // Show-off celebration dialog for friends
    selectedAchievementForShowoff?.let { ach ->
        AlertDialog(
            onDismissRequest = { selectedAchievementForShowoff = null },
            confirmButton = {
                TextButton(onClick = { selectedAchievementForShowoff = null }) {
                    Text("Close", color = Color(0xFF66C0F4))
                }
            },
            title = {
                Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = Modifier.fillMaxWidth()) {
                    Text("🏆", fontSize = 54.sp)
                    Spacer(Modifier.height(8.dp))
                    Text("ACHIEVEMENT UNLOCKED", fontSize = 12.sp, fontWeight = FontWeight.ExtraBold, color = Color(0xFF66C0F4))
                    Text(ach.title, fontSize = 20.sp, fontWeight = FontWeight.Bold, color = Color.White, textAlign = TextAlign.Center)
                }
            },
            text = {
                Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = Modifier.fillMaxWidth()) {
                    Text(ach.description, fontSize = 13.sp, color = Color(0xFFCBD5E1), textAlign = TextAlign.Center)
                    Spacer(Modifier.height(14.dp))
                    Surface(shape = RoundedCornerShape(6.dp), color = Color(0xFF1B2838)) {
                        Text(
                            text = "🎮 ${ach.gameName}",
                            fontSize = 12.sp,
                            color = Color(0xFF94A3B8),
                            modifier = Modifier.padding(horizontal = 10.dp, vertical = 6.dp)
                        )
                    }
                    ach.unlockTime?.let { ts ->
                        Spacer(Modifier.height(6.dp))
                        val dateStr = SimpleDateFormat("MMM d, yyyy 'at' h:mm a", Locale.getDefault()).format(Date(ts * 1000))
                        Text("Earned: $dateStr", fontSize = 11.sp, color = Color(0xFF64748B))
                    }
                }
            },
            containerColor = Color(0xFF171A21)
        )
    }

    // Wi-Fi Connection Settings Dialog
    if (showSettingsDialog) {
        AlertDialog(
            onDismissRequest = { showSettingsDialog = false },
            title = { Text("🏠 Home Wi-Fi Sync Settings", color = Color.White, fontSize = 17.sp, fontWeight = FontWeight.Bold) },
            text = {
                Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                    Text("Enter your PC's IP address on your home Wi-Fi (found in Sager PC dashboard):", fontSize = 12.sp, color = Color(0xFFCBD5E1))
                    OutlinedTextField(
                        value = pcHostInput,
                        onValueChange = { pcHostInput = it },
                        label = { Text("Sager PC Address (e.g. 192.168.0.178:3939)") },
                        singleLine = true,
                        colors = OutlinedTextFieldDefaults.colors(
                            focusedBorderColor = Color(0xFF66C0F4),
                            unfocusedBorderColor = Color(0xFF2A475E),
                            focusedTextColor = Color.White,
                            unfocusedTextColor = Color.White
                        )
                    )
                    Text("💡 Ensure phone and PC are connected to the same home Wi-Fi.", fontSize = 11.sp, color = Color(0xFF8F98A0))
                }
            },
            confirmButton = {
                Button(
                    onClick = {
                        showSettingsDialog = false
                        viewModel.syncNow(pcHostInput)
                    },
                    colors = ButtonDefaults.buttonColors(containerColor = Color(0xFF66C0F4))
                ) {
                    Text("Save & Sync", color = Color(0xFF0F141C), fontWeight = FontWeight.Bold)
                }
            },
            dismissButton = {
                TextButton(onClick = { showSettingsDialog = false }) {
                    Text("Cancel", color = Color(0xFF8F98A0))
                }
            },
            containerColor = Color(0xFF171A21)
        )
    }
}

@Composable
fun AchievementCard(
    achievement: Achievement,
    onShowOffClick: () -> Unit
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable { onShowOffClick() },
        shape = RoundedCornerShape(10.dp),
        colors = CardDefaults.cardColors(containerColor = Color(0xFF171A21)),
        border = ButtonDefaults.outlinedButtonBorder.copy(brush = Brush.linearGradient(listOf(Color(0xFF2A475E), Color(0xFF1B2838))))
    ) {
        Row(
            modifier = Modifier.padding(14.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            // Gold Trophy Icon Badge
            Surface(
                modifier = Modifier.size(46.dp),
                shape = RoundedCornerShape(8.dp),
                color = Color(0xFF1B2838),
                border = ButtonDefaults.outlinedButtonBorder.copy(brush = Brush.linearGradient(listOf(Color(0xFFFBBF24), Color(0xFFD97706))))
            ) {
                Box(contentAlignment = Alignment.Center) {
                    Text("🏆", fontSize = 22.sp)
                }
            }

            Spacer(Modifier.width(12.dp))

            Column(modifier = Modifier.weight(1f)) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = achievement.title,
                        fontWeight = FontWeight.Bold,
                        fontSize = 14.sp,
                        color = Color.White,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                        modifier = Modifier.weight(1f)
                    )
                    Text(
                        text = "UNLOCKED",
                        fontSize = 9.sp,
                        fontWeight = FontWeight.ExtraBold,
                        color = Color(0xFF10B981)
                    )
                }

                Spacer(Modifier.height(3.dp))

                Text(
                    text = achievement.description,
                    fontSize = 12.sp,
                    color = Color(0xFF8F98A0),
                    maxLines = 2,
                    overflow = TextOverflow.Ellipsis
                )

                Spacer(Modifier.height(6.dp))

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = achievement.gameName,
                        fontSize = 11.sp,
                        color = Color(0xFF66C0F4),
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis
                    )
                    Text(
                        text = "Show ↗",
                        fontSize = 10.sp,
                        fontWeight = FontWeight.Bold,
                        color = Color(0xFFFBBF24)
                    )
                }
            }
        }
    }
}
