#![windows_subsystem = "windows"]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

const GITHUB_RELEASE_ZIP: &str =
    "https://github.com/ybreakless/Sager/releases/download/v0.2.0/sager-v0.2.0-windows-x64.zip";
const GITHUB_APK_URL: &str =
    "https://github.com/ybreakless/Sager/releases/download/v0.2.0/sager-companion-v0.2.0.apk";

const ID_BTN_INSTALL: isize = 101;
const ID_BTN_STARTUP: isize = 102;
const ID_BTN_LAUNCH: isize = 103;
const ID_BTN_ANDROID: isize = 104;
const ID_BTN_EXIT: isize = 105;

const SS_CENTER: u32 = 1;

static mut H_STATUS_LABEL: HWND = null_mut();
static mut H_STARTUP_LABEL: HWND = null_mut();
static mut H_PROGRESS_LABEL: HWND = null_mut();
static mut H_DARK_BRUSH: HBRUSH = null_mut();
static mut H_CARD_BRUSH: HBRUSH = null_mut();

fn get_install_dir() -> PathBuf {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        PathBuf::from(local_app_data).join("Sager")
    } else if let Ok(app_data) = std::env::var("APPDATA") {
        PathBuf::from(app_data).join("Sager")
    } else {
        PathBuf::from(r"C:\Sager")
    }
}

fn get_installed_exe_path() -> PathBuf {
    get_install_dir().join("sager.exe")
}

fn get_startup_shortcut_path() -> Option<PathBuf> {
    std::env::var("APPDATA").ok().map(|appdata| {
        PathBuf::from(appdata).join(r"Microsoft\Windows\Start Menu\Programs\Startup\Sager.lnk")
    })
}

fn is_installed() -> bool {
    get_installed_exe_path().exists()
}

fn is_startup_enabled() -> bool {
    get_startup_shortcut_path()
        .map(|p| p.exists())
        .unwrap_or(false)
}

fn is_sager_running() -> bool {
    // Quick test if port 3939 or process is alive
    if let Ok(mut stream) = std::net::TcpStream::connect("127.0.0.1:3939") {
        use std::io::Write;
        let _ = stream.write_all(b"HEAD /api/status HTTP/1.0\r\n\r\n");
        return true;
    }
    false
}

fn set_startup(enable: bool) -> Result<(), String> {
    let lnk = get_startup_shortcut_path().ok_or("Cannot resolve APPDATA")?;
    let exe = if is_installed() {
        get_installed_exe_path()
    } else if let Ok(cur) = std::env::current_exe() {
        let local_sager = cur.parent().unwrap_or(Path::new("")).join("sager.exe");
        if local_sager.exists() {
            local_sager
        } else {
            get_installed_exe_path()
        }
    } else {
        get_installed_exe_path()
    };

    if enable {
        if let Some(parent) = lnk.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let cur_dir = exe.parent().unwrap_or(Path::new("C:\\"));
        let ps_cmd = format!(
            "$s = (New-Object -ComObject WScript.Shell).CreateShortcut('{}'); \
             $s.TargetPath = '{}'; \
             $s.Arguments = '--background'; \
             $s.WorkingDirectory = '{}'; \
             $s.Save()",
            lnk.to_string_lossy().replace('\'', "''"),
            exe.to_string_lossy().replace('\'', "''"),
            cur_dir.to_string_lossy().replace('\'', "''"),
        );
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &ps_cmd])
            .output();
    } else if lnk.exists() {
        let _ = fs::remove_file(&lnk);
    }
    Ok(())
}

