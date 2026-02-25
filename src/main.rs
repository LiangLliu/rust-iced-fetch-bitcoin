use iced::window;
use iced::Size;
use tracing_subscriber::EnvFilter;

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
fn main() -> iced::Result {
    // Initialize logging: default INFO, override with RUST_LOG env var
    // e.g. RUST_LOG=debug cargo run
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("iced_fetch_bitcoin=debug,warn")),
        )
        .init();

    iced::application(App::boot, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .subscription(App::subscription)
        .window(window::Settings {
            icon: window::icon::from_file_data(WINDOW_ICON, None).ok(),
            position: window::Position::Centered,
            size: Size::new(700.0, 500.0),
            ..Default::default()
        })
        .run()
}
