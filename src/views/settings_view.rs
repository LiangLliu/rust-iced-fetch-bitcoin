use iced::widget::{button, checkbox, column, container, pick_list, row, slider, text};
use iced::{Alignment, Length};

use crate::message::SettingsMessage;
use crate::pages::settings_page::Theme;

/// Settings view component
pub struct SettingsView {
    auto_refresh_interval: u32,
    selected_theme: Theme,
    notifications_enabled: bool,
}

impl SettingsView {
    /// Creates a new settings view
    pub fn new(
        auto_refresh_interval: u32,
        selected_theme: &Theme,
        notifications_enabled: bool,
    ) -> Self {
        Self {
            auto_refresh_interval,
            selected_theme: selected_theme.clone(),
            notifications_enabled,
        }
    }

    /// Renders the settings view
    pub fn view(self) -> iced::Element<'static, SettingsMessage> {
        let title = text("Settings").size(32);

        let refresh_section = column![
            text("Auto Refresh Interval").size(16),
            row![
                slider(
                    5..=300,
                    self.auto_refresh_interval,
                    SettingsMessage::AutoRefreshIntervalChanged
                ),
                text(format!("{} seconds", self.auto_refresh_interval)).width(Length::Fixed(80.0))
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        ]
        .spacing(10);

        let theme_section = column![
            text("Theme").size(16),
            pick_list(
                vec![Theme::Light, Theme::Dark, Theme::Nord],
                Some(self.selected_theme),
                SettingsMessage::ThemeChanged
            )
            .placeholder("Select theme...")
        ]
        .spacing(10);

        let notifications_section = column![
            text("Notifications").size(16),
            checkbox("Enable price alerts", self.notifications_enabled)
                .on_toggle(SettingsMessage::NotificationsToggled)
        ]
        .spacing(10);

        let reset_button = button("Reset to Defaults")
            .on_press(SettingsMessage::ResetToDefaults)
            .style(|_theme, _status| iced::widget::button::Style {
                background: Some(iced::Background::Color([0.8, 0.2, 0.2, 1.0].into())),
                text_color: [1.0, 1.0, 1.0, 1.0].into(),
                border: iced::Border {
                    color: [0.6, 0.1, 0.1, 1.0].into(),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow::default(),
            });

        container(
            column![
                title,
                refresh_section,
                theme_section,
                notifications_section,
                reset_button
            ]
            .spacing(30)
            .padding(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Light => write!(f, "Light"),
            Theme::Dark => write!(f, "Dark"),
            Theme::Nord => write!(f, "Nord"),
        }
    }
}
