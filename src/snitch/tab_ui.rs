// SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
// SPDX-License-Identifier: MIT

// rsnitch tab ui
//
#![allow(clippy::new_without_default)]
#![allow(dead_code)]
use {
    super::status_bar::StatusBar,
    super::tabs::hosts_tab::{HostsMessage, HostsTab},
    super::tabs::settings_tab::{SettingsMessage, SettingsTab, TabBarPosition},
    crate::Environment,
    iced::{
        alignment::{Horizontal, Vertical},
        executor, subscription,
        widget::{Column, Container, Text},
        window, Application, Command, Element, Event, Length, Subscription, Theme,
    },
    iced_aw::{TabLabel, Tabs},
};

pub struct TabUi {
    active_tab: usize,
    hosts_tab: HostsTab,
    // controls_tab: ControlsTab,
    settings_tab: SettingsTab,
    status_bar: StatusBar,
    poll_interval_secs: u64,
}

#[derive(Clone, Debug)]
pub enum Message {
    Hosts(HostsMessage),
    // Controls(ControlsMessage),
    Settings(SettingsMessage),
    TabSelected(usize),
    EventOccurred(Event),
    ClockTick(time::OffsetDateTime),
    Tab(usize),
}

impl TabUi {
    const POLL_INTERVAL: u64 = 10; // sntop uses 180 seconds by default
}

impl Application for TabUi {
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Environment;
    type Message = Message;

    fn new(env: Environment) -> (TabUi, Command<Message>) {
        let tab_bar = TabUi {
            active_tab: 0,
            hosts_tab: HostsTab::new(&env),
            // controls_tab: ControlsTab::new(&env),
            settings_tab: SettingsTab::new(&env),
            status_bar: StatusBar::new(&env, "hello martin".to_string()),
            poll_interval_secs: Self::POLL_INTERVAL,
        };

        (tab_bar, Command::none())
    }

    fn title(&self) -> String {
        String::from("TabUi")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::TabSelected(selected) => {
                self.active_tab = selected;
            }
            Message::Hosts(message) => self.hosts_tab.update(message),
            // Message::Controls(message) => self.controls_tab.update(message),
            // Message::Settings(message) => self.settings_tab.update(message),
            Message::ClockTick(t) => {
                self.hosts_tab.update(HostsMessage::ClockTick(t));
                //  self.controls_tab.update(ControlsMessage::ClockTick(t));
                //  self.settings_tab.update(SettingsMessage::ClockTick(t));
            }
            Message::EventOccurred(event)
                if event == Event::Window(window::Event::CloseRequested) =>
            {
                let _ = window::close::<Message>();
            }
            Message::EventOccurred(_) => (),
            _ => (),
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let _ = subscription::events().map(Message::EventOccurred);

        iced::time::every(std::time::Duration::from_millis(
            self.poll_interval_secs * 1000,
        ))
        .map(|_| {
            Message::ClockTick(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            )
        })
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let position = self
            .settings_tab
            .settings()
            .tab_bar_position
            .unwrap_or_default();
        let theme = self
            .settings_tab
            .settings()
            .tab_bar_theme
            .unwrap_or_default();

        Tabs::new(self.active_tab, Message::TabSelected)
            .push(self.hosts_tab.tab_label(), self.hosts_tab.content())
            // .push(self.controls_tab.tab_label(), self.controls_tab.view())
            .push(self.settings_tab.tab_label(), self.settings_tab.content())
            .tab_bar_style(theme)
            .tab_bar_position(match position {
                TabBarPosition::Top => iced_aw::TabBarPosition::Top,
                TabBarPosition::Bottom => iced_aw::TabBarPosition::Bottom,
            })
            .into()
    }
}

pub trait Tab {
    const HEADER_SIZE: u16 = 32;
    const TAB_PADDING: u16 = 16;

    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(Self::HEADER_SIZE))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(Self::TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
