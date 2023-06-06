// SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
// SPDX-License-Identifier: MIT

// quad widget
use iced_native::{
    layout::{self, Layout},
    renderer,
    widget::{self, Widget},
    Color, Element, Length, Point, Rectangle, Size,
};

#[derive(Default, Copy, Clone, Debug)]
pub struct Quad {
    pub width: i32,
    pub height: i32,
}

impl Quad {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    pub fn width(&self, width: i32) -> Self {
        Self {
            width,
            height: self.height,
        }
    }

    pub fn height(&self, height: i32) -> Self {
        Self {
            width: self.width,
            height,
        }
    }
}

pub fn quad(width: i32, height: i32) -> Quad {
    Quad { width, height }
}

impl<Message, Renderer> Widget<Message, Renderer> for Quad
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::new(self.width as f32, self.height as f32))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Color::BLACK,
        );
    }
}

impl<'a, Message, Renderer> From<Quad> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(quad: Quad) -> Self {
        Self::new(quad)
    }
}
