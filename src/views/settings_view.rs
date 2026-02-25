use iced::widget::{button, checkbox, column, container, pick_list, row, slider, text, toggler};
use iced::{Center, Fill, Length, Theme};

use crate::message::SettingsMessage;

/// Available themes for the pick-list
const THEME_LIST: &[Theme] = &[
    Theme::Light,
    Theme::Dark,
    Theme::Nord,
    Theme::TokyoNight,
    Theme::Dracula,
    Theme::SolarizedLight,
    Theme::SolarizedDark,
    Theme::GruvboxLight,
    Theme::GruvboxDark,
    Theme::CatppuccinLatte,
    Theme::CatppuccinMocha,
];

/// Settings view component
pub struct SettingsView {
    auto_refresh_interval: u32,
    auto_refresh_enabled: bool,
    selected_theme: Theme,
    notifications_enabled: bool,
}

impl SettingsView {
    pub fn new(
        auto_refresh_interval: u32,
        auto_refresh_enabled: bool,
        selected_theme: &Theme,
        notifications_enabled: bool,
    ) -> Self {
        Self {
            auto_refresh_interval,
            auto_refresh_enabled,
            selected_theme: selected_theme.clone(),
            notifications_enabled,
        }
    }

    pub fn view(self) -> iced::Element<'static, SettingsMessage> {
        let title = text("Settings").size(32);

        // ── Auto-Refresh Section ────────────────────────────────
        let refresh_section = column![
            text("Auto Refresh").size(18),
            row![
                text("Enable auto-refresh").width(Fill),
                toggler(self.auto_refresh_enabled)
                    .on_toggle(SettingsMessage::AutoRefreshToggled)
                    .size(25),
            ]
            .align_y(Center)
            .spacing(10),
            row![
                slider(
                    5..=300,
                    self.auto_refresh_interval,
                    SettingsMessage::AutoRefreshIntervalChanged
                )
                .step(5u32),
                text(format!("{}s", self.auto_refresh_interval))
                    .width(Length::Fixed(50.0))
            ]
            .spacing(10)
            .align_y(Center),
        ]
        .spacing(10);

        // ── Theme Section ───────────────────────────────────────
        let theme_section = column![
            text("Theme").size(18),
            pick_list(
                THEME_LIST,
                Some(self.selected_theme),
                SettingsMessage::ThemeChanged
            )
            .placeholder("Select theme...")
            .width(200)
        ]
        .spacing(10);

        // ── Notifications Section ───────────────────────────────
        let notifications_section = column![
            text("Notifications").size(18),
            checkbox(self.notifications_enabled)
                .label("Enable price alerts")
                .on_toggle(SettingsMessage::NotificationsToggled)
                .size(20),
        ]
        .spacing(10);

        // ── Reset Button ────────────────────────────────────────
        let reset_button = button("Reset to Defaults")
            .on_press(SettingsMessage::ResetToDefaults)
            .style(button::danger);

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
        .center_x(Fill)
        .into()
    }
}
