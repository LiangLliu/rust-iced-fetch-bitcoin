use iced::widget::{column, container, text, vertical_space};
use iced::{Alignment, Length};

use crate::message::AboutMessage;

/// About view component
pub struct AboutView;

impl AboutView {
    /// Creates a new about view
    pub fn new() -> Self {
        Self
    }

    /// Renders the about view
    pub fn view(self) -> iced::Element<'static, AboutMessage> {
        let title = text("About Bitcoin Price Monitor").size(32);
        
        let version_info = column![
            text("Version: 0.1.0").size(16),
            text("Built with Iced Framework").size(14),
            text("Rust Edition 2021").size(14),
        ]
        .spacing(5);

        let description = text(
            "A modern Bitcoin price monitoring application that fetches real-time\n\
            cryptocurrency prices from CoinGecko API and displays them in\n\
            multiple currencies with beautiful country flags."
        )
        .size(14);

        let features = column![
            text("Features:").size(16),
            text("• Real-time Bitcoin price fetching").size(14),
            text("• Multi-currency support").size(14),
            text("• Country flags display").size(14),
            text("• Modern UI with Nord theme").size(14),
            text("• Error handling and loading states").size(14),
        ]
        .spacing(5);

        let credits = column![
            text("Credits:").size(16),
            text("• CoinGecko API for price data").size(14),
            text("• Flag Icons by lipis/flag-icons").size(14),
            text("• Iced GUI framework").size(14),
        ]
        .spacing(5);

        container(
            column![
                title,
                vertical_space().height(20),
                version_info,
                vertical_space().height(20),
                description,
                vertical_space().height(20),
                features,
                vertical_space().height(20),
                credits,
            ]
            .align_x(Alignment::Center)
            .spacing(10)
            .padding(20)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
    }
}
