use iced::window::Position;
use iced::{window, Point, Size};

mod api;
mod country;
mod http_utils;
mod message;
mod state;
use state::App;

fn theme(_: &App) -> iced::Theme {
    iced::Theme::Nord
}

fn main() -> Result<(), iced::Error> {
    iced::application("Get Latest Bitcoin Price", App::update, App::view)
        .window(window::Settings {
            position: Position::Specific(Point::new(1000.0, 200.0)),
            resizable: false,
            size: Size::new(600.0, 500.0),
            ..Default::default()
        })
        .theme(theme)
        .run_with(App::new)

    //
    // let download_dir = "./asserts";
    //
    // http_utils::download_files(currencies.clone(), download_dir)
    //     .await
    //     .expect("TODO: panic message");
}
