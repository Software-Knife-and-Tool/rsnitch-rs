//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(clippy::collapsible_match)]
#![allow(unused_imports)]
use {
    super::super::super::tab_ui::Message,
    super::super::hosts_tab::HostsMessage,
    crate::Environment,
    iced::{
        executor,
        keyboard::Event::CharacterReceived,
        subscription, theme,
        widget::{button, column, container, row, rule, text, Column, Space, Text},
        window, Alignment, Application, Color, Command, Element, Event, Length, Renderer,
        Subscription, Theme,
    },
    iced_aw::Grid,
    std::sync::RwLock,
};

#[derive(Debug, Default)]
pub struct GroupBox {
    cols: usize,
}

impl GroupBox {
    pub fn new(_env: &Environment, cols: usize) -> Self {
        GroupBox { cols }
    }

    pub fn view(&self, groups: &[String]) -> iced_native::Element<'_, HostsMessage, Renderer> {
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
                    .on_press(HostsMessage::GroupPress(id)),
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
