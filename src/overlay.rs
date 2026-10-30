// Steam In-Game TopMost Overlay Toast Window
// Renders an authentic Steam achievement unlock popup over running games

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};
use std::sync::mpsc::{channel, Sender};
use std::sync::Once;
use std::thread;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

#[derive(Clone, Debug)]
pub struct SteamOverlayData {
    pub game_name: String,
    pub title: String,
    pub description: String,
}

static REGISTER_CLASS_ONCE: Once = Once::new();
static mut OVERLAY_TX: Option<Sender<SteamOverlayData>> = None;

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

pub fn init_steam_overlay() {
    REGISTER_CLASS_ONCE.call_once(|| {
        let (tx, rx) = channel::<SteamOverlayData>();
        unsafe {
            OVERLAY_TX = Some(tx);
        }

        thread::spawn(move || {
            while let Ok(data) = rx.recv() {
                display_overlay_toast(data);
            }
        });
    });
}

pub fn show_steam_overlay(game_name: &str, title: &str, description: &str) {
    init_steam_overlay();
    unsafe {
        if let Some(ref tx) = OVERLAY_TX {
            let _ = tx.send(SteamOverlayData {
                game_name: game_name.to_string(),
                title: title.to_string(),
                description: description.to_string(),
            });
        }
    }
}

