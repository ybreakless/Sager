use crate::achievements::AchievementNotification;
use crate::config;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Shell::{
    Shell_NotifyIconW, NIF_ICON, NIF_INFO, NIF_MESSAGE, NIF_TIP, NIIF_INFO, NIM_ADD, NIM_DELETE,
    NIM_MODIFY, NOTIFYICONDATAW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DispatchMessageW,
    GetCursorPos, GetMessageW, LoadIconW, PostQuitMessage, RegisterClassW,
    SetForegroundWindow, TrackPopupMenu, TranslateMessage, IDI_APPLICATION,
    MB_ICONASTERISK, MF_GRAYED, MF_SEPARATOR, MF_STRING, TPM_BOTTOMALIGN, TPM_RIGHTBUTTON,
    WM_APP, WM_COMMAND, WM_DESTROY, WM_LBUTTONUP, WM_RBUTTONUP, WNDCLASSW,
    WS_OVERLAPPEDWINDOW,
};

pub const MF_POPUP: u32 = 0x00000010;
pub const WM_LBUTTONDBLCLK: u32 = 0x0203;

#[repr(C)]
struct IconInfo {
    f_icon: i32,
    x_hotspot: u32,
    y_hotspot: u32,
    hbm_mask: isize,
    hbm_color: isize,
}

#[link(name = "user32")]
extern "system" {
    fn MessageBeep(uType: u32) -> i32;
    fn CreateIconIndirect(piconinfo: *const IconInfo) -> isize;
    fn DestroyIcon(hicon: isize) -> i32;
    fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> i32;
    fn IsWindowVisible(hWnd: HWND) -> i32;
}

#[link(name = "kernel32")]
extern "system" {
    fn GetConsoleWindow() -> HWND;
}

pub const SW_HIDE: i32 = 0;
pub const SW_SHOW: i32 = 5;

pub fn hide_console() -> bool {
    unsafe {
        let hwnd = GetConsoleWindow();
        if !hwnd.is_null() {
            ShowWindow(hwnd, SW_HIDE);
            true
        } else {
            false
        }
    }
}

pub fn show_console() -> bool {
    unsafe {
        let hwnd = GetConsoleWindow();
        if !hwnd.is_null() {
            ShowWindow(hwnd, SW_SHOW);
            SetForegroundWindow(hwnd);
            true
        } else {
            false
        }
    }
}

pub fn is_console_visible() -> bool {
    unsafe {
        let hwnd = GetConsoleWindow();
        if !hwnd.is_null() {
            IsWindowVisible(hwnd) != 0
        } else {
            false
        }
    }
}

pub fn toggle_console() -> bool {
    if is_console_visible() {
        hide_console();
        false
    } else {
        show_console();
        true
    }
}

#[link(name = "gdi32")]
extern "system" {
    fn CreateBitmap(
        nWidth: i32,
        nHeight: i32,
        nPlanes: u32,
        nBitCount: u32,
        lpBits: *const core::ffi::c_void,
    ) -> isize;
    fn DeleteObject(ho: isize) -> i32;
}

pub const WM_TRAY_CALLBACK: u32 = WM_APP + 1;

pub const ID_TRAY_TITLE: usize = 1000;
pub const ID_STATUS_HEADER: usize = 1001;
pub const ID_OPEN_DASHBOARD: usize = 1002;
pub const ID_BACKUP_ALL: usize = 1003;
pub const ID_OPEN_BACKUP_FOLDER: usize = 1004;
pub const ID_TEST_ACHIEVEMENT: usize = 1005;
pub const ID_SETTINGS: usize = 1006;
pub const ID_EXIT: usize = 1007;
pub const ID_LAUNCH_GDRIVE: usize = 1008;
pub const ID_GDRIVE_STATUS: usize = 1009;
pub const ID_TOGGLE_CONSOLE: usize = 1010;
pub const ID_RUN_BACKGROUND: usize = 1011;
pub const ID_GAME_BACKUP_BASE: usize = 2000;

