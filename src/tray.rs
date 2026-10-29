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

#[link(name = "user32")]
extern "system" {
    fn MessageBeep(uType: u32) -> i32;
}

pub const WM_TRAY_CALLBACK: u32 = WM_APP + 1;

pub const ID_TRAY_TITLE: usize = 1000;
pub const ID_OPEN_DASHBOARD: usize = 1001;
pub const ID_BACKUP_ALL: usize = 1002;
pub const ID_OPEN_BACKUP_FOLDER: usize = 1003;
pub const ID_TEST_ACHIEVEMENT: usize = 1004;
pub const ID_SETTINGS: usize = 1005;
pub const ID_EXIT: usize = 1006;
pub const ID_LAUNCH_GDRIVE: usize = 1007;
pub const ID_GDRIVE_STATUS: usize = 1008;

pub fn to_wide_chars(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
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
}

impl SystemTray {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let class_name = to_wide_chars("SagerTrayClass");
            let hinstance = GetModuleHandleW(null());

            let wc = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(tray_window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: hinstance,
                hIcon: LoadIconW(null_mut(), IDI_APPLICATION),
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

            let icon = LoadIconW(null_mut(), IDI_APPLICATION);

            let mut nid = std::mem::zeroed::<NOTIFYICONDATAW>();
            nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
            nid.hWnd = hwnd;
            nid.uID = 1;
            nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
            nid.uCallbackMessage = WM_TRAY_CALLBACK;
            nid.hIcon = icon;

            let tip = to_wide_chars("Sager - Game Companion & Save Sync");
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

/// Build and show the context menu with live Google Drive status
fn show_context_menu(hwnd: HWND) {
    unsafe {
        let mut pt = POINT { x: 0, y: 0 };
        GetCursorPos(&mut pt);
        let hmenu = CreatePopupMenu();

        let cfg = config::load_config();
        let drive_status = config::get_drive_status(&cfg);

        // ── Title ──
        let title_text = to_wide_chars("Sager v0.1.0");
        AppendMenuW(hmenu, MF_STRING | MF_GRAYED, ID_TRAY_TITLE, title_text.as_ptr());
        AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

        // ── Google Drive Status Section ──
        let gdrive_label = if drive_status.google_drive_running {
            to_wide_chars("Google Drive: Running")
        } else if drive_status.google_drive_installed {
            to_wide_chars("Google Drive: Stopped")
        } else {
            to_wide_chars("Google Drive: Not Installed")
        };
        AppendMenuW(hmenu, MF_STRING | MF_GRAYED, ID_GDRIVE_STATUS, gdrive_label.as_ptr());

        if !drive_status.google_drive_running && drive_status.google_drive_installed {
            let launch_text = to_wide_chars("Launch Google Drive");
            AppendMenuW(hmenu, MF_STRING, ID_LAUNCH_GDRIVE, launch_text.as_ptr());
        }

        let dir_status = if drive_status.backup_dir_exists {
            format!("Backup: {}", cfg.backup_directory.display())
        } else {
            format!("Backup: {} (missing)", cfg.backup_directory.display())
        };
        let dir_text = to_wide_chars(&dir_status);
        AppendMenuW(hmenu, MF_STRING | MF_GRAYED, 0, dir_text.as_ptr());

        AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

        // ── Actions ──
        let dash_text = to_wide_chars("Open Dashboard");
        AppendMenuW(hmenu, MF_STRING, ID_OPEN_DASHBOARD, dash_text.as_ptr());

        let backup_text = to_wide_chars("Backup All Game Saves Now");
        AppendMenuW(hmenu, MF_STRING, ID_BACKUP_ALL, backup_text.as_ptr());

        let folder_text = to_wide_chars("Open Backup Folder");
        AppendMenuW(hmenu, MF_STRING, ID_OPEN_BACKUP_FOLDER, folder_text.as_ptr());

        AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

        let test_text = to_wide_chars("Test Achievement Alert");
        AppendMenuW(hmenu, MF_STRING, ID_TEST_ACHIEVEMENT, test_text.as_ptr());

        let settings_text = to_wide_chars("Settings (Dashboard)");
        AppendMenuW(hmenu, MF_STRING, ID_SETTINGS, settings_text.as_ptr());

        AppendMenuW(hmenu, MF_SEPARATOR, 0, null());

        let exit_text = to_wide_chars("Exit Sager");
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
                    // Left-click: open the backup folder directly
                    let cfg = config::load_config();
                    if cfg.backup_directory.exists() {
                        let _ = open::that(&cfg.backup_directory);
                    } else {
                        // Backup dir doesn't exist, open dashboard instead
                        let _ = open::that("http://127.0.0.1:3939");
                    }
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
                ID_EXIT => {
                    let handle = TrayHandle(hwnd as isize);
                    handle.remove_icon();
                    PostQuitMessage(0);
                }
                _ => {}
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
