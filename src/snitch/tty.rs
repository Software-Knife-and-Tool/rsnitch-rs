//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(unused_imports)]

use {crate::Environment, std::sync::RwLock};

#[derive(Debug)]
pub struct TtyWidget {
    image: RwLock<String>,
    lines: RwLock<Vec<String>>,
    rows: usize,
    cols: usize,
    cursor: char,
}

impl TtyWidget {
    pub fn new(rows: usize, cols: usize) -> Self {
        TtyWidget {
            image: RwLock::new(String::new()),
            lines: RwLock::new(vec![String::new(); rows]),
            rows,
            cols,
            cursor: '\u{00ab}',
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

        img.push(self.cursor);

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

    pub fn write_string(&self, str: String) {
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
}
