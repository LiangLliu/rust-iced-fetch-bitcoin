use iced::Theme;

use crate::message::SettingsMessage;
use crate::views::settings_view::SettingsView;

/// Settings page — owns configuration that the App layer reads
#[derive(Debug, Clone)]
pub struct SettingsPage {
    /// Auto refresh interval in seconds
    auto_refresh_interval: u32,
    /// Whether auto-refresh is enabled
    auto_refresh_enabled: bool,
    /// Theme selection — uses iced's built-in Theme directly
    selected_theme: Theme,
    /// Enable notifications
    notifications_enabled: bool,
}

impl Default for SettingsPage {
    fn default() -> Self {
        Self {
            auto_refresh_interval: 30,
            auto_refresh_enabled: false,
            selected_theme: Theme::Nord,
            notifications_enabled: false,
        }
    }
}

impl SettingsPage {
    pub fn new() -> Self {
        Self::default()
    }

    // ── Public getters (read by App) ────────────────────────────

    pub fn selected_theme(&self) -> &Theme {
        &self.selected_theme
    }

    pub fn auto_refresh_enabled(&self) -> bool {
        self.auto_refresh_enabled
    }

    pub fn auto_refresh_interval(&self) -> u32 {
        self.auto_refresh_interval
    }

    // ── Update ──────────────────────────────────────────────────

    pub fn update(&mut self, message: SettingsMessage) {
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
            SettingsMessage::AutoRefreshToggled(enabled) => {
                self.auto_refresh_enabled = enabled;
            }
            SettingsMessage::ResetToDefaults => {
                *self = Self::default();
            }
        }
    }

    // ── View ────────────────────────────────────────────────────

    pub fn view(&self) -> iced::Element<'_, SettingsMessage> {
        SettingsView::new(
            self.auto_refresh_interval,
            self.auto_refresh_enabled,
            &self.selected_theme,
            self.notifications_enabled,
        )
        .view()
    }
}
