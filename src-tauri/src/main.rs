// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod common;
mod event;
mod invoke;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            event::listenFrontEvent(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![invoke::dispatch_command])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(focused) => {
                // hide window whenever it loses focus
                // if !focused {
                //     event.window().set_ignore_cursor_events(false).unwrap();
                // }
                // println!("聚焦");
            }
            tauri::WindowEvent::Moved(moved) => {
                // println!("移动");
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