fn install_sager() -> Result<String, String> {
    let target_dir = get_install_dir();
    fs::create_dir_all(&target_dir).map_err(|e| format!("Failed to create folder: {}", e))?;

    let cur_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    // Check if we have a local sager.exe to install immediately
    let local_sager = cur_dir.join("sager.exe");
    let target_exe = target_dir.join("sager.exe");

    if local_sager.exists() && local_sager != target_exe {
        fs::copy(&local_sager, &target_exe)
            .map_err(|e| format!("Failed to copy sager.exe: {}", e))?;

        // Also copy assets directory if present
        let local_assets = cur_dir.join("assets");
        if local_assets.is_dir() {
            copy_dir_recursive(&local_assets, &target_dir.join("assets"))?;
        }
    } else {
        // Download latest release zip from GitHub
        let temp_zip = std::env::temp_dir().join("sager_download.zip");
        let ps_cmd = format!(
            "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; \
             Invoke-WebRequest -Uri '{}' -OutFile '{}' -UseBasicParsing; \
             Expand-Archive -Path '{}' -DestinationPath '{}' -Force; \
             Remove-Item -Force '{}'",
            GITHUB_RELEASE_ZIP,
            temp_zip.to_string_lossy().replace('\'', "''"),
            temp_zip.to_string_lossy().replace('\'', "''"),
            target_dir.to_string_lossy().replace('\'', "''"),
            temp_zip.to_string_lossy().replace('\'', "''"),
        );

        let out = Command::new("powershell")
            .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &ps_cmd])
            .output()
            .map_err(|e| format!("Download execution failed: {}", e))?;

        if !target_exe.exists() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            return Err(format!("Download failed: {}", stderr));
        }
    }

    // Create Desktop Shortcut
    if let Ok(userprofile) = std::env::var("USERPROFILE") {
        let desktop_lnk = PathBuf::from(userprofile).join(r"Desktop\Sager.lnk");
        let ps_cmd = format!(
            "$s = (New-Object -ComObject WScript.Shell).CreateShortcut('{}'); \
             $s.TargetPath = '{}'; \
             $s.WorkingDirectory = '{}'; \
             $s.Save()",
            desktop_lnk.to_string_lossy().replace('\'', "''"),
            target_exe.to_string_lossy().replace('\'', "''"),
            target_dir.to_string_lossy().replace('\'', "''"),
        );
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &ps_cmd])
            .output();
    }

    // Automatically enable Windows startup
    let _ = set_startup(true);

    Ok("Installed successfully in AppData\\Local\\Sager & Added to Startup!".to_string())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    if !dst.exists() {
        fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    }
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            let _ = fs::copy(&src_path, &dst_path);
        }
    }
    Ok(())
}

fn launch_sager() -> Result<(), String> {
    let target_exe = if is_installed() {
        get_installed_exe_path()
    } else {
        let cur = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("sager.exe")))
            .unwrap_or_else(get_installed_exe_path);
        if cur.exists() {
            cur
        } else {
            return Err("Sager executable not found. Please click 'Install Sager' first.".into());
        }
    };

    let cur_dir = target_exe.parent().unwrap_or(Path::new("C:\\"));
    Command::new(&target_exe)
        .current_dir(cur_dir)
        .spawn()
        .map_err(|e| format!("Failed to launch Sager: {}", e))?;

    let _ = open::that("http://127.0.0.1:3939");
    Ok(())
}

unsafe fn update_labels() {
    let running_str = if is_sager_running() {
        "● Active in Tray"
    } else {
        "○ Idle"
    };
    let install_text = if is_installed() {
        format!("Status: [ INSTALLED ] in AppData\\Local\\Sager ({})\0", running_str)
    } else {
        format!("Status: [ NOT INSTALLED ] Click 'Download & Install' below ({})\0", running_str)
    };
    let startup_text = if is_startup_enabled() {
        "Windows Boot Startup: [ ACTIVE ] (Launches silently in tray)\0".to_string()
    } else {
        "Windows Boot Startup: [ DISABLED ] Click 'Toggle Startup' to enable\0".to_string()
    };

    let install_utf16: Vec<u16> = install_text.encode_utf16().collect();
    let startup_utf16: Vec<u16> = startup_text.encode_utf16().collect();

    if !H_STATUS_LABEL.is_null() {
        SetWindowTextW(H_STATUS_LABEL, install_utf16.as_ptr());
    }
    if !H_STARTUP_LABEL.is_null() {
        SetWindowTextW(H_STARTUP_LABEL, startup_utf16.as_ptr());
    }
}

