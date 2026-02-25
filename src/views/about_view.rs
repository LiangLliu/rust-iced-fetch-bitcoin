use iced::widget::{column, container, space, text};
use iced::{Center, Fill};

use crate::message::Message;

/// About view component
pub struct AboutView;

impl AboutView {
    pub fn new() -> Self {
        Self
    }

    pub fn view(self) -> iced::Element<'static, Message> {
        let title = text("About Bitcoin Price Monitor").size(32);

        let version_info = column![
            text(format!("Version: {}", env!("CARGO_PKG_VERSION"))).size(16),
            text("Built with Iced 0.14").size(14),
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
            text("  •  Real-time Bitcoin price fetching").size(14),
            text("  •  Auto-refresh with configurable interval").size(14),
            text("  •  Multi-currency support (45 currencies)").size(14),
            text("  •  Country flags display (SVG)").size(14),
            text("  •  Multiple theme support").size(14),
            text("  •  Multi-page navigation").size(14),
        ]
        .spacing(5);

        let credits = column![
            text("Credits:").size(16),
            text("  •  CoinGecko API for price data").size(14),
            text("  •  Flag Icons by lipis/flag-icons").size(14),
            text("  •  Iced GUI framework").size(14),
        ]
        .spacing(5);

        container(
            column![
                title,
                space::vertical().height(20),
                version_info,
                space::vertical().height(20),
                description,
                space::vertical().height(20),
                features,
                space::vertical().height(20),
                credits,
            ]
            .align_x(Center)
            .spacing(10)
            .padding(20)
        )
        .center_x(Fill)
        .into()
    }
}
