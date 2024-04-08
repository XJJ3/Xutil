// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time};

use tauri::Manager;

mod common;
mod invoke;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // listen to the `event-name` (emitted on any window)
            app.listen_global("click", move |event| {
                println!("got event-name with payload {:?}", event.payload());
                send(&EventType::MouseMove { x: 400.0, y: 400.0 });
                send(&EventType::ButtonPress(Button::Left));
            });
            // let window = app.get_window("main").unwrap();
            // window
            //     .set_ignore_cursor_events(true)
            //     .expect("error setting ignore cursor events");

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

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}
