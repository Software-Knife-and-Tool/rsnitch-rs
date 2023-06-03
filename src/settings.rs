//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
#![allow(unused_imports)]
use {
    crate::{snitch, Environment},
    serde::{Deserialize, Serialize},
    //     textui::text_ui::TextUi as TextUi_,
};

#[derive(Default, Serialize, Deserialize)]
pub struct Settings {
    window: Window,
    textui: TextUi,
}

#[derive(Default, Serialize, Deserialize)]
struct Window {
    size: Option<(u32, u32)>,
    min_size: Option<(u32, u32)>,
    max_size: Option<(u32, u32)>,
    resizable: Option<bool>,
}

#[derive(Default, Serialize, Deserialize)]
struct TextUi {
    rows: Option<usize>,
    cursor: Option<usize>,
}

impl Settings {
    const DEFAULT: Settings = Settings {
        window: Window {
            size: None,
            min_size: None,
            max_size: None,
            resizable: None,
        },
        textui: TextUi {
            rows: None,
            cursor: None,
        },
    };

    pub fn from_env(env: &Environment) -> Option<Self> {
        let dot_path = env.dot_path.as_path();

        if dot_path.exists() {
            // let settings = std::path::Path::new(Environment::SETTINGS_FILE);
            // let _settings_path = std::path::Path::join(dot_path, settings);
            // let _serialized = serde_json::to_string(&Settings {}).unwrap();

            Some(Settings::default())
        } else {
            None
        }
    }
}
