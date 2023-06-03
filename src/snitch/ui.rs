//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(clippy::collapsible_match)]
#![allow(unused_imports)]
use {
    crate::{
        snitch::{host::Host, text_widget::TextWidget, widgets::quad::quad},
        Environment,
    },
    iced::{
        executor,
        keyboard::Event::CharacterReceived,
        subscription, theme,
        widget::{button, column, container, row, rule, text, Column, Space, Text},
        window, Alignment, Application, Color, Command, Element, Event, Length, Subscription,
        Theme,
    },
    iced_aw::{grid, Grid},
    serde::{Deserialize, Serialize},
    serde_json::{Result as SerdeResult, Value},
    time::{self, OffsetDateTime},
};

use std::sync::RwLock;

pub struct Ui {
    env: Environment,
    hosts: Option<Vec<Host>>,
    groups: Vec<String>,
    filter: RwLock<String>,
    states: RwLock<Vec<bool>>,
    last: Vec<Event>,
    text_widget: TextWidget,
}

#[derive(Debug, Clone)]
pub enum Message {
    HostPress(usize),
    GroupPress(usize),
    Clear,
    EventOccurred(Event),
    ClockTick(time::OffsetDateTime),
    Poll,
}

impl Ui {
    const POLL_INTERVAL: u64 = 10; // sntop uses 180 seconds by default
    const BUTTON_COLS: usize = 4;

    pub fn effective_hid(&self, id: usize, name: &String) -> usize {
        let hosts = self.hosts.as_ref().unwrap();
        let filter = self.filter.read().unwrap();

        if filter.is_empty() {
            id
        } else {
            hosts.iter().position(|host| &host.host == name).unwrap()
        }
    }
}

impl Application for Ui {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Environment;

    fn new(env: Environment) -> (Ui, Command<Message>) {
        let hosts = Host::load(&env);
        let mut groups: Vec<String> = Vec::new();

        let states = match &hosts {
            Some(hosts) => hosts.iter().map(Host::is_up).collect(),
            None => Vec::new(),
        };

        match &hosts {
            Some(hosts) => {
                for host in hosts {
                    match groups.iter().find(|group| group == &&host.group) {
                        Some(_) => (),
                        None => groups.push(host.group.clone()),
                    }
                }
            }
            None => (),
        }

        let ui = Ui {
            last: Vec::<Event>::new(),
            text_widget: TextWidget::new(6, 80),
            filter: RwLock::new(String::new()),
            hosts,
            groups,
            states: RwLock::new(states),
            env,
        };

        (ui, Command::none())
    }

    fn title(&self) -> String {
        String::from("Rsnitch-rs - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Poll => match &self.hosts {
                Some(hosts) => {
                    let mut states = self.states.write().unwrap();

                    *states = hosts.iter().map(Host::is_up).collect();
                }
                None => (),
            },
            Message::GroupPress(id) => {
                let mut filter = self.filter.write().unwrap();

                *filter = self.groups[id].clone();
            }
            Message::Clear => {
                let mut filter = self.filter.write().unwrap();

                *filter = String::new();
            }
            Message::HostPress(id) => {
                let host = &self.hosts.as_ref().unwrap()[id];

                self.text_widget.clear();
                self.text_widget
                    .write_string(format!("host: {}", host.host));
                self.text_widget.scroll();
                self.text_widget
                    .write_string(format!("group: {}", host.group));
                self.text_widget.scroll();
                self.text_widget
                    .write_string(format!("label: {}", host.label));
                self.text_widget.scroll();
                self.text_widget
                    .write_string(format!("info: {}", Host::info(host)));
            }
            Message::ClockTick(_t) => match &self.hosts {
                Some(hosts) => {
                    let mut states = self.states.write().unwrap();

                    *states = hosts.iter().map(Host::is_up).collect();
                }
                None => (),
            },
            Message::EventOccurred(event)
                if event == Event::Window(window::Event::CloseRequested) =>
            {
                let _ = window::close::<Message>();
            }
            Message::EventOccurred(_) => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let _ = subscription::events().map(Message::EventOccurred);

        iced::time::every(std::time::Duration::from_millis(Self::POLL_INTERVAL * 1000)).map(|_| {
            Message::ClockTick(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            )
        })
    }

    fn view(&self) -> Element<Message> {
        let txt = Column::new().push(text(self.text_widget.contents()));
        let hosts = self.hosts.as_ref().unwrap();
        let states = self.states.read().unwrap();
        let groups = &self.groups;
        let filter = self.filter.read().unwrap();

        let grid_spacer = "                                 ";

        let mut group_grid = Grid::with_columns(Self::BUTTON_COLS);
        for (id, group) in groups.iter().enumerate() {
            if id % Self::BUTTON_COLS == 0 {
                for _ in 0..Self::BUTTON_COLS {
                    group_grid.insert(text(grid_spacer));
                }
            }

            group_grid.insert(
                iced::widget::button(text(group))
                    .style(theme::Button::Primary)
                    .on_press(Message::GroupPress(id)),
            );
        }

        let mut button_grid = Grid::with_columns(Self::BUTTON_COLS);
        for (id, host) in hosts
            .iter()
            .filter(|host| {
                if filter.is_empty() {
                    true
                } else {
                    host.group == *filter
                }
            })
            .enumerate()
        {
            if id % Self::BUTTON_COLS == 0 {
                for _ in 0..Self::BUTTON_COLS {
                    button_grid.insert(text(grid_spacer));
                }
            }

            button_grid.insert(
                iced::widget::button(text(&host.label))
                    .style(if states[self.effective_hid(id, &host.host)] {
                        theme::Button::Primary
                    } else {
                        theme::Button::Secondary
                    })
                    .on_press(Message::HostPress(self.effective_hid(id, &host.host))),
            );
        }

        let control_panel = row![
            iced::widget::button(text("clear filter")).on_press(Message::Clear),
            iced::widget::button(text("poll")).on_press(Message::Poll),
        ];

        let content = Column::new()
            .align_items(Alignment::Start)
            .spacing(20)
            .push(text("rsnitch-rs: v0.0.1".to_string()).size(28))
            .push(quad(500, 1))
            .push(group_grid)
            .push(txt.height(120))
            .push(button_grid)
            .push(quad(500, 1))
            .push(text(format!("filter: {}", filter)))
            .push(control_panel.width(500).align_items(Alignment::End));

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}
