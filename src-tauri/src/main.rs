// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod common;
mod invoke;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![invoke::dispatch_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
