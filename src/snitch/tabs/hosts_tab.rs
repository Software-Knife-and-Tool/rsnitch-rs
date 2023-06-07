// SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
// SPDX-License-Identifier: MIT

// hosts tab
//
#![allow(clippy::collapsible_match)]
#![allow(clippy::new_without_default)]
#![allow(dead_code)]
use {
    super::{
        super::tab_ui::{Message, Tab},
        hosts::{
            group_box::GroupBox,
            host::{Host, Poll},
            host_box::HostBox,
            info_box::InfoBox,
        },
    },
    crate::Environment,
    iced::{
        alignment::{Horizontal, Vertical},
        widget::{container, horizontal_rule, Column, Container, Row},
        Alignment, Element, Event, Length,
    },
    iced_aw::tab_bar::TabLabel,
    std::sync::RwLock,
    time,
};

pub struct HostsTab {
    group_box: GroupBox,
    groups: Vec<String>,
    poll: Poll,
    hosts: Option<Vec<Host>>,
    host_box: HostBox,
    info_box: InfoBox,
    last: Vec<Event>,
    states: RwLock<Vec<bool>>,
}

#[derive(Clone, Debug)]
pub enum HostsMessage {
    Clear,
    ClockTick(time::OffsetDateTime),
    GroupPress(usize),
    HostPress(usize),
    Poll,
}

impl HostsTab {
    pub fn new(env: &Environment) -> Self {
        let hosts = Host::load(env);
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
        let group_box = GroupBox::new(env, 5);

        let host_box = HostBox::new(env, 5);
        let info_box = InfoBox::new(env, 6, 80);
        let last = Vec::<Event>::new();
        let poll = Poll::new(env);

        let states = RwLock::new(match &hosts {
            Some(hosts) => poll.poll_all(hosts),
            None => Vec::new(),
        });

        HostsTab {
            group_box,
            groups,
            hosts,
            host_box,
            info_box,
            poll,
            last,
            states,
        }
    }

    pub fn update(&mut self, message: HostsMessage) {
        match message {
            HostsMessage::Poll => match &self.hosts {
                Some(hosts) => {
                    let mut states = self.states.write().unwrap();

                    *states = self.poll.poll_all(hosts);
                }
                None => (),
            },
            HostsMessage::GroupPress(_id) => {
                // self.controls.set_filter(self.groups[id].clone());
            }
            HostsMessage::Clear => {
                // self.controls.set_filter(String::new());
            }
            HostsMessage::HostPress(id) => {
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
            HostsMessage::ClockTick(_t) => match &self.hosts {
                Some(hosts) => {
                    let mut states = self.states.write().unwrap();

                    *states = self.poll.poll_all(hosts);
                }
                None => (),
            },
        }
    }

    pub fn view(&self) -> Element<'_, HostsMessage> {
        let hosts = self.hosts.as_ref().unwrap();
        let states = self.states.read().unwrap();
        // let filter = self.controls.get_filter();

        let button_col = Column::new()
            .align_items(Alignment::Start)
            .push(self.group_box.view(&self.groups))
            .push(horizontal_rule(1))
            .push(self.host_box.view("".to_string(), hosts, &states));

        let info_col = Column::new()
            .align_items(Alignment::Start)
            .push(self.info_box.view());

        let content = Row::new()
            .align_items(Alignment::Start)
            .spacing(5)
            .push(info_col.width(400))
            .push(button_col);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}

impl Tab for HostsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("rSnitch::HostTab")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("hosts".to_string())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        // let _states = self.states.read().unwrap();
        let content: Element<'_, HostsMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Start)
                .max_width(800)
                .padding(20)
                .spacing(10)
                .push(self.view()),
        )
        .align_x(Horizontal::Left)
        .align_y(Vertical::Top)
        .into();

        content.map(Message::Hosts)
    }
}