fn display_overlay_toast(data: SteamOverlayData) {
    unsafe {
        let class_name = to_wide("SagerSteamAchievementOverlay");
        let hinstance = GetModuleHandleW(null());

        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(overlay_wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: null_mut(),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: null_mut(),
            lpszMenuName: null(),
            lpszClassName: class_name.as_ptr(),
        };

        RegisterClassW(&wnd_class);

        let width = 360;
        let height = 86;
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        let x = screen_w - width - 24;
        let y = screen_h - height - 36;

        let title_wide = to_wide("Steam Achievement");

        // Box data onto heap and pass as lParam
        let data_box = Box::new(data);
        let lparam = Box::into_raw(data_box) as isize;

        let hwnd = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            class_name.as_ptr(),
            title_wide.as_ptr(),
            WS_POPUP,
            x,
            y,
            width,
            height,
            null_mut(),
            null_mut(),
            hinstance,
            lparam as *const std::ffi::c_void,
        );

        if hwnd.is_null() {
            return;
        }

        // Auto close after 4.5 seconds
        SetTimer(hwnd, 1, 4500, None);

        // Show topmost without stealing focus
        ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        UpdateWindow(hwnd);

        let mut msg = std::mem::zeroed::<MSG>();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

unsafe extern "system" fn overlay_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            let cs = lparam as *const CREATESTRUCTW;
            if !cs.is_null() {
                let p_data = (*cs).lpCreateParams as isize;
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, p_data);
            }
            0
        }
        WM_PAINT => {
            let mut ps = std::mem::zeroed::<PAINTSTRUCT>();
            let hdc = BeginPaint(hwnd, &mut ps);

            let p_data = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut SteamOverlayData;
            let (game_name, title, desc) = if !p_data.is_null() {
                let d = &*p_data;
                (d.game_name.as_str(), d.title.as_str(), d.description.as_str())
            } else {
                ("Steam Game", "Achievement Unlocked!", "")
            };

            let mut rect = std::mem::zeroed::<RECT>();
            GetClientRect(hwnd, &mut rect);

            // 1. Draw Steam Charcoal Dark Navy Background (#1b2838)
            let bg_brush = CreateSolidBrush(0x0038281b); // RGB(27, 40, 56) in 0x00BBGGRR
            FillRect(hdc, &rect, bg_brush);
            DeleteObject(bg_brush);

            // 2. Draw Outer Border (#2a475e -> RGB(42, 71, 94) -> 0x005E472A)
            let border_brush = CreateSolidBrush(0x005E472A);
            FrameRect(hdc, &rect, border_brush);
            DeleteObject(border_brush);

            // 3. Top Glowing Steam Cyan Accent Line (#66c0f4 -> RGB(102, 192, 244) -> 0x00F4C066)
            let top_line = RECT {
                left: 1,
                top: 1,
                right: rect.right - 1,
                bottom: 3,
            };
            let accent_brush = CreateSolidBrush(0x00F4C066);
            FillRect(hdc, &top_line, accent_brush);
            DeleteObject(accent_brush);

            // 4. Icon Badge Container on Left (56x56 square, #101822 -> 0x00221810)
            let icon_box = RECT {
                left: 14,
                top: 14,
                right: 70,
                bottom: 70,
            };
            let icon_bg = CreateSolidBrush(0x00221810);
            FillRect(hdc, &icon_box, icon_bg);
            DeleteObject(icon_bg);

            // Draw glowing icon border
            let icon_border = CreateSolidBrush(0x00F4C066);
            FrameRect(hdc, &icon_box, icon_border);
            DeleteObject(icon_border);

            // Icon text / symbol (🏆 or STAR)
            SetBkMode(hdc, 1); // TRANSPARENT
            SetTextColor(hdc, 0x0024BFFB); // Gold / Amber (RGB 251, 191, 36 -> 0x0024BFFB)
            let icon_font = CreateFontW(
                28, 0, 0, 0, FW_BOLD as i32, 0, 0, 0, DEFAULT_CHARSET as u32,
                OUT_DEFAULT_PRECIS as u32, CLIP_DEFAULT_PRECIS as u32, CLEARTYPE_QUALITY as u32,
                (DEFAULT_PITCH | FF_DONTCARE) as u32, to_wide("Segoe UI Emoji").as_ptr(),
            );
            let old_font = SelectObject(hdc, icon_font);
            let mut icon_rect = icon_box;
            let icon_symbol = to_wide("🏆");
            DrawTextW(
                hdc,
                icon_symbol.as_ptr(),
                -1,
                &mut icon_rect,
                DT_CENTER | DT_VCENTER | DT_SINGLELINE,
            );
            SelectObject(hdc, old_font);
            DeleteObject(icon_font);

            // 5. Steam Header Text ("STEAM ACHIEVEMENT UNLOCKED")
            SetTextColor(hdc, 0x00F4C066); // Steam Blue (#66c0f4)
            let header_font = CreateFontW(
                13, 0, 0, 0, FW_BOLD as i32, 0, 0, 0, DEFAULT_CHARSET as u32,
                OUT_DEFAULT_PRECIS as u32, CLIP_DEFAULT_PRECIS as u32, CLEARTYPE_QUALITY as u32,
                (DEFAULT_PITCH | FF_DONTCARE) as u32, to_wide("Segoe UI").as_ptr(),
            );
            let old_font = SelectObject(hdc, header_font);
            let mut header_rect = RECT {
                left: 80,
                top: 13,
                right: rect.right - 12,
                bottom: 28,
            };
            let header_text = to_wide("STEAM ACHIEVEMENT UNLOCKED");
            DrawTextW(
                hdc,
                header_text.as_ptr(),
                -1,
                &mut header_rect,
                DT_LEFT | DT_SINGLELINE | DT_NOPREFIX,
            );
            SelectObject(hdc, old_font);
            DeleteObject(header_font);

            // 6. Achievement Title Text (Bold White)
            SetTextColor(hdc, 0x00FFFFFF); // White
            let title_font = CreateFontW(
                18, 0, 0, 0, FW_BOLD as i32, 0, 0, 0, DEFAULT_CHARSET as u32,
                OUT_DEFAULT_PRECIS as u32, CLIP_DEFAULT_PRECIS as u32, CLEARTYPE_QUALITY as u32,
                (DEFAULT_PITCH | FF_DONTCARE) as u32, to_wide("Segoe UI").as_ptr(),
            );
            let old_font = SelectObject(hdc, title_font);
            let mut title_rect = RECT {
                left: 80,
                top: 31,
                right: rect.right - 12,
                bottom: 53,
            };
            let title_wide = to_wide(title);
            DrawTextW(
                hdc,
                title_wide.as_ptr(),
                -1,
                &mut title_rect,
                DT_LEFT | DT_SINGLELINE | DT_NOPREFIX | DT_END_ELLIPSIS,
            );
            SelectObject(hdc, old_font);
            DeleteObject(title_font);

            // 7. Subtitle Text (Game Title or Description - Steam Gray #8f98a0 -> 0x00A0988F)
            SetTextColor(hdc, 0x00A0988F);
            let sub_font = CreateFontW(
                13, 0, 0, 0, FW_NORMAL as i32, 0, 0, 0, DEFAULT_CHARSET as u32,
                OUT_DEFAULT_PRECIS as u32, CLIP_DEFAULT_PRECIS as u32, CLEARTYPE_QUALITY as u32,
                (DEFAULT_PITCH | FF_DONTCARE) as u32, to_wide("Segoe UI").as_ptr(),
            );
            let old_font = SelectObject(hdc, sub_font);
            let mut sub_rect = RECT {
                left: 80,
                top: 55,
                right: rect.right - 12,
                bottom: 74,
            };
            let sub_text = if !desc.is_empty() {
                format!("{}: {}", game_name, desc)
            } else {
                game_name.to_string()
            };
            let sub_wide = to_wide(&sub_text);
            DrawTextW(
                hdc,
                sub_wide.as_ptr(),
                -1,
                &mut sub_rect,
                DT_LEFT | DT_SINGLELINE | DT_NOPREFIX | DT_END_ELLIPSIS,
            );
            SelectObject(hdc, old_font);
            DeleteObject(sub_font);

            EndPaint(hwnd, &ps);
            0
        }
        WM_TIMER => {
            KillTimer(hwnd, 1);
            DestroyWindow(hwnd);
            0
        }
        WM_DESTROY => {
            let p_data = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut SteamOverlayData;
            if !p_data.is_null() {
                let _ = Box::from_raw(p_data);
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
            }
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
