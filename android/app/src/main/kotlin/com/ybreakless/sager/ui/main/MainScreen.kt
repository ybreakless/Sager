package com.ybreakless.sager.ui.main

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
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
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Surface(
                            modifier = Modifier.size(34.dp),
                            shape = RoundedCornerShape(8.dp),
                            color = Color.White
                        ) {
                            Box(contentAlignment = Alignment.Center) {
                                Text("★", color = Color.Black, fontSize = 18.sp, fontWeight = FontWeight.Bold)
                            }
                        }
                        Spacer(Modifier.width(10.dp))
                        Column {
                            Text(
                                "SAGER",
                                fontWeight = FontWeight.Black,
                                fontSize = 18.sp,
                                letterSpacing = 1.sp,
                                color = Color.White
                            )
                            Text(
                                "MINIMAL TROPHY VAULT",
                                fontSize = 9.sp,
                                letterSpacing = 1.sp,
                                fontWeight = FontWeight.Medium,
                                color = Color(0xFFA1A1AA)
                            )
                        }
                    }
                },
                actions = {
                    TextButton(
                        onClick = { showSettingsDialog = true },
                        colors = ButtonDefaults.textButtonColors(contentColor = Color(0xFFA1A1AA))
                    ) {
                        Text("IP SYNC", fontSize = 11.sp, fontWeight = FontWeight.Bold)
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = Color(0xFF000000)
                )
            )
        },
        containerColor = Color(0xFF000000)
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .padding(innerPadding)
                .fillMaxSize()
        ) {
            // Minimal Divider Line
            Box(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(1.dp)
                    .background(Color(0xFF27272A))
            )

            // Minimal Sync Bar
            Surface(
                color = Color(0xFF0A0A0A),
                modifier = Modifier.fillMaxWidth()
            ) {
                Row(
                    modifier = Modifier.padding(horizontal = 16.dp, vertical = 12.dp),
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Column {
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            Box(
                                modifier = Modifier
                                    .size(6.dp)
                                    .clip(CircleShape)
                                    .background(Color.White)
                            )
                            Spacer(Modifier.width(6.dp))
                            Text(
                                text = payload?.pcName ?: "Desktop PC",
                                fontSize = 12.sp,
                                fontWeight = FontWeight.SemiBold,
                                color = Color.White
                            )
                        }
                        Text(
                            text = if (isSyncing) "Syncing with Sager PC..." else "Home Wi-Fi Local Sync",
                            fontSize = 10.sp,
                            color = Color(0xFF71717A)
                        )
                    }

                    OutlinedButton(
                        onClick = { viewModel.syncNow() },
                        enabled = !isSyncing,
                        modifier = Modifier.height(34.dp),
                        shape = RoundedCornerShape(6.dp),
                        contentPadding = PaddingValues(horizontal = 14.dp, vertical = 0.dp),
                        colors = ButtonDefaults.outlinedButtonColors(
                            containerColor = Color.Black,
                            contentColor = Color.White
                        ),
                        border = ButtonDefaults.outlinedButtonBorder.copy(
                            brush = androidx.compose.ui.graphics.SolidColor(Color.White)
                        )
                    ) {
                        if (isSyncing) {
                            CircularProgressIndicator(
                                modifier = Modifier.size(14.dp),
                                strokeWidth = 2.dp,
                                color = Color.White
                            )
                        } else {
                            Text("SYNC NOW", fontSize = 11.sp, fontWeight = FontWeight.Bold, letterSpacing = 0.5.sp)
                        }
                    }
                }
            }

            Box(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(1.dp)
                    .background(Color(0xFF27272A))
            )

            // Overview Metrics Strip (Black & White)
            val totalUnlocked = payload?.unlockedAchievementsCount ?: 0
            val totalGames = payload?.totalGames ?: 0

            Surface(
                color = Color(0xFF09090B),
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(14.dp),
                shape = RoundedCornerShape(8.dp),
                border = ButtonDefaults.outlinedButtonBorder.copy(
                    brush = androidx.compose.ui.graphics.SolidColor(Color(0xFF27272A))
                )
            ) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 14.dp),
                    horizontalArrangement = Arrangement.SpaceEvenly,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Text(
                            "$totalUnlocked",
                            fontSize = 22.sp,
                            fontWeight = FontWeight.Bold,
                            color = Color.White
                        )
                        Text(
                            "UNLOCKED",
                            fontSize = 10.sp,
                            fontWeight = FontWeight.Medium,
                            letterSpacing = 0.8.sp,
                            color = Color(0xFFA1A1AA)
                        )
                    }
                    Box(modifier = Modifier.height(24.dp).width(1.dp).background(Color(0xFF27272A)))
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Text(
                            "$totalGames",
                            fontSize = 22.sp,
                            fontWeight = FontWeight.Bold,
                            color = Color.White
                        )
                        Text(
                            "GAMES",
                            fontSize = 10.sp,
                            fontWeight = FontWeight.Medium,
                            letterSpacing = 0.8.sp,
                            color = Color(0xFFA1A1AA)
                        )
                    }
                    Box(modifier = Modifier.height(24.dp).width(1.dp).background(Color(0xFF27272A)))
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Text(
                            "VAULT",
                            fontSize = 18.sp,
                            fontWeight = FontWeight.Bold,
                            color = Color.White
                        )
                        Text(
                            "OFFLINE",
                            fontSize = 10.sp,
                            fontWeight = FontWeight.Medium,
                            letterSpacing = 0.8.sp,
                            color = Color(0xFFA1A1AA)
                        )
                    }
                }
            }

            // Minimal Filter Row
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 14.dp, vertical = 2.dp),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                FilterChip(
                    selected = activeFilter == "all",
                    onClick = { viewModel.setFilter("all") },
                    label = { Text("ALL TROPHIES ($totalUnlocked)", fontSize = 11.sp, fontWeight = FontWeight.SemiBold) },
                    colors = FilterChipDefaults.filterChipColors(
                        selectedContainerColor = Color.White,
                        selectedLabelColor = Color.Black,
                        containerColor = Color(0xFF121212),
                        labelColor = Color(0xFFA1A1AA)
                    ),
                    shape = RoundedCornerShape(6.dp)
                )
                FilterChip(
                    selected = activeFilter == "unlocked",
                    onClick = { viewModel.setFilter("unlocked") },
                    label = { Text("★ EARNED", fontSize = 11.sp, fontWeight = FontWeight.SemiBold) },
                    colors = FilterChipDefaults.filterChipColors(
                        selectedContainerColor = Color.White,
                        selectedLabelColor = Color.Black,
                        containerColor = Color(0xFF121212),
                        labelColor = Color(0xFFA1A1AA)
                    ),
                    shape = RoundedCornerShape(6.dp)
                )
            }

            // Achievement Showcase Feed
            val achievements = payload?.unlockedAchievements ?: emptyList()
            if (achievements.isEmpty()) {
                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .weight(1f),
                    contentAlignment = Alignment.Center
                ) {
                    Column(
                        horizontalAlignment = Alignment.CenterHorizontally,
                        modifier = Modifier.padding(24.dp)
                    ) {
                        Surface(
                            modifier = Modifier.size(54.dp),
                            shape = CircleShape,
                            color = Color(0xFF18181B)
                        ) {
                            Box(contentAlignment = Alignment.Center) {
                                Text("★", fontSize = 24.sp, color = Color.White)
                            }
                        }
                        Spacer(Modifier.height(14.dp))
                        Text(
                            "NO TROPHIES SYNCED YET",
                            fontWeight = FontWeight.Bold,
                            fontSize = 13.sp,
                            letterSpacing = 1.sp,
                            color = Color.White
                        )
                        Spacer(Modifier.height(4.dp))
                        Text(
                            "Connect your phone to the same Wi-Fi as your PC and tap SYNC NOW",
                            fontSize = 11.sp,
                            textAlign = TextAlign.Center,
                            color = Color(0xFF71717A)
                        )
                    }
                }
            } else {
                LazyColumn(
                    modifier = Modifier.weight(1f),
                    contentPadding = PaddingValues(horizontal = 14.dp, vertical = 8.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    items(achievements) { ach ->
                        AchievementCard(
                            achievement = ach,
                            onClick = { selectedAchievementForShowoff = ach }
                        )
                    }
                }
            }
        }
    }

    // Celebration Dialog (Minimal Monochrome)
    selectedAchievementForShowoff?.let { ach ->
        AlertDialog(
            onDismissRequest = { selectedAchievementForShowoff = null },
            confirmButton = {
                TextButton(onClick = { selectedAchievementForShowoff = null }) {
                    Text("CLOSE", color = Color.White, fontWeight = FontWeight.Bold)
                }
            },
            title = {
                Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = Modifier.fillMaxWidth()) {
                    Surface(
                        modifier = Modifier.size(50.dp),
                        shape = CircleShape,
                        color = Color.White
                    ) {
                        Box(contentAlignment = Alignment.Center) {
                            Text("★", fontSize = 26.sp, color = Color.Black)
                        }
                    }
                    Spacer(Modifier.height(12.dp))
                    Text(
                        "UNLOCKED",
                        fontSize = 11.sp,
                        fontWeight = FontWeight.Bold,
                        letterSpacing = 1.5.sp,
                        color = Color(0xFFA1A1AA)
                    )
                    Spacer(Modifier.height(4.dp))
                    Text(
                        ach.title,
                        fontSize = 18.sp,
                        fontWeight = FontWeight.Bold,
                        color = Color.White,
                        textAlign = TextAlign.Center
                    )
                }
            },
            text = {
                Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = Modifier.fillMaxWidth()) {
                    Text(
                        ach.description,
                        fontSize = 12.sp,
                        color = Color(0xFFD4D4D8),
                        textAlign = TextAlign.Center
                    )
                    Spacer(Modifier.height(14.dp))
                    Surface(
                        shape = RoundedCornerShape(4.dp),
                        color = Color(0xFF18181B),
                        border = ButtonDefaults.outlinedButtonBorder.copy(
                            brush = androidx.compose.ui.graphics.SolidColor(Color(0xFF27272A))
                        )
                    ) {
                        Text(
                            text = ach.gameName.uppercase(),
                            fontSize = 11.sp,
                            fontWeight = FontWeight.SemiBold,
                            letterSpacing = 0.5.sp,
                            color = Color.White,
                            modifier = Modifier.padding(horizontal = 10.dp, vertical = 6.dp)
                        )
                    }
                    ach.unlockTime?.let { ts ->
                        Spacer(Modifier.height(8.dp))
                        val dateStr = SimpleDateFormat("MMM d, yyyy 'at' h:mm a", Locale.getDefault()).format(Date(ts * 1000))
                        Text(
                            "Earned $dateStr",
                            fontSize = 10.sp,
                            color = Color(0xFF71717A)
                        )
                    }
                }
            },
            containerColor = Color(0xFF0C0C0E),
            shape = RoundedCornerShape(10.dp)
        )
    }

    // Wi-Fi Connection Settings Dialog (Minimal Monochrome)
    if (showSettingsDialog) {
        AlertDialog(
            onDismissRequest = { showSettingsDialog = false },
            title = {
                Text(
                    "HOME WI-FI SYNC SETTINGS",
                    color = Color.White,
                    fontSize = 14.sp,
                    fontWeight = FontWeight.Bold,
                    letterSpacing = 1.sp
                )
            },
            text = {
                Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                    Text(
                        "Enter the Sager PC address shown on your PC Companion dashboard:",
                        fontSize = 12.sp,
                        color = Color(0xFFA1A1AA)
                    )
                    OutlinedTextField(
                        value = pcHostInput,
                        onValueChange = { pcHostInput = it },
                        label = { Text("PC Address (e.g. 192.168.0.178:3939)", color = Color(0xFF71717A)) },
                        singleLine = true,
                        shape = RoundedCornerShape(6.dp),
                        colors = OutlinedTextFieldDefaults.colors(
                            focusedBorderColor = Color.White,
                            unfocusedBorderColor = Color(0xFF27272A),
                            focusedTextColor = Color.White,
                            unfocusedTextColor = Color.White,
                            cursorColor = Color.White
                        )
                    )
                    Text(
                        "Make sure your phone and gaming PC are on the same local network.",
                        fontSize = 10.sp,
                        color = Color(0xFF71717A)
                    )
                }
            },
            confirmButton = {
                Button(
                    onClick = {
                        showSettingsDialog = false
                        viewModel.syncNow(pcHostInput)
                    },
                    colors = ButtonDefaults.buttonColors(
                        containerColor = Color.White,
                        contentColor = Color.Black
                    ),
                    shape = RoundedCornerShape(6.dp)
                ) {
                    Text("SAVE & SYNC", fontWeight = FontWeight.Bold, fontSize = 11.sp)
                }
            },
            dismissButton = {
                TextButton(onClick = { showSettingsDialog = false }) {
                    Text("CANCEL", color = Color(0xFF71717A), fontSize = 11.sp)
                }
            },
            containerColor = Color(0xFF0C0C0E),
            shape = RoundedCornerShape(10.dp)
        )
    }
}

