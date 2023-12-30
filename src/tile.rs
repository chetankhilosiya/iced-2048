use iced::{
    alignment::{Horizontal, Vertical},
    theme,
    widget::{container, text},
    Color, Element, Renderer,
};

use crate::app::AppMessage;

///
/// Tile is used to display number. The number is aligned in center.
/// Tile will have color scheme based on number inside it.
///

enum TileColor {
    BLACK,
    RED,
    GREEN,
    BLUE,
}

impl TileColor {
    fn color(&self) -> Color {
        match self {
            TileColor::RED => Color::from_rgb8(235, 100, 52),
            TileColor::GREEN => Color::from_rgb8(52, 235, 107),
            TileColor::BLUE => Color::from_rgb8(52, 55, 235),
            TileColor::BLACK => Color::from_rgb8(0, 0, 0),
        }
    }

    fn get_color(value: u32) -> Self {
        match value {
            a if (2..=8).contains(&a) => TileColor::RED,
            a if (9..=256).contains(&a) => TileColor::BLUE,
            a if a > 256 => TileColor::GREEN,
            _ => TileColor::BLACK,
        }
    }
}

pub struct Tile {
    value: Option<u32>,
    color: TileColor,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            value: Some(2),
            color: TileColor::BLACK,
        }
    }

    pub fn update(&mut self, value: u32) {
        self.value = if value > 0 { Some(value) } else { None };
        self.color = TileColor::get_color(value)
    }

    pub fn view<'a>(&self) -> Element<'a, AppMessage, Renderer> {
        let mut text = if self.value.is_some() {
            text(self.value.unwrap()).style(self.color.color())
        } else {
            text("")
        };
        text = text
            .height(36)
            .width(36)
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center)
            .size(16);
        container(text).style(theme::Container::Box).into()
    }
}
