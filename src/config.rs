use std::{collections::HashMap, fs, process::exit};

use global_hotkey::hotkey::HotKey;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    shortcuts: HashMap<String, String>,
}

impl Config {
    pub fn get_config() -> Config {
        let Some(home) = std::env::home_dir().map(|x| x.join(".config/rskhd")) else {
            eprintln!("Unexpected error, homedir doesn't exist");
            exit(0)
        };

        fs::create_dir_all(&home).expect("Unable to create config directory");

        fs::read_to_string(home.join("config.toml"))
            .ok()
            .and_then(|x| toml::from_str(&x).ok())
            .unwrap_or_default()
    }

    pub fn shortcuts(&self) -> HashMap<HotKey, String> {
        self.shortcuts
            .iter()
            .filter_map(|(name, command)| {
                let hotkey: HotKey = name.parse().ok()?;
                Some((hotkey, command.clone()))
            })
            .collect()
    }

    pub fn shortcuts_with_id(&self) -> HashMap<u32, String> {
        self.shortcuts
            .iter()
            .filter_map(|(name, command)| {
                let hotkey: HotKey = name.parse().ok()?;
                Some((hotkey.id(), command.clone()))
            })
            .collect()
    }
}