@Composable
fun AchievementCard(
    achievement: Achievement,
    onClick: () -> Unit
) {
    Surface(
        modifier = Modifier
            .fillMaxWidth()
            .clickable { onClick() },
        shape = RoundedCornerShape(8.dp),
        color = Color(0xFF09090B),
        border = ButtonDefaults.outlinedButtonBorder.copy(
            brush = androidx.compose.ui.graphics.SolidColor(Color(0xFF27272A))
        )
    ) {
        Row(
            modifier = Modifier.padding(12.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Surface(
                modifier = Modifier.size(42.dp),
                shape = RoundedCornerShape(6.dp),
                color = Color(0xFF18181B),
                border = ButtonDefaults.outlinedButtonBorder.copy(
                    brush = androidx.compose.ui.graphics.SolidColor(Color(0xFF3F3F46))
                )
            ) {
                Box(contentAlignment = Alignment.Center) {
                    Text("★", fontSize = 18.sp, color = Color.White)
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
                        fontSize = 13.sp,
                        color = Color.White,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                        modifier = Modifier.weight(1f)
                    )
                    Text(
                        text = "EARNED",
                        fontSize = 9.sp,
                        fontWeight = FontWeight.Bold,
                        letterSpacing = 0.5.sp,
                        color = Color(0xFFA1A1AA)
                    )
                }

                Spacer(Modifier.height(2.dp))

                Text(
                    text = achievement.description,
                    fontSize = 11.sp,
                    color = Color(0xFF71717A),
                    maxLines = 2,
                    overflow = TextOverflow.Ellipsis
                )

                Spacer(Modifier.height(4.dp))

                Text(
                    text = achievement.gameName.uppercase(),
                    fontSize = 10.sp,
                    fontWeight = FontWeight.Medium,
                    letterSpacing = 0.5.sp,
                    color = Color(0xFFA1A1AA),
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )
            }
        }
    }
}
