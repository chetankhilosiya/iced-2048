use iced::{self, Application, Settings};

mod app;
mod tile;

fn main() -> iced::Result {
    app::App::run(Settings::default())
}
