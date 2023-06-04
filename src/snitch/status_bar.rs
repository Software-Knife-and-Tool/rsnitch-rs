//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(clippy::collapsible_match)]
#![allow(unused_imports)]
use {
    super::ui::Message,
    crate::Environment,
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
pub struct StatusBar {
    host_path: String,
}

impl StatusBar {
    pub fn new(env: &Environment, _style: String) -> Self {
        let host_path = match &env.hosts_path {
            Some(path) => path.to_str().unwrap().to_string(),
            None => "~/.rsnitch-rs/hosts.json".to_string(),
        };

        StatusBar { host_path }
    }

    pub fn view(&self) -> Element<Message> {
        let status_bar = row![
            text("rsnitch-rs: v0.0.2".to_string()).size(20),
            text(self.host_path.clone()).size(20),
        ]
        .spacing(20);

        let content = Column::new()
            .align_items(Alignment::Start)
            .spacing(20)
            .push(status_bar);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