unsafe fn set_progress(text: &str) {
    if !H_PROGRESS_LABEL.is_null() {
        let mut msg = text.to_string();
        msg.push('\0');
        let utf16: Vec<u16> = msg.encode_utf16().collect();
        SetWindowTextW(H_PROGRESS_LABEL, utf16.as_ptr());
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            // Dark solid background brush
            H_DARK_BRUSH = CreateSolidBrush(0x000F0F0F); // #0F0F0F (pure dark)
            H_CARD_BRUSH = CreateSolidBrush(0x00181818); // #181818

            let hinstance = GetModuleHandleW(null_mut());

            fn to_w(s: &str) -> Vec<u16> {
                let mut v: Vec<u16> = s.encode_utf16().collect();
                v.push(0);
                v
            }

            // Header Title
            CreateWindowExW(
                0,
                to_w("STATIC").as_ptr(),
                to_w("★  S A G E R").as_ptr(),
                WS_CHILD | WS_VISIBLE | (SS_CENTER as u32),
                20,
                24,
                460,
                32,
                hwnd,
                null_mut(),
                hinstance,
                null_mut(),
            );

            // Subtitle
            CreateWindowExW(
                0,
                to_w("STATIC").as_ptr(),
                to_w("Steam Achievements & Save Cloud Hub — Minimal Setup & Launcher").as_ptr(),
                WS_CHILD | WS_VISIBLE | (SS_CENTER as u32),
                20,
                60,
                460,
                20,
                hwnd,
                null_mut(),
                hinstance,
                null_mut(),
            );

            // Status Label
            H_STATUS_LABEL = CreateWindowExW(
                0,
                to_w("STATIC").as_ptr(),
                to_w("Checking Sager status...").as_ptr(),
                WS_CHILD | WS_VISIBLE,
                30,
                100,
                440,
                22,
                hwnd,
                null_mut(),
                hinstance,
                null_mut(),
            );

            // Startup Label
            H_STARTUP_LABEL = CreateWindowExW(
                0,
                to_w("STATIC").as_ptr(),
                to_w("Checking Windows startup...").as_ptr(),
                WS_CHILD | WS_VISIBLE,
                30,
                126,
                440,
                22,
                hwnd,
                null_mut(),
                hinstance,
                null_mut(),
            );

            // Progress Message Label
            H_PROGRESS_LABEL = CreateWindowExW(
                0,
                to_w("STATIC").as_ptr(),
                to_w("Ready.").as_ptr(),
                WS_CHILD | WS_VISIBLE | (SS_CENTER as u32),
                20,
                156,
                460,
                22,
                hwnd,
                null_mut(),
                hinstance,
                null_mut(),
            );

            // Button 1: Download & Install Sager
            CreateWindowExW(
                0,
                to_w("BUTTON").as_ptr(),
                to_w("⬇  Download & Install Sager").as_ptr(),
                WS_CHILD | WS_VISIBLE | (BS_PUSHBUTTON as u32),
                30,
                190,
                440,
                38,
                hwnd,
                ID_BTN_INSTALL as HMENU,
                hinstance,
                null_mut(),
            );

            // Button 2: Toggle Windows Startup
            CreateWindowExW(
                0,
                to_w("BUTTON").as_ptr(),
                to_w("🚀  Toggle Windows Startup (Boot with PC)").as_ptr(),
                WS_CHILD | WS_VISIBLE | (BS_PUSHBUTTON as u32),
                30,
                236,
                440,
                38,
                hwnd,
                ID_BTN_STARTUP as HMENU,
                hinstance,
                null_mut(),
            );

            // Button 3: Launch Sager Now
            CreateWindowExW(
                0,
                to_w("BUTTON").as_ptr(),
                to_w("🎮  Launch Sager (System Tray & UI)").as_ptr(),
                WS_CHILD | WS_VISIBLE | (BS_PUSHBUTTON as u32),
                30,
                282,
                440,
                38,
                hwnd,
                ID_BTN_LAUNCH as HMENU,
                hinstance,
                null_mut(),
            );

            // Button 4: Android Companion App
            CreateWindowExW(
                0,
                to_w("BUTTON").as_ptr(),
                to_w("📱  Get Android Companion (.apk)").as_ptr(),
                WS_CHILD | WS_VISIBLE | (BS_PUSHBUTTON as u32),
                30,
                328,
                215,
                34,
                hwnd,
                ID_BTN_ANDROID as HMENU,
                hinstance,
                null_mut(),
            );

            // Button 5: Exit
            CreateWindowExW(
                0,
                to_w("BUTTON").as_ptr(),
                to_w("✕  Close Launcher").as_ptr(),
                WS_CHILD | WS_VISIBLE | (BS_PUSHBUTTON as u32),
                255,
                328,
                215,
                34,
                hwnd,
                ID_BTN_EXIT as HMENU,
                hinstance,
                null_mut(),
            );

            update_labels();
            0
        }
        WM_COMMAND => {
            let id = wparam as isize;
            match id {
                ID_BTN_INSTALL => {
                    set_progress("Downloading and installing Sager to AppData\\Local\\Sager...");
                    let res = install_sager();
                    match res {
                        Ok(msg) => {
                            set_progress(&msg);
                            update_labels();
                        }
                        Err(err) => {
                            set_progress(&format!("Error: {}", err));
                        }
                    }
                }
                ID_BTN_STARTUP => {
                    let next_state = !is_startup_enabled();
                    let _ = set_startup(next_state);
                    if next_state {
                        set_progress("✓ Added to Windows Startup (starts silently in tray).");
                    } else {
                        set_progress("✓ Removed from Windows Startup.");
                    }
                    update_labels();
                }
                ID_BTN_LAUNCH => {
                    set_progress("Starting Sager in System Tray and opening companion...");
                    match launch_sager() {
                        Ok(()) => {
                            set_progress("✓ Sager is running!");
                            update_labels();
                        }
                        Err(e) => {
                            set_progress(&format!("Launch failed: {}", e));
                        }
                    }
                }
                ID_BTN_ANDROID => {
                    let _ = open::that(GITHUB_APK_URL);
                    set_progress("Opening download for sager-companion-v0.2.0.apk...");
                }
                ID_BTN_EXIT => {
                    DestroyWindow(hwnd);
                }
                _ => {}
            }
            0
        }
        WM_CTLCOLORSTATIC => {
            let hdc = wparam as HDC;
            SetTextColor(hdc, 0x00FFFFFF); // Pure White Text
            SetBkColor(hdc, 0x000F0F0F); // Match Dark Background
            SetBkMode(hdc, TRANSPARENT as i32);
            H_DARK_BRUSH as LRESULT
        }
        WM_DESTROY => {
            if !H_DARK_BRUSH.is_null() {
                DeleteObject(H_DARK_BRUSH);
            }
            if !H_CARD_BRUSH.is_null() {
                DeleteObject(H_CARD_BRUSH);
            }
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--install" || a == "-i") {
        match install_sager() {
            Ok(msg) => println!("{}", msg),
            Err(e) => eprintln!("Error: {}", e),
        }
        return;
    }
    if args.iter().any(|a| a == "--startup") {
        let _ = set_startup(true);
        println!("Startup enabled.");
        return;
    }
    if args.iter().any(|a| a == "--uninstall-startup") {
        let _ = set_startup(false);
        println!("Startup disabled.");
        return;
    }

    // Fully automatic first-run experience: install, configure startup, and launch.
    let force_ui = args.iter().any(|a| a == "--ui" || a == "--setup");
    if !is_installed() && !force_ui {
        match install_sager() {
            Ok(_) => {
                let _ = launch_sager();
                return;
            }
            Err(e) => eprintln!("Automatic install failed: {}. Opening setup window...", e),
        }
    }

    unsafe {
        let hinstance = GetModuleHandleW(null_mut());

        fn to_w(s: &str) -> Vec<u16> {
            let mut v: Vec<u16> = s.encode_utf16().collect();
            v.push(0);
            v
        }

        let class_name = to_w("SagerLauncherWindowClass");
        let window_title = to_w("Sager - Setup & Launcher");

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: null_mut(),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: CreateSolidBrush(0x000F0F0F),
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
        };

        RegisterClassW(&wc);

        // Center on screen
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        let win_w = 516;
        let win_h = 420;
        let pos_x = (screen_w - win_w) / 2;
        let pos_y = (screen_h - win_h) / 2;

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_title.as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE,
            pos_x,
            pos_y,
            win_w,
            win_h,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut(),
        );

        if hwnd.is_null() {
            return;
        }

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
