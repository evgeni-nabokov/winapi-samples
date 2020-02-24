#![windows_subsystem = "windows"]

use std::ptr::null_mut;
use winapi::um::winuser::{MB_OK, MessageBoxW};
use widestring::U16CString;

fn main() {
    let w_caption = U16CString::from_str("Note").unwrap();
    let w_msg = U16CString::from_str("Goodbye, cruel world!").unwrap();
    unsafe {
        MessageBoxW(null_mut(), w_msg.as_ptr(), w_caption.as_ptr(), MB_OK)
    };
}