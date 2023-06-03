//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(clippy::collapsible_match)]

use {
    super::{
        controls::Controls, host::Host, info::InfoBox, statusbar::StatusBar, widgets::quad::quad,
    },
    crate::Environment,
    iced::{
        executor, subscription, theme,
        widget::{container, text, Column},
        window, Alignment, Application, Command, Element, Event, Length, Subscription, Theme,
    },
    iced_aw::Grid,
    std::sync::RwLock,
    time,
};

pub struct Ui {
    controls: Controls,
    env: Environment,
    groups: Vec<String>,
    hosts: Option<Vec<Host>>,
    info_box: InfoBox,
    last: Vec<Event>,
    states: RwLock<Vec<bool>>,
    status_bar: StatusBar,
}

#[derive(Debug, Clone)]
pub enum Message {
    Clear,
    ClockTick(time::OffsetDateTime),
    EventOccurred(Event),
    GroupPress(usize),
    HostPress(usize),
    Poll,
}

impl Ui {
    const POLL_INTERVAL: u64 = 10; // sntop uses 180 seconds by default
    const BUTTON_COLS: usize = 4;

    pub fn effective_hid(&self, id: usize, name: &String) -> usize {
        let hosts = self.hosts.as_ref().unwrap();
        let filter = self.controls.get_filter();

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

        let status_bar = StatusBar::new(&env, String::new());

        let ui = Ui {
            controls: Controls::new(),
            env,
            groups,
            hosts,
            info_box: InfoBox::new(6, 80),
            last: Vec::<Event>::new(),
            states: RwLock::new(states),
            status_bar,
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
                self.controls.set_filter(self.groups[id].clone());
            }
            Message::Clear => {
                self.controls.set_filter(String::new());
            }
            Message::HostPress(id) => {
                let host = &self.hosts.as_ref().unwrap()[id];

                self.info_box.clear();
                self.info_box.write(format!("host: {}", host.host));
                self.info_box.scroll();
                self.info_box.write(format!("group: {}", host.group));
                self.info_box.scroll();
                self.info_box.write(format!("label: {}", host.label));
                self.info_box.scroll();
                self.info_box.write(format!("info: {}", Host::info(host)));
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
        let hosts = self.hosts.as_ref().unwrap();
        let states = self.states.read().unwrap();
        let groups = &self.groups;
        let filter = self.controls.get_filter();

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

        let content = Column::new()
            .align_items(Alignment::Start)
            .spacing(20)
            .push(group_grid)
            .push(quad(500, 1))
            .push(self.info_box.view())
            .push(button_grid)
            .push(quad(500, 1))
            .push(self.controls.view())
            .push(quad(500, 1))
            .push(self.status_bar.view());

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}
