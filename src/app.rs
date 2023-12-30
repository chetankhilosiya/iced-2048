use iced::{
    executor,
    widget::{column, container, horizontal_space, row, scrollable::Viewport, text},
    Application, Command, Length, Theme,
};

use crate::tile::Tile;

pub struct App {
    score: u32,
    grid_values: [[u32; 4]; 4],
    grid: Vec<Vec<Tile>>,
}

#[derive(Debug)]
pub enum AppMessage {}

impl Application for App {
    type Message = AppMessage;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut grid = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut row: Vec<Tile> = Vec::with_capacity(4);
            for _ in 0..4 {
                row.push(Tile::new());
            }
            grid.push(row);
        }
        (
            Self {
                score: 0,
                grid_values: [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                grid,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Iced 2048")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {}
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
                grid_row = grid_row.push(self.grid[i][j].view());
            }
            grid_view = grid_view.push(grid_row);
        }

        container(column![score_bar, grid_view].padding(32)).into()
    }
}
