use tauri::App;
use tauri::Manager;

use rdev::{simulate, Button, EventType, SimulateError};
use std::{thread, time};

pub fn listen_front_event(app: &mut App) {
    let window = app.get_window("main").unwrap();
    let callback_app_handle = app.app_handle().clone();

    window.listen("blur", move |_event| {
        let win = callback_app_handle.get_window("main").unwrap();
        win.set_ignore_cursor_events(true)
            .expect("error setting ignore cursor events");

        thread::sleep(time::Duration::from_millis(20));

        send(&EventType::ButtonPress(Button::Left));
        send(&EventType::ButtonRelease(Button::Left));

        thread::sleep(time::Duration::from_millis(500));

        win.set_ignore_cursor_events(false)
            .expect("error setting ignore cursor events");
    });
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    thread::sleep(delay); // Let ths OS catchup (at least MacOS)
}
