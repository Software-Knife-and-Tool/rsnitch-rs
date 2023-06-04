//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(clippy::collapsible_match)]
#![allow(unused_imports)]
use {
    super::{host::Host, ui::Message},
    crate::Environment,
    iced::{
        executor,
        keyboard::Event::CharacterReceived,
        subscription, theme,
        widget::{button, column, container, row, rule, text, Column, Space, Text},
        window, Alignment, Application, Color, Command, Element, Event, Length, Subscription,
        Theme,
    },
    iced_aw::Grid,
    std::sync::RwLock,
};

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

    pub fn view(&self, filter: String, hosts: &[Host], states: &[bool]) -> Element<Message> {
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
