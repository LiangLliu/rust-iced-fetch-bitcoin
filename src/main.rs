use iced::advanced::graphics::image::image_rs::ImageFormat;
use iced::window::{Position, Settings};
use iced::{window, Point, Size};

mod api;
mod country;
mod http_utils;
mod message;
mod state;
use state::App;

const WINDOW_ICON: &[u8] = include_bytes!("../resources/Bitcoin.png");

fn theme(_: &App) -> iced::Theme {
    iced::Theme::Nord
}

fn main() -> Result<(), iced::Error> {
    iced::application("Get Latest Bitcoin Price", App::update, App::view)
        .window(Settings {
            icon: window::icon::from_file_data(WINDOW_ICON, Some(ImageFormat::Png)).ok(),
            position: Position::Specific(Point::new(1000.0, 200.0)),
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            size: Size::new(700.0, 500.0),
            ..Default::default()
        })
        .theme(theme)
        .run_with(App::new)
}