pub fn to_wide_chars(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

/// Dynamically creates a sharp 32x32 32-bit ARGB padlock with star logo icon for the Windows system tray
pub fn create_sager_icon() -> isize {
    unsafe {
        let mut pixels = [0u32; 32 * 32];

        for y in 0..32 {
            for x in 0..32 {
                let dx = (x as f32 - 15.5).abs();
                let dy = (y as f32 - 15.5).abs();
                let dist = (dx.powi(4) + dy.powi(4)).powf(0.25);

                if dist > 14.5 {
                    pixels[y * 32 + x] = 0; // 100% Transparent outside badge
                    continue;
                }

                // Outer border ring: glowing electric cyan
                if dist >= 13.0 {
                    let r = 14u8;
                    let g = 165u8;
                    let b = 233u8;
                    pixels[y * 32 + x] =
                        (255u32 << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                    continue;
                }

                // Check if inside the padlock emblem:
                let mut is_lock = false;

                // 1. Shackle: arched top (y from 6 to 13, x from 11 to 20)
                let shackle_dx = (x as f32 - 15.5).abs();
                let shackle_dy = (y as f32 - 11.5).min(0.0);
                let shackle_r = (shackle_dx * shackle_dx + shackle_dy * shackle_dy).sqrt();

                if y >= 6 && y <= 13 && shackle_dx <= 5.2 && (shackle_dx >= 2.5 || (shackle_dy < 0.0 && shackle_r >= 2.5 && shackle_r <= 5.2)) {
                    is_lock = true;
                }

                // 2. Lock body: rectangular with rounded bottom corners (y from 13 to 25, x from 9 to 22)
                if y >= 13 && y <= 25 && x >= 9 && x <= 22 {
                    if !((y >= 24) && (x == 9 || x == 22)) {
                        is_lock = true;
                    }
                }

                // 3. Five-pointed star cutout inside lock body:
                let mut is_star = false;
                if is_lock && y >= 16 && y <= 23 && x >= 12 && x <= 19 {
                    let sx = x as f32 - 15.5;
                    let sy = y as f32 - 19.5;
                    let sr = (sx * sx + sy * sy).sqrt();
                    let angle = sy.atan2(sx);
                    let star_radius = 1.3 + 2.0 * (5.0 * angle - std::f32::consts::FRAC_PI_2).cos().max(0.0);
                    if sr <= star_radius {
                        is_star = true;
                    }
                }

                if is_star {
                    // Radiant golden amber star emblem
                    let r = 251u8;
                    let g = 191u8;
                    let b = 36u8;
                    pixels[y * 32 + x] =
                        (255u32 << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                } else if is_lock {
                    // Padlock body in crisp clean white/light cyan
                    let r = 245u8;
                    let g = 250u8;
                    let b = 255u8;
                    pixels[y * 32 + x] =
                        (255u32 << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                } else {
                    // Deep obsidian gamer background
                    let bg_r = 15u8;
                    let bg_g = 23u8;
                    let bg_b = 42u8;
                    pixels[y * 32 + x] =
                        (245u32 << 24) | ((bg_r as u32) << 16) | ((bg_g as u32) << 8) | (bg_b as u32);
                }
            }
        }

        let hbm_color = CreateBitmap(32, 32, 1, 32, pixels.as_ptr() as *const _);
        let mask = [0u8; 128]; // 32 * 32 / 8 = 128 bytes of zeros
        let hbm_mask = CreateBitmap(32, 32, 1, 1, mask.as_ptr() as *const _);

        if hbm_color == 0 || hbm_mask == 0 {
            if hbm_color != 0 {
                DeleteObject(hbm_color);
            }
            if hbm_mask != 0 {
                DeleteObject(hbm_mask);
            }
            return 0;
        }

        let icon_info = IconInfo {
            f_icon: 1,
            x_hotspot: 0,
            y_hotspot: 0,
            hbm_mask,
            hbm_color,
        };

        let hicon = CreateIconIndirect(&icon_info);
        DeleteObject(hbm_color);
        DeleteObject(hbm_mask);

        hicon
    }
}

#[derive(Clone, Copy)]
pub struct TrayHandle(pub isize);
unsafe impl Send for TrayHandle {}
unsafe impl Sync for TrayHandle {}

impl TrayHandle {
    pub fn show_balloon_notification(&self, title: &str, message: &str, sound: bool) {
        unsafe {
            let hwnd = self.0 as HWND;
            let mut nid = std::mem::zeroed::<NOTIFYICONDATAW>();
            nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
            nid.hWnd = hwnd;
            nid.uID = 1;
            nid.uFlags = NIF_INFO;
            nid.dwInfoFlags = NIIF_INFO;

            let title_wide = to_wide_chars(title);
            let t_len = title_wide.len().min(nid.szInfoTitle.len() - 1);
            nid.szInfoTitle[..t_len].copy_from_slice(&title_wide[..t_len]);

            let msg_wide = to_wide_chars(message);
            let m_len = msg_wide.len().min(nid.szInfo.len() - 1);
            nid.szInfo[..m_len].copy_from_slice(&msg_wide[..m_len]);

            Shell_NotifyIconW(NIM_MODIFY, &nid);

            if sound {
                MessageBeep(MB_ICONASTERISK);
            }
        }
    }

    pub fn remove_icon(&self) {
        unsafe {
            let hwnd = self.0 as HWND;
            let mut nid = std::mem::zeroed::<NOTIFYICONDATAW>();
            nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
            nid.hWnd = hwnd;
            nid.uID = 1;
            Shell_NotifyIconW(NIM_DELETE, &nid);
        }
    }
}

pub struct SystemTray {
    pub handle: TrayHandle,
    pub icon_handle: isize,
}

impl SystemTray {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let class_name = to_wide_chars("SagerTrayClass");
            let hinstance = GetModuleHandleW(null());

            // Create custom "S" logo icon
            let s_icon = create_sager_icon();
            let icon = if s_icon != 0 {
                s_icon as windows_sys::Win32::UI::WindowsAndMessaging::HICON
            } else {
                LoadIconW(null_mut(), IDI_APPLICATION)
            };

            let wc = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(tray_window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: hinstance,
                hIcon: icon,
                hCursor: null_mut(),
                hbrBackground: null_mut(),
                lpszMenuName: null(),
                lpszClassName: class_name.as_ptr(),
            };

            let _ = RegisterClassW(&wc);

            let window_title = to_wide_chars("Sager Game Companion");
            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                window_title.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                0,
                0,
                0,
                0,
                null_mut(),
                null_mut(),
                hinstance,
                null(),
            );

            if hwnd.is_null() {
                return Err("Failed to create tray window".to_string());
            }

            let mut nid = std::mem::zeroed::<NOTIFYICONDATAW>();
            nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
            nid.hWnd = hwnd;
            nid.uID = 1;
            nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
            nid.uCallbackMessage = WM_TRAY_CALLBACK;
            nid.hIcon = icon;

            let tip = to_wide_chars("Sager - Game Companion & Cloud Save Sync");
            let len = tip.len().min(nid.szTip.len() - 1);
            nid.szTip[..len].copy_from_slice(&tip[..len]);

            let add_res = Shell_NotifyIconW(NIM_ADD, &nid);

            if add_res == 0 {
                return Err(
                    "Failed to add system tray icon (Explorer taskbar not ready or isolated session)"
                        .to_string(),
                );
            }

            Ok(Self {
                handle: TrayHandle(hwnd as isize),
                icon_handle: s_icon,
            })
        }
    }

    pub fn handle(&self) -> TrayHandle {
        self.handle
    }

    pub fn run_message_loop(&self) {
        unsafe {
            let mut msg = std::mem::zeroed();
            while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            if self.icon_handle != 0 {
                DestroyIcon(self.icon_handle);
            }
        }
    }
}

pub fn start_notification_listener(
    tray_handle: TrayHandle,
    rx: Receiver<AchievementNotification>,
    config_provider: Arc<Mutex<config::SagerConfig>>,
) {
    thread::spawn(move || {
        while let Ok(notif) = rx.recv() {
            let (notifications_enabled, sound_enabled) = {
                let cfg = config_provider.lock().unwrap();
                (cfg.notifications_enabled, cfg.sound_enabled)
            };

            if notifications_enabled {
                let title = format!("Achievement Unlocked! ({})", notif.game_name);
                let message = if notif.description.is_empty() {
                    notif.title
                } else {
                    format!("{}: {}", notif.title, notif.description)
                };

                tray_handle.show_balloon_notification(&title, &message, sound_enabled);
            }
        }
    });
}

/// Build and show rich context menu with live games list, Google Drive status, and backup triggers
fn show_context_menu(hwnd: HWND) {
    unsafe {
        let mut pt = POINT { x: 0, y: 0 };
        GetCursorPos(&mut pt);
        let hmenu = CreatePopupMenu();

        let cfg = config::load_config();
        let drive_status = config::get_drive_status(&cfg);
        let games = crate::backup::SaveScanner::scan_all(&cfg);
        let achievements = crate::achievements::load_achievements_history();
        let bpd = cfg.get_backups_per_day();

        // ── Title & Status Header ──
        let title_text = to_wide_chars("🎮 SAGER - Game Companion");
        AppendMenuW(hmenu, MF_STRING | MF_GRAYED, ID_TRAY_TITLE, title_text.as_ptr());

        let status_info = format!("📊 Status: {} Games | {} Trophies", games.len(), achievements.len());
        AppendMenuW(hmenu, MF_STRING | MF_GRAYED, ID_STATUS_HEADER, to_wide_chars(&status_info).as_ptr());

        let launcher_info = format!("🚀 Launcher: {}", cfg.preferred_launcher);
        AppendMenuW(hmenu, MF_STRING | MF_GRAYED, 0, to_wide_chars(&launcher_info).as_ptr());

        let sched_info = if cfg.auto_backup_enabled {
            format!("⏱️ Auto-Backup: {}x / day (Every {} min)", bpd, cfg.auto_backup_interval_minutes)
        } else {
            "⏱️ Auto-Backup: Disabled (Manual only)".to_string()
        };
        AppendMenuW(hmenu, MF_STRING | MF_GRAYED, 0, to_wide_chars(&sched_info).as_ptr());

        AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

        // ── Discovered Games Submenu ──
        let games_submenu = CreatePopupMenu();
        if games.is_empty() {
            AppendMenuW(games_submenu, MF_STRING | MF_GRAYED, 0, to_wide_chars("(No games detected yet)").as_ptr());
        } else {
            for (i, game) in games.iter().take(15).enumerate() {
                let item_text = format!("💾 Backup {}", game.name);
                AppendMenuW(games_submenu, MF_STRING, ID_GAME_BACKUP_BASE + i, to_wide_chars(&item_text).as_ptr());
            }
            if games.len() > 15 {
                let more_text = format!("... +{} more in Dashboard", games.len() - 15);
                AppendMenuW(games_submenu, MF_STRING | MF_GRAYED, 0, to_wide_chars(&more_text).as_ptr());
            }
        }
        let games_menu_label = format!("🎮 Discovered Games ({})", games.len());
        AppendMenuW(hmenu, MF_POPUP, games_submenu as usize, to_wide_chars(&games_menu_label).as_ptr());

        let backup_all_label = format!("💾 Backup All Games ({}) Now", games.len());
        AppendMenuW(hmenu, MF_STRING, ID_BACKUP_ALL, to_wide_chars(&backup_all_label).as_ptr());

        AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

        // ── Google Drive Section ──
        let gdrive_label = if drive_status.google_drive_running {
            to_wide_chars("☁️ Google Drive: Running")
        } else if drive_status.google_drive_installed {
            to_wide_chars("☁️ Google Drive: Stopped")
        } else {
            to_wide_chars("☁️ Google Drive: Not Installed")
        };
        AppendMenuW(hmenu, MF_STRING | MF_GRAYED, ID_GDRIVE_STATUS, gdrive_label.as_ptr());

        if !drive_status.google_drive_running && drive_status.google_drive_installed {
            let launch_text = to_wide_chars("🚀 Launch Google Drive");
            AppendMenuW(hmenu, MF_STRING, ID_LAUNCH_GDRIVE, launch_text.as_ptr());
        }

        let folder_text = to_wide_chars("📂 Open Backup Folder (Google Drive)");
        AppendMenuW(hmenu, MF_STRING, ID_OPEN_BACKUP_FOLDER, folder_text.as_ptr());

        AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

        // ── Achievements & Dashboard ──
        let ach_count_text = format!("🏆 Achievements: {} Unlocked", achievements.len());
        AppendMenuW(hmenu, MF_STRING | MF_GRAYED, 0, to_wide_chars(&ach_count_text).as_ptr());

        let test_text = to_wide_chars("🔔 Test Achievement Alert");
        AppendMenuW(hmenu, MF_STRING, ID_TEST_ACHIEVEMENT, test_text.as_ptr());

        let dash_text = to_wide_chars("🌐 Open Web Dashboard");
        AppendMenuW(hmenu, MF_STRING, ID_OPEN_DASHBOARD, dash_text.as_ptr());

        let settings_text = to_wide_chars("⚙️ Settings & Schedule");
        AppendMenuW(hmenu, MF_STRING, ID_SETTINGS, settings_text.as_ptr());

        AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

        // ── Window & Background Mode ──
        if is_console_visible() {
            let bg_text = to_wide_chars("🕶️ Run in Background (Hide Console)");
            AppendMenuW(hmenu, MF_STRING, ID_RUN_BACKGROUND, bg_text.as_ptr());
        } else {
            let show_text = to_wide_chars("👁️ Make Console Appear (Show Window)");
            AppendMenuW(hmenu, MF_STRING, ID_TOGGLE_CONSOLE, show_text.as_ptr());
        }

        AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

        let exit_text = to_wide_chars("❌ Exit Sager");
        AppendMenuW(hmenu, MF_STRING, ID_EXIT, exit_text.as_ptr());

        SetForegroundWindow(hwnd);
        TrackPopupMenu(
            hmenu,
            TPM_RIGHTBUTTON | TPM_BOTTOMALIGN,
            pt.x,
            pt.y,
            0,
            hwnd,
            null(),
        );
        DestroyMenu(hmenu);
    }
}

pub unsafe extern "system" fn tray_window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAY_CALLBACK => {
            let event = lparam as u32;
            match event {
                WM_RBUTTONUP => {
                    show_context_menu(hwnd);
                }
                WM_LBUTTONUP => {
                    // Left-click: Open ProtonVPN-style Sager Companion Dashboard immediately
                    let _ = open::that("http://127.0.0.1:3939");
                }
                WM_LBUTTONDBLCLK => {
                    // Double left-click: open web dashboard
                    let _ = open::that("http://127.0.0.1:3939");
                }
                _ => {}
            }
            0
        }
        WM_COMMAND => {
            let cmd = wparam as usize;
            match cmd {
                ID_OPEN_DASHBOARD | ID_SETTINGS => {
                    let _ = open::that("http://127.0.0.1:3939");
                }
                ID_BACKUP_ALL => {
                    let cfg = config::load_config();
                    let res = crate::backup::SaveArchiver::backup_all(&cfg);
                    let success_count = res.iter().filter(|r| r.is_ok()).count();
                    let total = res.len();

                    let handle = TrayHandle(hwnd as isize);
                    handle.show_balloon_notification(
                        "Sager Backup Complete",
                        &format!(
                            "Backed up {} of {} games to {}",
                            success_count,
                            total,
                            cfg.backup_directory.display()
                        ),
                        cfg.sound_enabled,
                    );
                }
                ID_OPEN_BACKUP_FOLDER => {
                    let cfg = config::load_config();
                    let _ = open::that(&cfg.backup_directory);
                }
                ID_TEST_ACHIEVEMENT => {
                    let handle = TrayHandle(hwnd as isize);
                    handle.show_balloon_notification(
                        "Achievement Unlocked! (Test)",
                        "Sager Master: Achievement tracking & save backup working!",
                        true,
                    );
                }
                ID_LAUNCH_GDRIVE => {
                    let cfg = config::load_config();
                    match config::launch_google_drive(&cfg) {
                        Ok(()) => {
                            let handle = TrayHandle(hwnd as isize);
                            handle.show_balloon_notification(
                                "Google Drive Launched",
                                "Google Drive for Desktop is starting up...",
                                false,
                            );
                        }
                        Err(e) => {
                            let handle = TrayHandle(hwnd as isize);
                            handle.show_balloon_notification(
                                "Google Drive Error",
                                &e,
                                true,
                            );
                        }
                    }
                }
                ID_RUN_BACKGROUND => {
                    hide_console();
                    let handle = TrayHandle(hwnd as isize);
                    handle.show_balloon_notification(
                        "Sager Running in Background",
                        "Sager is now operating silently in your system tray.",
                        false,
                    );
                }
                ID_TOGGLE_CONSOLE => {
                    show_console();
                    let handle = TrayHandle(hwnd as isize);
                    handle.show_balloon_notification(
                        "Sager Console Restored",
                        "Console window made visible.",
                        false,
                    );
                }
                ID_EXIT => {
                    let handle = TrayHandle(hwnd as isize);
                    handle.remove_icon();
                    PostQuitMessage(0);
                }
                _ => {
                    // Check if it's an individual game backup command from the submenu
                    if cmd >= ID_GAME_BACKUP_BASE && cmd < ID_GAME_BACKUP_BASE + 100 {
                        let game_idx = cmd - ID_GAME_BACKUP_BASE;
                        let cfg = config::load_config();
                        let games = crate::backup::SaveScanner::scan_all(&cfg);
                        if let Some(game) = games.get(game_idx) {
                            let handle = TrayHandle(hwnd as isize);
                            match crate::backup::SaveArchiver::backup_game(&game.name, &game.source_path, &cfg) {
                                Ok(info) => {
                                    handle.show_balloon_notification(
                                        "Game Backup Complete",
                                        &format!("{} backed up ({}) to Google Drive!", game.name, info.file_name),
                                        cfg.sound_enabled,
                                    );
                                }
                                Err(e) => {
                                    handle.show_balloon_notification(
                                        "Backup Failed",
                                        &format!("Failed to backup {}: {}", game.name, e),
                                        cfg.sound_enabled,
                                    );
                                }
                            }
                        }
                    }
                }
            }
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
