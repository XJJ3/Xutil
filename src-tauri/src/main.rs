// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

mod common;
mod event;
mod invoke;
mod logger;

fn main() {
    let _ = fix_path_env::fix(); // <---- Add this
    let _ = logger::register_logger();

    for (key, value) in std::env::vars() {
        log::trace!("{}: {}", key, value);
    }

    tauri::Builder::default()
        .setup(|app| {
            event::listen_front_event(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![invoke::dispatch_command])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(focused) => {
                // hide window whenever it loses focus
                let _ = event.window().emit("windowFocused", focused);
            }
            tauri::WindowEvent::Moved(_moved) => {
                // println!("移动");
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
