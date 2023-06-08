//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]

mod settings;
mod snitch;

use {
    iced::{window, Application, Settings},
    serde::{Deserialize, Serialize},
    snitch::snitch_ui::SnitchUi,
};

#[derive(Default, Serialize, Deserialize)]
pub struct Environment {
    user: String,
    hostname: String,
    home_path: std::path::PathBuf,
    config_path: std::path::PathBuf,
    hosts_path: Option<std::path::PathBuf>,
    settings: Option<settings::Settings>,
}

impl Environment {
    const CONFIG_PATH: &str = ".config/rsnitch-rs";
    const SETTINGS_FILE: &str = "settings.json";

    fn dotfiles(self) -> Self {
        let config_path = self.config_path.as_path();

        if !config_path.exists() {
            std::fs::create_dir(config_path).unwrap();
        }

        let settings = settings::Settings::from_env(&self);

        Environment {
            user: self.user,
            hostname: self.hostname,
            home_path: self.home_path,
            config_path: self.config_path,
            hosts_path: self.hosts_path,
            settings,
        }
    }
}

pub fn main() -> iced::Result {
    let hosts_path = &envmnt::get_or("RSNITCH_HOSTS", "");
    let home = &envmnt::get_or("HOME", "");
    let home_path = std::path::Path::new(home);
    let config_path = std::path::Path::new(Environment::CONFIG_PATH);

    let env = Environment {
        user: whoami::username(),
        hostname: whoami::hostname(),
        home_path: home_path.to_path_buf(),
        config_path: std::path::Path::join(home_path, config_path),
        hosts_path: if hosts_path.is_empty() {
            None
        } else {
            Some(std::path::Path::new(hosts_path).to_path_buf())
        },
        settings: None,
    }
    .dotfiles();

    sudo::with_env(&["HOME", "USER", "RSNITCH_HOSTS", "XDG_RUNTIME_DIR"]).expect("sudo failed");

    SnitchUi::run(Settings {
        exit_on_close_request: true,
        flags: env,
        window: window::Settings {
            size: (1000, 500),
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        // default_font: Some(include_bytes!("path-to-font ttf")),
        antialiasing: true,
        ..Default::default()
    })
}
