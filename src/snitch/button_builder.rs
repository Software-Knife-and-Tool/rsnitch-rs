//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(unused_imports)]

use {
    super::ui::Message,
    crate::Environment,
    iced::{
        executor,
        subscription, theme,
        widget::{button, column, container, row, rule, text, Column, Space, Text},
        window, Alignment, Application, Color, Command, Element, Event, Length, Subscription,
        Theme,
    },
};

#[derive(Debug, Default)]
pub struct Button {

}

#[derive(Debug, Default)]
pub struct InfoBoxBuilder {
    rows: Option<usize>,
    cols: Option<usize>,
}

impl InfoBoxBuilder {
    const ROWS: usize = 25;
    const COLS: usize = 80;

    pub fn new() -> Self {
        InfoBoxBuilder {
            rows: None,
            cols: None,
        }
    }

    pub fn rows(&self, rows: usize) -> Self {
        InfoBoxBuilder {
            rows: Some(rows),
            cols: self.cols,
        }
    }

    pub fn cols(&self, cols: usize) -> Self {
        InfoBoxBuilder {
            rows: self.rows,
            cols: Some(cols),
        }
    }

    pub fn build(&self) -> InfoBox {
        let rows = match self.rows {
            Some(rows) => rows,
            None => Self::ROWS,
        };

        let cols = match self.cols {
            Some(cols) => cols,
            None => Self::COLS,
        };

        InfoBox {
            image: RwLock::new(String::new()),
            lines: RwLock::new(vec![String::new(); rows]),
            rows,
            cols,
        }
    }
}

impl Button {
    pub fn new(_env: &Environment, rows: usize, cols: usize) -> Self {
        InfoBox {
            image: RwLock::new(String::new()),
            lines: RwLock::new(vec![String::new(); rows]),
            rows,
            cols,
        }
    }
}
