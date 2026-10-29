use crate::achievements::{load_achievements_history, Achievement, AchievementNotification};
use crate::backup::{SaveArchiver, SaveRestore, SaveScanner};
use crate::config::{self, save_config, SagerConfig};
use crate::web::ui::get_dashboard_html;
use serde_json::json;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use tiny_http::{Header, Method, Response, Server, StatusCode};

pub struct WebServer;

impl WebServer {
    pub fn start(
        config_provider: Arc<Mutex<SagerConfig>>,
        notifier_tx: Sender<AchievementNotification>,
    ) {
        thread::spawn(move || {
            let port = {
                let cfg = config_provider.lock().unwrap();
                cfg.web_port
            };

            let addr = format!("127.0.0.1:{}", port);
            let server = match Server::http(&addr) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to start Sager web server on {}: {}", addr, e);
                    return;
                }
            };

            println!("Sager Web Dashboard running on http://{}", addr);

            for mut req in server.incoming_requests() {
                let url = req.url().to_string();
                let method = req.method().clone();

                let json_header =
                    Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                let html_header = Header::from_bytes(
                    &b"Content-Type"[..],
                    &b"text/html; charset=utf-8"[..],
                )
                .unwrap();

                if url == "/" && method == Method::Get {
                    let html = get_dashboard_html();
                    let resp = Response::from_string(html).with_header(html_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/logo" && method == Method::Get {
                    let logo_bytes: &[u8] = include_bytes!("../../assets/icon.png");
                    let png_header =
                        Header::from_bytes(&b"Content-Type"[..], &b"image/png"[..]).unwrap();
                    let resp = Response::from_data(logo_bytes).with_header(png_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/logo-full" && method == Method::Get {
                    let logo_bytes: &[u8] = include_bytes!("../../assets/logo.png");
                    let png_header =
                        Header::from_bytes(&b"Content-Type"[..], &b"image/png"[..]).unwrap();
                    let resp = Response::from_data(logo_bytes).with_header(png_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/status" && method == Method::Get {
                    let cfg = config_provider.lock().unwrap().clone();
                    let games = SaveScanner::scan_all(&cfg);
                    let achievements = load_achievements_history();
                    let drive = config::get_drive_status(&cfg);

                    let status = json!({
                        "version": "0.1.0",
                        "backup_dir": cfg.backup_directory.to_string_lossy(),
                        "games_count": games.len(),
                        "achievements_count": achievements.len(),
                        "auto_backup_enabled": cfg.auto_backup_enabled,
                        "auto_backup_interval": cfg.auto_backup_interval_minutes,
                        "backups_per_day": cfg.get_backups_per_day(),
                        "sound_enabled": cfg.sound_enabled,
                        "google_drive_running": drive.google_drive_running,
                        "google_drive_installed": drive.google_drive_installed,
                        "backup_dir_exists": drive.backup_dir_exists,
                        "backup_dir_is_google_drive": drive.backup_dir_is_google_drive,
                    });

                    let resp =
                        Response::from_string(status.to_string()).with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/drive-status" && method == Method::Get {
                    let cfg = config_provider.lock().unwrap().clone();
                    let drive = config::get_drive_status(&cfg);
                    let resp = Response::from_string(
                        serde_json::to_string(&drive).unwrap_or_default(),
                    )
                    .with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/launch-gdrive" && method == Method::Post {
                    let cfg = config_provider.lock().unwrap().clone();
                    match config::launch_google_drive(&cfg) {
                        Ok(()) => {
                            let resp_body = json!({
                                "success": true,
                                "message": "Google Drive is starting up..."
                            });
                            let resp = Response::from_string(resp_body.to_string())
                                .with_header(json_header);
                            let _ = req.respond(resp);
                        }
                        Err(e) => {
                            let resp_body = json!({
                                "success": false,
                                "message": e
                            });
                            let resp = Response::from_string(resp_body.to_string())
                                .with_header(json_header);
                            let _ = req.respond(resp);
                        }
                    }
                    continue;
                }

                if url == "/api/games" && method == Method::Get {
                    let cfg = config_provider.lock().unwrap().clone();
                    let games = SaveScanner::scan_all(&cfg);
                    let resp = Response::from_string(
                        serde_json::to_string(&games).unwrap_or_default(),
                    )
                    .with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/backups" && method == Method::Get {
                    let cfg = config_provider.lock().unwrap().clone();
                    let backups = SaveRestore::list_all_backups(&cfg);
                    let resp = Response::from_string(
                        serde_json::to_string(&backups).unwrap_or_default(),
                    )
                    .with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/backup" && method == Method::Post {
                    let mut body = String::new();
                    let _ = req.as_reader().read_to_string(&mut body);
                    let cfg = config_provider.lock().unwrap().clone();

                    let parsed: serde_json::Value =
                        serde_json::from_str(&body).unwrap_or_default();
                    if parsed
                        .get("all")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false)
                    {
                        let res = SaveArchiver::backup_all(&cfg);
                        let success_count = res.iter().filter(|r| r.is_ok()).count();
                        let total = res.len();

                        let resp_body = json!({
                            "success": true,
                            "message": format!("Backed up {} of {} game saves to {}", success_count, total, cfg.backup_directory.display())
                        });
                        let resp = Response::from_string(resp_body.to_string())
                            .with_header(json_header);
                        let _ = req.respond(resp);
                    } else {
                        let name = parsed
                            .get("game_name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Game");
                        let path_str = parsed
                            .get("source_path")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let path = PathBuf::from(path_str);

                        match SaveArchiver::backup_game(name, &path, &cfg) {
                            Ok(info) => {
                                let resp_body = json!({
                                    "success": true,
                                    "message": format!("Successfully backed up {} ({})", name, info.file_name),
                                    "backup": info
                                });
                                let resp = Response::from_string(resp_body.to_string())
                                    .with_header(json_header);
                                let _ = req.respond(resp);
                            }
                            Err(e) => {
                                let resp_body = json!({
                                    "success": false,
                                    "message": format!("Failed to backup {}: {}", name, e)
                                });
                                let resp = Response::from_string(resp_body.to_string())
                                    .with_header(json_header);
                                let _ = req.respond(resp);
                            }
                        }
                    }
                    continue;
                }

                if url == "/api/restore" && method == Method::Post {
                    let mut body = String::new();
                    let _ = req.as_reader().read_to_string(&mut body);
                    let cfg = config_provider.lock().unwrap().clone();

                    let parsed: serde_json::Value =
                        serde_json::from_str(&body).unwrap_or_default();
                    let zip_str = parsed
                        .get("zip_path")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let zip_path = PathBuf::from(zip_str);

                    match SaveRestore::restore_backup(&zip_path, None, &cfg) {
                        Ok(res) => {
                            let resp_body = json!({
                                "success": true,
                                "message": format!("Restored {} files to {}", res.restored_files, res.target_path.display()),
                                "result": res
                            });
                            let resp = Response::from_string(resp_body.to_string())
                                .with_header(json_header);
                            let _ = req.respond(resp);
                        }
                        Err(e) => {
                            let resp_body = json!({
                                "success": false,
                                "message": e
                            });
                            let resp = Response::from_string(resp_body.to_string())
                                .with_header(json_header);
                            let _ = req.respond(resp);
                        }
                    }
                    continue;
                }

                if url == "/api/achievements" && method == Method::Get {
                    let list = load_achievements_history();
                    let resp = Response::from_string(
                        serde_json::to_string(&list).unwrap_or_default(),
                    )
                    .with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/achievements/test" && method == Method::Post {
                    let now = chrono::Utc::now().timestamp();
                    let test_notif = AchievementNotification {
                        game_name: "Steam / Project Lightning".to_string(),
                        title: "Sager Master Unlocked".to_string(),
                        description:
                            "Achievement & Save Backup Companion successfully activated!"
                                .to_string(),
                        unlock_time: now,
                    };
                    let _ = notifier_tx.send(test_notif);

                    let test_ach = Achievement {
                        id: format!("test_ach_{}", now),
                        game_name: "Steam / Project Lightning".to_string(),
                        app_id: Some("480".to_string()),
                        title: "Sager Master Unlocked".to_string(),
                        description:
                            "Achievement & Save Backup Companion successfully activated!"
                                .to_string(),
                        unlocked: true,
                        unlock_time: Some(now),
                        icon_path: None,
                    };
                    crate::achievements::record_unlocked_achievement(&test_ach);

                    let resp_body =
                        json!({ "success": true, "message": "Test achievement alert triggered!" });
                    let resp = Response::from_string(resp_body.to_string())
                        .with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/config" && method == Method::Get {
                    let cfg = config_provider.lock().unwrap().clone();
                    let resp = Response::from_string(
                        serde_json::to_string(&cfg).unwrap_or_default(),
                    )
                    .with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/config" && method == Method::Post {
                    let mut body = String::new();
                    let _ = req.as_reader().read_to_string(&mut body);

                    let mut cfg = config_provider.lock().unwrap();
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(&body) {
                        if let Some(dir) =
                            val.get("backup_directory").and_then(|v| v.as_str())
                        {
                            cfg.backup_directory = PathBuf::from(dir);
                        }
                        if let Some(bpd) = val.get("backups_per_day").and_then(|v| v.as_u64()) {
                            cfg.set_backups_per_day(bpd as u32);
                        } else if let Some(interval) = val
                            .get("auto_backup_interval_minutes")
                            .and_then(|v| v.as_u64())
                        {
                            cfg.auto_backup_interval_minutes = interval;
                            cfg.backups_per_day = Some((1440 / interval.max(1)).max(1) as u32);
                        }
                        if let Some(sound) =
                            val.get("sound_enabled").and_then(|v| v.as_bool())
                        {
                            cfg.sound_enabled = sound;
                        }
                        if let Some(notif) =
                            val.get("notifications_enabled").and_then(|v| v.as_bool())
                        {
                            cfg.notifications_enabled = notif;
                        }
                        if let Some(auto) =
                            val.get("auto_backup_enabled").and_then(|v| v.as_bool())
                        {
                            cfg.auto_backup_enabled = auto;
                        }
                        if let Some(gdrive) = val.get("google_drive_exe") {
                            if gdrive.is_null() || gdrive.as_str() == Some("") {
                                cfg.google_drive_exe = None;
                            } else if let Some(s) = gdrive.as_str() {
                                cfg.google_drive_exe = Some(PathBuf::from(s));
                            }
                        }
                        save_config(&cfg);
                    }

                    let resp_body =
                        json!({ "success": true, "message": "Settings updated" });
                    let resp = Response::from_string(resp_body.to_string())
                        .with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/test-sound" && method == Method::Post {
                    #[link(name = "user32")]
                    extern "system" {
                        fn MessageBeep(uType: u32) -> i32;
                    }
                    unsafe {
                        MessageBeep(0x00000040); // MB_ICONASTERISK
                    }
                    let resp_body = json!({ "success": true, "message": "Audio chime played" });
                    let resp = Response::from_string(resp_body.to_string())
                        .with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/open-folder" && method == Method::Post {
                    let cfg = config_provider.lock().unwrap().clone();
                    let _ = open::that(&cfg.backup_directory);
                    let resp_body = json!({ "success": true });
                    let resp = Response::from_string(resp_body.to_string())
                        .with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/open-config-folder" && method == Method::Post {
                    let config_dir = config::get_config_dir();
                    let _ = open::that(&config_dir);
                    let resp_body = json!({ "success": true, "path": config_dir.to_string_lossy() });
                    let resp = Response::from_string(resp_body.to_string())
                        .with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/test/write-test" && method == Method::Post {
                    let cfg = config_provider.lock().unwrap().clone();
                    let _ = std::fs::create_dir_all(&cfg.backup_directory);
                    let test_file = cfg.backup_directory.join(".sager_diag_test.tmp");
                    let start_time = std::time::Instant::now();
                    
                    match std::fs::write(&test_file, b"Sager Google Drive sync permission & latency test OK") {
                        Ok(_) => {
                            let elapsed_ms = start_time.elapsed().as_secs_f64() * 1000.0;
                            let _ = std::fs::remove_file(&test_file);
                            let resp_body = json!({
                                "success": true,
                                "message": format!("Write test passed in {:.2} ms! Directory is ready for cloud sync.", elapsed_ms),
                                "latency_ms": elapsed_ms,
                                "path": cfg.backup_directory.to_string_lossy()
                            });
                            let resp = Response::from_string(resp_body.to_string()).with_header(json_header);
                            let _ = req.respond(resp);
                        }
                        Err(e) => {
                            let resp_body = json!({
                                "success": false,
                                "message": format!("Write permission failed: {}", e),
                                "path": cfg.backup_directory.to_string_lossy()
                            });
                            let resp = Response::from_string(resp_body.to_string()).with_header(json_header);
                            let _ = req.respond(resp);
                        }
                    }
                    continue;
                }

                if url == "/api/test/custom-achievement" && method == Method::Post {
                    let mut body = String::new();
                    let _ = req.as_reader().read_to_string(&mut body);
                    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap_or_default();

                    let game_name = parsed.get("game_name").and_then(|v| v.as_str()).unwrap_or("Project Lightning / Steam");
                    let title = parsed.get("title").and_then(|v| v.as_str()).unwrap_or("Achievement Unlocked!");
                    let desc = parsed.get("description").and_then(|v| v.as_str()).unwrap_or("Sager test achievement verified!");

                    let now = chrono::Utc::now().timestamp();
                    let notif = AchievementNotification {
                        game_name: game_name.to_string(),
                        title: title.to_string(),
                        description: desc.to_string(),
                        unlock_time: now,
                    };
                    let _ = notifier_tx.send(notif);

                    let ach = Achievement {
                        id: format!("test_ach_{}", now),
                        game_name: game_name.to_string(),
                        app_id: Some("0".to_string()),
                        title: title.to_string(),
                        description: desc.to_string(),
                        unlocked: true,
                        unlock_time: Some(now),
                        icon_path: None,
                    };
                    crate::achievements::record_unlocked_achievement(&ach);

                    let resp_body = json!({
                        "success": true,
                        "message": format!("Notification popped for '{}' in {}", title, game_name)
                    });
                    let resp = Response::from_string(resp_body.to_string()).with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                if url == "/api/test/diagnostics" && method == Method::Get {
                    let cfg = config_provider.lock().unwrap().clone();
                    let drive = config::get_drive_status(&cfg);
                    let games = SaveScanner::scan_all(&cfg);
                    let total_games_size: u64 = games.iter().map(|g| g.total_size_bytes).sum();
                    let backups = SaveRestore::list_all_backups(&cfg);
                    let total_archives: usize = backups.iter().map(|b| b.1.len()).sum();
                    let total_backup_size: u64 = backups.iter().flat_map(|b| &b.1).map(|a| a.size_bytes).sum();
                    let achievements = load_achievements_history();

                    // Check write permission
                    let _ = std::fs::create_dir_all(&cfg.backup_directory);
                    let test_file = cfg.backup_directory.join(".sager_diag_test.tmp");
                    let start_time = std::time::Instant::now();
                    let (write_ok, latency_ms) = match std::fs::write(&test_file, b"Sager diag test") {
                        Ok(_) => {
                            let ms = start_time.elapsed().as_secs_f64() * 1000.0;
                            let _ = std::fs::remove_file(&test_file);
                            (true, ms)
                        }
                        Err(_) => (false, 0.0)
                    };

                    // Check Windows Startup shortcut
                    let startup_installed = if let Ok(appdata) = std::env::var("APPDATA") {
                        let lnk = PathBuf::from(appdata).join(r"Microsoft\Windows\Start Menu\Programs\Startup\Sager.lnk");
                        lnk.exists()
                    } else {
                        false
                    };

                    let diag = json!({
                        "google_drive": {
                            "running": drive.google_drive_running,
                            "installed": drive.google_drive_installed,
                            "backup_dir": cfg.backup_directory.to_string_lossy(),
                            "backup_dir_exists": drive.backup_dir_exists,
                            "backup_dir_is_google_drive": drive.backup_dir_is_google_drive,
                            "write_test_passed": write_ok,
                            "write_latency_ms": latency_ms
                        },
                        "watcher": {
                            "steam_detected": cfg.steam_path.is_some(),
                            "steam_path": cfg.steam_path.as_ref().map(|p| p.to_string_lossy()),
                            "goldberg_detected": cfg.goldberg_saves_path.is_some(),
                            "goldberg_path": cfg.goldberg_saves_path.as_ref().map(|p| p.to_string_lossy()),
                            "project_lightning_detected": cfg.project_lightning_path.is_some(),
                            "project_lightning_path": cfg.project_lightning_path.as_ref().map(|p| p.to_string_lossy()),
                        },
                        "save_scanner": {
                            "games_count": games.len(),
                            "total_size_bytes": total_games_size
                        },
                        "cloud_backups": {
                            "total_archives": total_archives,
                            "total_size_bytes": total_backup_size
                        },
                        "achievements": {
                            "total_unlocked": achievements.len()
                        },
                        "system": {
                            "sound_enabled": cfg.sound_enabled,
                            "notifications_enabled": cfg.notifications_enabled,
                            "auto_backup_enabled": cfg.auto_backup_enabled,
                            "backups_per_day": cfg.get_backups_per_day(),
                            "startup_shortcut_installed": startup_installed,
                            "config_path": config::get_config_file_path().to_string_lossy()
                        }
                    });

                    let resp = Response::from_string(diag.to_string()).with_header(json_header);
                    let _ = req.respond(resp);
                    continue;
                }

                let resp =
                    Response::from_string("Not Found").with_status_code(StatusCode(404));
                let _ = req.respond(resp);
            }
        });
    }
}
