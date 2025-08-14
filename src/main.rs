use iced::advanced::graphics::image::image_rs::ImageFormat;
use iced::window::{Position, Settings};
use iced::{window, Point, Size};

// Core modules
mod api;
mod app;
mod country;
mod http_utils;
mod message;
mod route;

// UI modules
mod pages;
mod views;

use app::App;

const WINDOW_ICON: &[u8] = include_bytes!("../resources/Bitcoin.png");



/// Main application entry point
fn main() -> Result<(), iced::Error> {
    iced::application("Bitcoin Price Monitor", App::update, App::view)
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
        .theme(|app| app.theme())
        .run_with(App::new)
}
