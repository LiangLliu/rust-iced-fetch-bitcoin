use crate::country::Country;
use crate::route::Route;
use iced::Theme;
use std::collections::HashMap;

/// Represents a country with its corresponding Bitcoin price
#[derive(Debug, Clone)]
pub struct CountryPrice {
    pub country: Country,
    pub price: f64,
}

impl CountryPrice {
    pub fn new(country: Country, price: f64) -> Self {
        Self { country, price }
    }
}

/// Application-level messages
#[derive(Debug, Clone)]
pub enum Message {
    /// Navigate to a different page
    Navigate(Route),
    /// Messages related to Bitcoin page functionality
    Bitcoin(BitcoinMessage),
    /// Messages related to Settings page functionality
    Settings(SettingsMessage),
    /// Messages related to About page functionality
    About(AboutMessage),
    /// Auto-refresh tick from subscription
    Tick,
}

/// Messages specific to Bitcoin price fetching and display
#[derive(Debug, Clone)]
pub enum BitcoinMessage {
    /// Trigger a refetch of Bitcoin prices
    Refetch,
    /// Received current Bitcoin prices
    CurrentPrice((f64, Vec<CountryPrice>)),
    /// SVG flag images have been loaded
    SvgLoaded(HashMap<String, Vec<u8>>),
    /// Error occurred during operation
    Error(String),
}

/// Messages specific to Settings page
#[derive(Debug, Clone)]
pub enum SettingsMessage {
    /// Auto refresh interval changed (in seconds)
    AutoRefreshIntervalChanged(u32),
    /// Theme selection changed — uses iced's built-in Theme
    ThemeChanged(Theme),
    /// Notifications toggle changed
    NotificationsToggled(bool),
    /// Auto-refresh toggle changed
    AutoRefreshToggled(bool),
    /// Reset all settings to defaults
    ResetToDefaults,
}

/// Messages specific to About page
#[derive(Debug, Clone)]
pub enum AboutMessage {
    /// No-op placeholder (page is purely static)
    #[allow(dead_code)]
    OpenUrl(String),
}
