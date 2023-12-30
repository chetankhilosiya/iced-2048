mod app;
mod game;
mod tile;

use iced::{self, Application, Settings};

fn main() -> iced::Result {
    app::App::run(Settings::default())
}
