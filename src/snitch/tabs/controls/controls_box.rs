//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(clippy::collapsible_match)]
#![allow(unused_imports)]
use {
    super::super::super::tab_ui::Message,
    super::super::hosts_tab::HostsMessage,
    iced::{
        executor,
        keyboard::Event::CharacterReceived,
        subscription, theme,
        widget::{button, column, container, row, rule, text, Column, Space, Text},
        window, Alignment, Application, Color, Command, Element, Event, Length, Subscription,
        Theme,
    },
    std::sync::RwLock,
};

#[derive(Debug, Default)]
pub struct Controls {
    filter: RwLock<String>,
}

impl Controls {
    pub fn new() -> Self {
        Controls {
            filter: RwLock::new(String::new()),
        }
    }

    pub fn set_filter(&self, value: String) {
        let mut filter = self.filter.write().unwrap();

        *filter = value;
    }

    pub fn get_filter(&self) -> String {
        let filter = self.filter.read().unwrap();

        (*filter).clone()
    }

    pub fn view(&self) -> Element<Message> {
        let filter = self.filter.read().unwrap();

        let control_panel = column![
            text("rsnitch-rs: v0.0.2".to_string()).size(28),
            text(format!("filter: {}", filter)).size(24),
            row![
                iced::widget::button(text("clear filter")).on_press(HostsMessage::Clear),
                iced::widget::button(text("poll")).on_press(HostsMessage::Poll),
            ]
            .align_items(Alignment::End),
        ];

        let content = Column::new()
            .align_items(Alignment::Start)
            .spacing(20)
            .push(
                control_panel
                    .width(500)
                    .spacing(10)
                    .align_items(Alignment::Start),
            );

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}
