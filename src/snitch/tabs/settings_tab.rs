// SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
// SPDX-License-Identifier: MIT

// settings panel
//
#![allow(clippy::new_without_default)]
use {
    super::super::tab_ui::{Message, Tab},
    crate::Environment,
    iced::{
        alignment::{Horizontal, Vertical},
        widget::{Column, Container, Radio, Text},
        Element,
    },
    iced_aw::{style::TabBarStyles, tab_bar::TabLabel},
    time,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabBarPosition {
    #[default]
    Top,
    Bottom,
}

impl TabBarPosition {
    pub const ALL: [TabBarPosition; 2] = [TabBarPosition::Top, TabBarPosition::Bottom];
}

impl From<TabBarPosition> for String {
    fn from(position: TabBarPosition) -> Self {
        String::from(match position {
            TabBarPosition::Top => "Top",
            TabBarPosition::Bottom => "Bottom",
        })
    }
}

//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabSettings {
    pub tab_bar_position: Option<TabBarPosition>,
    pub tab_bar_theme: Option<TabBarStyles>,
}

impl TabSettings {
    pub fn new() -> Self {
        TabSettings {
            tab_bar_position: Some(TabBarPosition::Top),
            tab_bar_theme: Some(TabBarStyles::default()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    PositionSelected(TabBarPosition),
    ThemeSelected(TabBarStyles),
    ClockTick(time::OffsetDateTime),
}

pub struct SettingsTab {
    settings: TabSettings,
}

impl SettingsTab {
    pub fn new(_env: &Environment) -> Self {
        SettingsTab {
            settings: TabSettings::new(),
        }
    }

    pub fn settings(&self) -> &TabSettings {
        &self.settings
    }

    pub fn update(&mut self, message: SettingsMessage) {
        match message {
            SettingsMessage::PositionSelected(position) => {
                self.settings.tab_bar_position = Some(position)
            }
            SettingsMessage::ThemeSelected(theme) => self.settings.tab_bar_theme = Some(theme),
            SettingsMessage::ClockTick(_t) => (),
        }
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Ariel::Settings")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("settings".to_string())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, SettingsMessage> = Container::new(
            Column::new()
                .push(Text::new("TabBar position:").size(20))
                .push(TabBarPosition::ALL.iter().cloned().fold(
                    Column::new().padding(10).spacing(10),
                    |column, position| {
                        column.push(
                            Radio::new(
                                position,
                                position,
                                self.settings().tab_bar_position,
                                SettingsMessage::PositionSelected,
                            )
                            .size(16),
                        )
                    },
                ))
                .push(Text::new("TabBar color:").size(20))
                .push(
                    (0..5).fold(Column::new().padding(10).spacing(10), |column, id| {
                        column.push(
                            Radio::new(
                                predefined_style(id),
                                predefined_style(id),
                                self.settings().tab_bar_theme,
                                SettingsMessage::ThemeSelected,
                            )
                            .size(16),
                        )
                    }),
                ),
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Top)
        .into();

        content.map(Message::Settings)
    }
}

fn predefined_style(index: usize) -> TabBarStyles {
    match index {
        0 => TabBarStyles::Default,
        1 => TabBarStyles::Red,
        2 => TabBarStyles::Blue,
        3 => TabBarStyles::Green,
        4 => TabBarStyles::Purple,
        _ => TabBarStyles::Default,
    }
}