// Hide the console window on Windows in release builds.
// On macOS/Linux this attribute is harmless (ignored).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::window;
use iced::Size;
use tracing_subscriber::EnvFilter;

// Core modules
mod api;
mod app;
mod country;
mod http_client;
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
    // Initialize logging: respect RUST_LOG env var if set,
    // otherwise debug builds default to DEBUG, release builds to INFO.
    let default_filter = if cfg!(debug_assertions) {
        "iced_fetch_bitcoin=debug,warn"
    } else {
        "iced_fetch_bitcoin=info,warn"
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(default_filter)),
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
