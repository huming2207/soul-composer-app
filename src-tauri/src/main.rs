#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod device;
mod prog;

use crate::device::serial_detect::detect_device;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![detect_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
