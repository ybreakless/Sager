#![windows_subsystem = "windows"]

pub mod achievements;
pub mod audio;
pub mod backup;
pub mod config;
pub mod overlay;
pub mod tray;
pub mod web;

use achievements::{AchievementNotification, AchievementWatcher};
use backup::SaveArchiver;
use config::load_config;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tray::{start_notification_listener, SystemTray};
use web::WebServer;

#[link(name = "kernel32")]
extern "system" {
    fn AttachConsole(dwProcessId: u32) -> i32;
}
const ATTACH_PARENT_PROCESS: u32 = 0xFFFFFFFF;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--console" || a == "--debug") {
        unsafe {
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }

    let config = load_config();
    let open_ui_on_start = args.iter().any(|a| a == "--ui" || a == "--dashboard" || a == "-d");

    // Automatically ensure Sager boots with Windows silently in tray
    ensure_auto_startup();

    let shared_config = Arc::new(Mutex::new(config));
    let (tx, rx) = channel::<AchievementNotification>();

    // 1. Start Achievement Watcher
    let watcher = AchievementWatcher::new(tx.clone());
    watcher.start(Arc::clone(&shared_config));
    println!(">> Achievement Watcher started.");

    // 2. Start Embedded Web Server & Dashboard
    WebServer::start(Arc::clone(&shared_config), tx.clone());
    println!(">> Web Dashboard started on http://127.0.0.1:3939");
    if open_ui_on_start {
        let _ = open::that("http://127.0.0.1:3939");
    }

    // 3. Start Background Auto-Backup Thread
    let backup_config = Arc::clone(&shared_config);
    thread::spawn(move || {
        loop {
            let (enabled, interval_mins) = {
                let cfg = backup_config.lock().unwrap();
                (cfg.auto_backup_enabled, cfg.auto_backup_interval_minutes)
            };

            if enabled {
                thread::sleep(Duration::from_secs(interval_mins.max(5) * 60));
                let cfg = backup_config.lock().unwrap().clone();
                let results = SaveArchiver::backup_all(&cfg);
                let successes = results.iter().filter(|r| r.is_ok()).count();
                println!(">> [Auto-Backup] Backed up {} of {} games to {}", successes, results.len(), cfg.backup_directory.display());
            } else {
                thread::sleep(Duration::from_secs(60));
            }
        }
    });

    // 4. Initialize Windows System Tray Icon
    println!(">> Initializing Windows System Tray...");
    if let Ok(tray) = SystemTray::new() {
        let handle = tray.handle();
        start_notification_listener(handle, rx, Arc::clone(&shared_config));

        // Startup notification
        let sound = shared_config.lock().unwrap().sound_enabled;
        handle.show_balloon_notification(
            "🎮 Sager Activated",
            "Monitoring Steam / Project Lightning achievements & syncing game saves!",
            sound,
        );

        println!(">> Sager system tray icon added. Running message loop...");
        tray.run_message_loop();
        println!(">> Sager shutting down cleanly.");
    } else {
        println!(">> Background engine active. Waiting for interactive desktop tray...");
        // Keep web server and watcher active and retry connecting to tray
        loop {
            thread::sleep(Duration::from_secs(5));
            if let Ok(tray) = SystemTray::new() {
                let handle = tray.handle();
                start_notification_listener(handle, rx, Arc::clone(&shared_config));
                let sound = shared_config.lock().unwrap().sound_enabled;
                handle.show_balloon_notification(
                    "🎮 Sager Activated",
                    "Monitoring Steam / Project Lightning achievements & syncing game saves!",
                    sound,
                );
                println!(">> Sager system tray icon connected! Running message loop...");
                tray.run_message_loop();
                break;
            }
        }
    }
}

fn ensure_auto_startup() {
    if let Ok(appdata) = std::env::var("APPDATA") {
        let lnk = std::path::PathBuf::from(&appdata)
            .join(r"Microsoft\Windows\Start Menu\Programs\Startup\Sager.lnk");
        if !lnk.exists() {
            if let Ok(exe_path) = std::env::current_exe() {
                let file_name = exe_path.file_name().and_then(|f| f.to_str()).unwrap_or("");
                if file_name.to_lowercase().starts_with("sager") {
                    let cur_dir = exe_path
                        .parent()
                        .unwrap_or_else(|| std::path::Path::new(r"C:\Mein Projekts\Sager"));
                    let ps_cmd = format!(
                        "$s = (New-Object -ComObject WScript.Shell).CreateShortcut('{}'); $s.TargetPath = '{}'; $s.WorkingDirectory = '{}'; $s.Save()",
                        lnk.to_string_lossy().replace('\'', "''"),
                        exe_path.to_string_lossy().replace('\'', "''"),
                        cur_dir.to_string_lossy().replace('\'', "''"),
                    );
                    let _ = std::process::Command::new("powershell")
                        .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &ps_cmd])
                        .output();
                }
            }
        }
    }
}
