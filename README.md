# 🎮 Sager - Game Companion & Cloud Save Sync

> **Pure Rust Windows System Tray Companion** combining **Steam & Project Lightning Achievement Notifications** with **Ludusavi-style Save Game Backup & Google Drive Cloud Sync**.

---

## 🌟 Overview

**Sager** solves two critical PC gaming workflows in one unified, lightweight, pure Rust application:

1. **🏆 Steam & Game Achievements Notifier**:
   - Seamlessly monitors **Project Lightning**, **Goldberg Steam Emulator**, and native **Steam** save/achievement locations (`%APPDATA%\Goldberg SteamEmu Saves`, `%LOCALAPPDATA%\ProjectLightning`, etc.).
   - Detects newly earned achievements in real-time.
   - Pops native **Windows System Tray Toast / Balloon Alerts** with custom titles, descriptions, and audio chimes.
   - Maintains a local history log (`sager_achievements.json`) of all unlocked achievements.

2. **💾 Save Backup & Google Drive Cloud Sync (Ludusavi Engine)**:
   - Automatically scans and detects game save directories across standard PC locations:
     - `%USERPROFILE%\Saved Games`
     - `%USERPROFILE%\Documents\My Games`
     - `%LOCALAPPDATA%` (`Local` & `LocalLow`)
     - `%APPDATA%` (Roaming & Goldberg emulator saves)
     - Steam Cloud Userdata (`<SteamPath>\userdata\<user_id>\<app_id>\remote`)
     - Custom user-configured game folders
   - **One-Click Backup**: Creates compressed timestamped zip archives with JSON manifests (`sager_manifest.json`) in your designated backup folder.
   - **Cross-PC Save Sync via Google Drive**: Point your backup path to your Google Drive folder (`G:\My Drive\SagerBackups` or `C:\Users\<user>\Google Drive\SagerBackups`) on your laptop and desktop. Save games backed up from your laptop can be instantly restored on another PC!
   - **One-Click Restore**: Safely extracts the cloud save directly into the game's original save directory on your PC. Automatically creates a safety backup of your existing local save before overwriting.
   - **Automated Background Backup**: Runs scheduled backups automatically at user-defined intervals (e.g. every 60 minutes).

3. **🖥️ Windows System Tray & Embedded Dashboard**:
   - Sits in the Windows Notification Area (System Tray) with zero intrusive window clutter.
   - Right-click tray menu provides instant controls:
     - `🌐 Open Dashboard`
     - `💾 Backup All Game Saves Now`
     - `📂 Open Backup Folder (Google Drive)`
     - `🔔 Test Achievement Alert`
     - `⚙️ Settings`
     - `❌ Exit Sager`
   - Double-clicking the tray icon opens the **Sager Web Dashboard** (`http://127.0.0.1:3939`), featuring a cyberpunk dark-mode interface for viewing achievements, managing saves, and restoring backups.

---

## 🛠️ Architecture

Sager is written in **100% pure Rust** with no heavyweight UI frameworks or external C++ runtimes:

```
Sager/
├── Cargo.toml
├── src/
│   ├── main.rs                 # Application entry, threads orchestration & Win32 message loop
│   ├── config.rs               # Auto-detection of Google Drive, Steam, Goldberg & JSON config
│   ├── tray.rs                 # Win32 System Tray, context menu, & balloon notifications
│   ├── achievements/
│   │   ├── mod.rs
│   │   ├── models.rs           # Achievement & Notification data structures
│   │   ├── watcher.rs          # Background scanner for Goldberg/Project Lightning/Steam achievements
│   │   └── history.rs          # Persistent JSON storage for unlocked achievements
│   ├── backup/
│   │   ├── mod.rs
│   │   ├── scanner.rs          # Ludusavi-style save directory discovery engine
│   │   ├── archiver.rs         # Zip compression & manifest generation
│   │   └── restore.rs          # Safe extraction & pre-restore safety snapshots
│   └── web/
│       ├── mod.rs
│       ├── server.rs           # Embedded HTTP server & JSON REST API (tiny_http)
│       └── ui.rs               # Inlined modern responsive dark-mode Dashboard HTML/CSS/JS
```

---

## 🚀 Getting Started

### Prerequisites
- Windows 10 / 11
- Rust toolchain (`cargo`, `rustc`)

### Build
```powershell
# Debug build
cargo build

# Optimized release binary
cargo build --release
```
The compiled executable is placed at `target\release\sager.exe`.

### Run
```powershell
cargo run --release
```
Sager will start silently in your Windows System Tray!

---

## ☁️ Setting Up Google Drive Cross-Device Sync

1. On your **Laptop**:
   - Open Sager Dashboard (`http://127.0.0.1:3939` or click `🌐 Open Dashboard` from the tray).
   - In **Settings & Paths**, verify the **Google Drive / Cloud Backup Directory** (e.g. `G:\My Drive\SagerBackups`).
   - Click **Backup All Saves** or let Auto-Backup run.
2. On your **Other PC**:
   - Run Sager and ensure Google Drive points to the same `SagerBackups` folder.
   - Go to **Game Saves & Cloud Restore**.
   - Under **Available Cloud Backups for Restore**, find the game and click **🔄 Restore to PC**.
   - Sager will extract the save to the PC's corresponding save path and you can start playing immediately!

---

## 📄 License
MIT License.
