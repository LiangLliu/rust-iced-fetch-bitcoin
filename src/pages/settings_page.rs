use iced::Task;

use crate::message::SettingsMessage;
use crate::views::settings_view::SettingsView;

/// Settings page for application configuration
#[derive(Debug, Clone)]
pub struct SettingsPage {
    /// Auto refresh interval in seconds
    auto_refresh_interval: u32,
    /// Theme selection
    selected_theme: Theme,
    /// Enable notifications
    notifications_enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Nord,
}

impl Default for SettingsPage {
    fn default() -> Self {
        Self {
            auto_refresh_interval: 30,
            selected_theme: Theme::Nord,
            notifications_enabled: false,
        }
    }
}

impl SettingsPage {
    /// Creates a new settings page
    pub fn new() -> (Self, Task<SettingsMessage>) {
        (Self::default(), Task::none())
    }

    /// Updates the settings page state
    pub fn update(&mut self, message: SettingsMessage) -> Task<SettingsMessage> {
        match message {
            SettingsMessage::AutoRefreshIntervalChanged(interval) => {
                self.auto_refresh_interval = interval;
            }
            SettingsMessage::ThemeChanged(theme) => {
                self.selected_theme = theme;
            }
            SettingsMessage::NotificationsToggled(enabled) => {
                self.notifications_enabled = enabled;
            }
            SettingsMessage::ResetToDefaults => {
                *self = Self::default();
            }
        }
        Task::none()
    }

    /// Renders the settings page view
    pub fn view(&self) -> iced::Element<'_, SettingsMessage> {
        SettingsView::new(
            self.auto_refresh_interval,
            &self.selected_theme,
            self.notifications_enabled,
        )
        .view()
    }
}
