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
    Black,
    Red,
    Green,
    Blue,
}

impl TileColor {
    fn color(&self) -> Color {
        match self {
            TileColor::Red => Color::from_rgb8(235, 100, 52),
            TileColor::Green => Color::from_rgb8(52, 235, 107),
            TileColor::Blue => Color::from_rgb8(52, 55, 235),
            TileColor::Black => Color::from_rgb8(0, 0, 0),
        }
    }

    fn get_color(value: u32) -> Self {
        match value {
            a if (2..=8).contains(&a) => TileColor::Red,
            a if (9..=256).contains(&a) => TileColor::Blue,
            a if a > 256 => TileColor::Green,
            _ => TileColor::Black,
        }
    }
}

pub struct Tile {
    value: Option<u32>,
    color: TileColor,
}

impl Tile {
    pub fn with_value(value: u32) -> Self {
        let val = if value == 0 { None } else { Some(value) };
        Tile {
            value: val,
            color: TileColor::get_color(value),
        }
    }

    pub fn view<'a>(&self) -> Element<'a, AppMessage, Renderer> {
        let mut text = if self.value.is_some() {
            text(self.value.unwrap()).style(self.color.color())
        } else {
            text("")
        };

        text = text
            .height(64)
            .width(64)
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center)
            .size(24);
        container(text).style(theme::Container::Box).into()
    }
}
