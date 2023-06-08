// SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
// SPDX-License-Identifier: MIT

// rsnitch tab ui
//
#![allow(clippy::new_without_default)]
#![allow(dead_code)]
#![allow(unused_imports)]
use {
    super::host::{Host, Poll},
    crate::Environment,
    iced::{
        alignment::{Horizontal, Vertical},
        executor, subscription, theme,
        widget::{container, horizontal_rule, row, text, Column, Container, Row, Text},
        window, Alignment, Application, Command, Element, Event, Length, Renderer, Subscription,
        Theme,
    },
    iced_aw::{grid, Grid},
    std::sync::RwLock,
};

// components
#[derive(Debug, Default)]
pub struct InfoBox {
    image: RwLock<String>,
    lines: RwLock<Vec<String>>,
    rows: usize,
    cols: usize,
}

impl InfoBox {
    pub fn new(_nv: &Environment, rows: usize, cols: usize) -> Self {
        InfoBox {
            image: RwLock::new(String::new()),
            lines: RwLock::new(vec![String::new(); rows]),
            rows,
            cols,
        }
    }

    fn collapse(&self) {
        let mut image = self.image.write().unwrap();
        let lines = self.lines.read().unwrap();

        let mut img = String::new();

        for line in &lines[0..self.rows - 1] {
            if line.is_empty() {
                img.push_str(" \n");
            } else {
                img.push_str(line);
                img.push('\n')
            }
        }

        if lines[self.rows - 1].is_empty() {
            img.push(' ');
        } else {
            img.push_str(&lines[self.rows - 1]);
        }

        *image = img
    }

    pub fn clear(&self) {
        {
            let mut lines = self.lines.write().unwrap();

            *lines = vec![String::new(); self.rows]
        }

        self.collapse()
    }

    pub fn scroll(&self) {
        {
            let mut lines = self.lines.write().unwrap();

            lines.remove(0);
            lines.push(String::new())
        }

        self.collapse()
    }

    pub fn backspace(&self) {
        {
            let mut lines = self.lines.write().unwrap();

            if !lines[self.rows - 1].is_empty() {
                lines[self.rows - 1].pop().unwrap();
            }
        }

        self.collapse()
    }

    pub fn write_char(&self, ch: char) {
        {
            let mut lines = self.lines.write().unwrap();

            lines[self.rows - 1].push(ch)
        }

        self.collapse()
    }

    pub fn write(&self, str: String) {
        {
            let mut lines = self.lines.write().unwrap();

            lines[self.rows - 1].push_str(&str)
        }

        self.collapse()
    }

    pub fn contents(&self) -> String {
        let image = self.image.read().unwrap();

        image.clone()
    }

    pub fn view(&self) -> Element<'_, Message, Renderer> {
        Column::new().push(text(self.contents())).into()
    }
}

#[derive(Debug, Default)]
pub struct GroupBox {
    cols: usize,
}

impl GroupBox {
    pub fn new(_env: &Environment, cols: usize) -> Self {
        GroupBox { cols }
    }

    pub fn view(&self, groups: &[String]) -> iced_native::Element<'_, Message, Renderer> {
        let grid_spacer = "                                 ";

        let mut group_grid = Grid::with_columns(self.cols);

        for (id, group) in groups.iter().enumerate() {
            if id % self.cols == 0 {
                for _ in 0..self.cols {
                    group_grid.insert(text(grid_spacer));
                }
            }

            group_grid.insert(
                iced::widget::button(text(group))
                    .height(30)
                    .style(theme::Button::Primary)
                    .on_press(Message::GroupPress(id)),
            );
        }

        let content = Column::new()
            .align_items(Alignment::Start)
            .spacing(20)
            .push(group_grid);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Debug, Default)]
pub struct HostBox {
    cols: usize,
}

impl HostBox {
    pub fn new(_env: &Environment, cols: usize) -> Self {
        HostBox { cols }
    }

    pub fn effective_hid(
        &self,
        hosts: &[Host],
        id: usize,
        name: &String,
        filter: &String,
    ) -> usize {
        if filter.is_empty() {
            id
        } else {
            hosts.iter().position(|host| &host.host == name).unwrap()
        }
    }

    pub fn view(
        &self,
        filter: String,
        hosts: &[Host],
        states: &[bool],
    ) -> Element<'_, Message, Renderer> {
        let grid_spacer = "                                 ";

