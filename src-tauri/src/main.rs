// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod invoke_command;

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![invoke_command::dispatch_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
