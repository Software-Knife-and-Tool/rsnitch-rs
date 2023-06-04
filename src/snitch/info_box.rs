//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
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
pub struct InfoBox {
    image: RwLock<String>,
    lines: RwLock<Vec<String>>,
    rows: usize,
    cols: usize,
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

impl InfoBox {
    pub fn new(rows: usize, cols: usize) -> Self {
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

    pub fn view(&self) -> Element<Message> {
        Column::new().push(text(self.contents())).into()
    }
}
