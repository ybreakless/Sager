pub mod achievements;
pub mod backup;
pub mod config;
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

fn main() {
    println!("==================================================");
    println!("   SAGER - Steam Achievements & Save Backup Hub   ");
    println!("   Pure Rust System Tray Companion for Windows    ");
    println!("==================================================");

    let config = load_config();
    println!("Backup Target Directory: {}", config.backup_directory.display());
    println!("Notifications: {}", if config.notifications_enabled { "Enabled" } else { "Disabled" });
    println!("Sound Chime: {}", if config.sound_enabled { "Enabled" } else { "Disabled" });
    println!("Auto-Backup: {} (every {} mins)", 
        if config.auto_backup_enabled { "Enabled" } else { "Disabled" }, 
        config.auto_backup_interval_minutes);

    let shared_config = Arc::new(Mutex::new(config));
    let (tx, rx) = channel::<AchievementNotification>();

    // 1. Start Achievement Watcher
    let watcher = AchievementWatcher::new(tx.clone());
    watcher.start(Arc::clone(&shared_config));
    println!(">> Achievement Watcher started.");

    // 2. Start Embedded Web Server & Dashboard
    WebServer::start(Arc::clone(&shared_config), tx.clone());
    println!(">> Web Dashboard started on http://127.0.0.1:3939");

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
    match SystemTray::new() {
        Ok(tray) => {
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
        }
        Err(e) => {
            eprintln!("Failed to create Windows System Tray icon: {}", e);
        }
    }
}
