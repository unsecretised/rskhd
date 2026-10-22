use std::thread;

use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use objc2::MainThreadMarker;
use objc2_app_kit::NSApp;

use crate::{app::App, config::Config};

mod app;
mod config;

fn main() {
    let config = Config::get_config();

    let app = App::new(config);
    app.register();

    let receiver = GlobalHotKeyEvent::receiver();

    thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            if let Some(command) = app.command_for_event(event.id())
                && event.state == HotKeyState::Pressed
            {
                run_command(command);
            }
        }
    });

    NSApp(MainThreadMarker::new().unwrap()).run();
}

fn run_command(command: &str) {
    std::process::Command::new("zsh")
        .args(["-c", &command])
        .spawn()
        .ok();
}
