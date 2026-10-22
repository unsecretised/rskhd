use std::collections::HashMap;

use global_hotkey::{GlobalHotKeyManager, hotkey::HotKey};

use crate::config::Config;

pub struct App {
    manager: global_hotkey::GlobalHotKeyManager,
    shortcuts: HashMap<u32, String>,
    config: Config,
}

impl App {
    pub fn new(config: Config) -> App {
        let shortcuts = config.shortcuts_with_id();
        App {
            manager: GlobalHotKeyManager::new().expect("Unable to create a manager"),
            shortcuts,
            config,
        }
    }

    pub fn register(&self) {
        self.manager
            .register_all(&self.config.shortcuts().into_keys().collect::<Vec<HotKey>>())
            .expect("Unable to register");
    }

    pub fn command_for_event(&self, event: u32) -> Option<&String> {
        self.shortcuts.get(&event)
    }
}
