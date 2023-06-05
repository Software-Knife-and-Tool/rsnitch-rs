//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(clippy::collapsible_match)]

use {
    super::{
        controls_box::Controls,
        group_box::GroupBox,
        host::{Host, Poll},
        host_box::HostBox,
        info_box::InfoBox,
        status_bar::StatusBar,
        widgets::quad::quad,
    },
    crate::Environment,
    iced::{
        executor, subscription,
        widget::{container, Column},
        window, Alignment, Application, Command, Element, Event, Length, Subscription, Theme,
    },
    std::sync::RwLock,
    time,
};

pub struct Ui {
    controls: Controls,
    env: Environment,
    group_box: GroupBox,
    groups: Vec<String>,
    poll: Poll,
    hosts: Option<Vec<Host>>,
    host_box: HostBox,
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
}

impl Application for Ui {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Environment;

    fn new(env: Environment) -> (Ui, Command<Message>) {
        let hosts = Host::load(&env);
        let mut groups: Vec<String> = Vec::new();

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
        let group_box = GroupBox::new(&env, 5);
        let host_box = HostBox::new(&env, 5);
        let info_box = InfoBox::new(&env, 6, 80);
        let poll = Poll::new(&env);
        let controls = Controls::new();
        let last = Vec::<Event>::new();

        let states = RwLock::new(match &hosts {
            Some(hosts) => poll.poll_all(hosts),
            None => Vec::new(),
        });

        let ui = Ui {
            controls,
            env,
            group_box,
            groups,
            hosts,
            host_box,
            info_box,
            poll,
            last,
            states,
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

                    *states = self.poll.poll_all(hosts);
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
                self.info_box
                    .write(format!("info: {}", Host::info(&self.poll, host)));
            }
            Message::ClockTick(_t) => match &self.hosts {
                Some(hosts) => {
                    let mut states = self.states.write().unwrap();

                    *states = self.poll.poll_all(hosts);
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
        let filter = self.controls.get_filter();

        let content = Column::new()
            .align_items(Alignment::Start)
            .spacing(20)
            .push(self.group_box.view(&self.groups))
            .push(quad(500, 1))
            .push(self.info_box.view())
            .push(self.host_box.view(filter, hosts, &states))
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
