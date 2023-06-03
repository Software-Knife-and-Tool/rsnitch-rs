//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]

mod settings;
mod snitch;

use {
    iced::{Application, Settings},
    serde::{Deserialize, Serialize},
    snitch::ui::Ui,
};

#[derive(Default, Serialize, Deserialize)]
pub struct Environment {
    user: String,
    hostname: String,
    home_path: std::path::PathBuf,
    dot_path: std::path::PathBuf,
    hosts_path: Option<std::path::PathBuf>,
    settings: Option<settings::Settings>,
}

impl Environment {
    const DOT_PATH: &str = ".rsnitch-rs";
    const SETTINGS_FILE: &str = "settings.json";

    fn dotfiles(self) -> Self {
        let dot_path = self.dot_path.as_path();

        if !dot_path.exists() {
            std::fs::create_dir(dot_path).unwrap();
        }

        let settings = settings::Settings::from_env(&self);

        Environment {
            user: self.user,
            hostname: self.hostname,
            home_path: self.home_path,
            dot_path: self.dot_path,
            hosts_path: self.hosts_path,
            settings,
        }
    }
}

pub fn main() -> iced::Result {
    let hosts_path = &envmnt::get_or("RSNITCH_HOSTS", "");
    let home = &envmnt::get_or("HOME", "");
    let home_path = std::path::Path::new(home);
    let dot_path = std::path::Path::new(Environment::DOT_PATH);

    let env = Environment {
        user: whoami::username(),
        hostname: whoami::hostname(),
        home_path: home_path.to_path_buf(),
        dot_path: std::path::Path::join(home_path, dot_path),
        hosts_path: if hosts_path.is_empty() {
            None
        } else {
            Some(std::path::Path::new(hosts_path).to_path_buf())
        },
        settings: None,
    }
    .dotfiles();

    /*
    let settings = Settings {
        window: window::Settings {
            size: (800, 600),
            resizeable: true,
            decorations: true,
        },
        // default_font: Some(include_bytes!("path-to-font ttf")),
        antialiasing: true,
    }
     */

    sudo::with_env(&["HOME", "USER", "RSNITCH_HOSTS", "XDG_RUNTIME_DIR"]).expect("sudo failed");

    Ui::run(Settings {
        exit_on_close_request: false,
        flags: env,
        ..Settings::default()
    })
}