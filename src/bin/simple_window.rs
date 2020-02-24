// A WinAPI program that shows a simple blank window.
// Inspired by following posts:
// https://www.codeproject.com/Tips/1053658/Win-GUI-Programming-In-Rust-Language
// https://github.com/retep998/winapi-rs/issues/122
// https://users.rust-lang.org/t/how-to-set-my-custom-entry-point/15968

#![windows_subsystem = "windows"]

use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, HINSTANCE, LRESULT, UINT, LPARAM, WPARAM};
use winapi::shared::windef::{HWND, HMENU, HBRUSH, POINT};
use winapi::um::winnt::{LPCWSTR};
use winapi::um::winuser::{DefWindowProcW, ShowWindow, CreateWindowExW, RegisterClassW,
                          GetMessageW, TranslateMessage, DispatchMessageW, PostQuitMessage,
                          LoadIconW, LoadCursorW,
                          MSG, WNDCLASSW, IDI_APPLICATION,
                          WS_OVERLAPPEDWINDOW, WS_VISIBLE, SW_SHOWNORMAL, WM_QUIT, WM_DESTROY};
use widestring::U16CString;

const WND_CLASS_NAME: &str = "SimpleWindowClass";
const WND_NAME: &str = "Simple Window";

pub unsafe extern "system"
fn wnd_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if msg == WM_DESTROY {
        PostQuitMessage(0);
    }
    DefWindowProcW(h_wnd, msg, w_param, l_param)
}

fn main()
{
    let w_wnd_class_name = U16CString::from_str(WND_CLASS_NAME).unwrap();
    let w_wnd_name = U16CString::from_str(WND_NAME).unwrap();
    unsafe {
        let wnd_class = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as HINSTANCE,
            hIcon: LoadIconW(0 as HINSTANCE, IDI_APPLICATION),
            hCursor: LoadCursorW(0 as HINSTANCE, IDI_APPLICATION),
            hbrBackground: 16 as HBRUSH,
            lpszMenuName: 0 as LPCWSTR,
            lpszClassName: w_wnd_class_name.as_ptr(),
        };

        RegisterClassW(&wnd_class);
        let h_wnd_window = CreateWindowExW(0,
                                           w_wnd_class_name.as_ptr(),
                                           w_wnd_name.as_ptr(),
                                           WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                                           0,
                                           0,
                                           400,
                                           400,
                                           0 as HWND,
                                           0 as HMENU,
                                           0 as HINSTANCE,
                                           null_mut());
        let mut msg = MSG {
            hwnd : 0 as HWND,
            message : 0 as UINT,
            wParam : 0 as WPARAM,
            lParam : 0 as LPARAM,
            time : 0 as DWORD,
            pt : POINT { x: 0, y: 0, },
        };

        ShowWindow(h_wnd_window, SW_SHOWNORMAL);

        loop
        {
            let pm = GetMessageW(&mut msg, 0 as HWND, 0, 0);
            if pm == 0 {
                break;
            }

            if msg.message == WM_QUIT {
                break;
            }

            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
    }
}