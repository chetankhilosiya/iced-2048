use iced::{
    executor, keyboard, subscription,
    widget::{column, container, horizontal_space, row, text, vertical_space},
    Application, Command, Event, Length, Theme,
};

use crate::game;
use crate::tile::Tile;

pub struct App {
    score: u32,
    grid_values: [[u32; 4]; 4],
}

#[derive(Debug)]
pub enum AppMessage {
    EventOccured(Event),
}

impl Application for App {
    type Message = AppMessage;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let grid_values: [[u32; 4]; 4] = [[0, 4, 4, 0], [0, 2, 0, 0], [2, 0, 2, 0], [2, 4, 2, 0]];
        (
            Self {
                score: 0,
                grid_values,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Iced 2048")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::EventOccured(e) => match e {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code,
                    modifiers: _,
                }) => {
                    match key_code {
                        keyboard::KeyCode::Up => {
                            game::slide_up(&mut self.grid_values);
                        }
                        keyboard::KeyCode::Down => {}
                        keyboard::KeyCode::Left => {}
                        keyboard::KeyCode::Right => {}
                        _ => (),
                    }

                    Command::none()
                }

                _ => Command::none(),
            },
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::events().map(AppMessage::EventOccured)
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let score_bar = row![
            horizontal_space(Length::Fill),
            text("Score: "),
            text(self.score).size(16),
            horizontal_space(Length::Fill)
        ];
        let mut grid_view = column![];
        for i in 0..4 {
            let mut grid_row = row![];
            for j in 0..4 {
                grid_row = grid_row
                    .push(Tile::with_value(self.grid_values[i][j]).view())
                    .push(horizontal_space(Length::Fixed(4.0)));
            }
            grid_view = grid_view
                .push(grid_row)
                .push(vertical_space(Length::Fixed(4.0)));
        }

        let middle_section = row![
            horizontal_space(Length::Fill),
            grid_view,
            horizontal_space(Length::Fill)
        ];

        container(
            column![
                score_bar,
                vertical_space(Length::Fixed(64.0)),
                middle_section
            ]
            .padding(32),
        )
        .into()
    }
}