        let mut host_grid = Grid::with_columns(self.cols);
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
            if id % self.cols == 0 {
                for _ in 0..self.cols {
                    host_grid.insert(text(grid_spacer));
                }
            }

            let host_id = if filter.is_empty() {
                id
            } else {
                hosts
                    .iter()
                    .position(|host_| host_.host == host.host)
                    .unwrap()
            };

            host_grid.insert(
                iced::widget::button(text(&host.label))
                    .style(if states[host_id] {
                        theme::Button::Primary
                    } else {
                        theme::Button::Secondary
                    })
                    .on_press(Message::HostPress(host_id)),
            );
        }

        let content = Column::new()
            .align_items(Alignment::Start)
            .spacing(20)
            .push(host_grid);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Debug, Default)]
pub struct StatusBar {
    host_path: String,
}

impl StatusBar {
    pub fn new(env: &Environment) -> Self {
        let host_path = match &env.hosts_path {
            Some(path) => path.to_str().unwrap().to_string(),
            None => "~/.rsnitch-rs/hosts.json".to_string(),
        };

        StatusBar { host_path }
    }

    pub fn view(&self, filter: String) -> Element<Message> {
        let status_bar = text(format!("{}    {}", self.host_path.clone(), filter)).size(20);

        let content = Row::new()
            .align_items(Alignment::Start)
            .spacing(10)
            .push(status_bar);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// main frame
pub struct SnitchUi {
    filter: RwLock<String>,
    group_box: GroupBox,
    groups: Vec<String>,
    host_box: HostBox,
    hosts: Option<Vec<Host>>,
    info_box: InfoBox,
    last: Vec<Event>,
    poll: Poll,
    states: RwLock<Vec<bool>>,
    status_bar: StatusBar,
    poll_interval_secs: u64,
}

#[derive(Clone, Debug)]
pub enum Message {
    Clear,
    ClockTick(time::OffsetDateTime),
    EventOccurred(Event),
    GroupPress(usize),
    HostPress(usize),
    Poll,
}

impl SnitchUi {
    const POLL_INTERVAL: u64 = 10; // sntop uses 180 seconds by default
    const HEADER_TEXT_SIZE: u16 = 20;
    const FRAME_PADDING: u16 = 5;
}

impl Application for SnitchUi {
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Environment;
    type Message = Message;

    fn new(env: Environment) -> (SnitchUi, Command<Message>) {
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

        let filter = RwLock::new(String::new());
        let group_box = GroupBox::new(&env, 5);
        let host_box = HostBox::new(&env, 5);
        let info_box = InfoBox::new(&env, 6, 80);
        let last = Vec::<Event>::new();
        let poll = Poll::new(&env);

        let states = RwLock::new(match &hosts {
            Some(hosts) => poll.poll_all(hosts),
            None => Vec::new(),
        });

        let status_bar = StatusBar::new(&env);

        let snitch_ui = SnitchUi {
            filter,
            group_box,
            groups,
            host_box,
            hosts,
            info_box,
            last,
            poll,
            poll_interval_secs: Self::POLL_INTERVAL,
            states,
            status_bar,
        };

        (snitch_ui, Command::none())
    }

    fn title(&self) -> String {
        String::from("rsnitch 0.0.2")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Poll => match &self.hosts {
                Some(hosts) => {
                    let mut states = self.states.write().unwrap();

                    *states = self.poll.poll_all(hosts);
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
            Message::EventOccurred(_) => (),
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
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

    fn view(&self) -> Element<'_, Message, Renderer> {
        let hosts = self.hosts.as_ref().unwrap();
        let states = self.states.read().unwrap();
        let filter = self.filter.read().unwrap();

        let button_col = Column::new()
            .align_items(Alignment::Start)
            .push(self.group_box.view(&self.groups))
            .push(horizontal_rule(1))
            .push(self.host_box.view(filter.to_string(), hosts, &states));

        let info_col = Column::new()
            .align_items(Alignment::Start)
            .push(self.info_box.view());

        let hosts_frame = Row::new()
            .align_items(Alignment::Start)
            .spacing(5)
            .push(info_col.width(400))
            .push(button_col);

        let snitch = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(Self::HEADER_TEXT_SIZE))
            .push(horizontal_rule(1))
            .push(hosts_frame)
            .push(horizontal_rule(1))
            .push(self.status_bar.view(filter.to_string()));

        container(snitch)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(Self::FRAME_PADDING)
            .into()
    }
}
