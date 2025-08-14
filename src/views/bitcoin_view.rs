use std::collections::HashMap;

use iced::widget::{image, svg, Column, Container, Row, Scrollable, Text};
use iced::{widget, Alignment, Length};

use crate::message::{BitcoinMessage, CountryPrice};

/// Default SVG content for missing flag images
const DEFAULT_SVG: &[u8] = br#"<svg width="40" height="30" xmlns="http://www.w3.org/2000/svg"><rect width="100%" height="100%" fill="gray"/></svg>"#;

/// View component for displaying Bitcoin prices
pub struct BitcoinView<'a> {
    price_usd: f64,
    vs_currencies: &'a Vec<CountryPrice>,
    svg_map: &'a HashMap<String, Vec<u8>>,
    is_loading: bool,
    error_message: Option<&'a str>,
}

impl<'a> BitcoinView<'a> {
    /// Creates a new Bitcoin view instance
    pub fn new(
        price_usd: f64,
        vs_currencies: &'a Vec<CountryPrice>,
        svg_map: &'a HashMap<String, Vec<u8>>,
        is_loading: bool,
        error_message: Option<&'a str>,
    ) -> Self {
        Self {
            price_usd,
            vs_currencies,
            svg_map,
            is_loading,
            error_message,
        }
    }

    /// Renders the Bitcoin view
    pub fn view(self) -> iced::Element<'a, BitcoinMessage> {
        let header = self.build_header();
        let controls = self.build_controls();
        let content = self.build_content();

        widget::column![header, controls, content]
            .width(Length::Fill)
            .spacing(15)
            .padding([10, 50])
            .align_x(Alignment::Center)
            .into()
    }

    /// Builds the header with Bitcoin logo and USD price
    fn build_header(&self) -> iced::Element<'a, BitcoinMessage> {
        let img = image("resources/Bitcoin.png");
        let img_content = Container::new(img)
            .width(Length::Shrink)
            .height(Length::Shrink);

        let usd_price = if self.is_loading {
            Text::new("Loading...").size(24)
        } else {
            Text::new(format!("USD: ${:.2}", self.price_usd)).size(24)
        };

        widget::column![img_content, usd_price]
            .spacing(10)
            .align_x(Alignment::Center)
            .into()
    }

    /// Builds the control buttons
    fn build_controls(&self) -> iced::Element<'a, BitcoinMessage> {
        let fetch_button = widget::button("Fetch Current Price")
            .on_press(BitcoinMessage::Refetch);

        let fetch_button = if self.is_loading {
            widget::button("Loading...")
        } else {
            fetch_button
        };

        Container::new(fetch_button)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
    }

    /// Builds the main content area with currency list or error message
    fn build_content(&self) -> iced::Element<'a, BitcoinMessage> {
        if let Some(error) = self.error_message {
            return Container::new(
                Text::new(format!("Error: {}", error))
                    .size(16)
                    .color([1.0, 0.0, 0.0]),
            )
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into();
        }

        if self.is_loading {
            return Container::new(Text::new("Loading prices...").size(16))
                .width(Length::Fill)
                .center_x(Length::Fill)
                .into();
        }

        self.build_currency_list()
    }

    /// Builds the scrollable list of currencies and their prices
    fn build_currency_list(&self) -> iced::Element<'a, BitcoinMessage> {
        let mut column = Column::new().padding(10);

        // Add header row
        let header_row = Row::new()
            .height(40)
            .spacing(10)
            .align_y(Alignment::Center)
            .push(Text::new("Flag").width(Length::Fixed(40.0)).size(14))
            .push(Text::new("Country").width(Length::FillPortion(6)).size(14))
            .push(Text::new("Currency").width(Length::FillPortion(2)).size(14))
            .push(Text::new("Price").width(Length::FillPortion(4)).size(14));

        column = column.push(header_row);

        // Add separator
        column = column.push(widget::horizontal_rule(1));

        // Add currency rows
        for country_price in self.vs_currencies {
            let country_row = self.build_currency_row(country_price);
            column = column.push(country_row);
        }

        Scrollable::new(column)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    /// Builds a single currency row
    fn build_currency_row(&self, country_price: &'a CountryPrice) -> Row<'a, BitcoinMessage> {
        let country_text = Text::new(&country_price.country.name).size(16);
        let currency_text = Text::new(country_price.country.currency.to_uppercase()).size(16);
        let price_text = Text::new(format!("{:.2}", country_price.price)).size(16);

        let svg_data = self
            .svg_map
            .get(&country_price.country.code)
            .cloned()
            .unwrap_or_else(|| DEFAULT_SVG.to_vec());

        let handle = svg::Handle::from_memory(svg_data);
        let svg_image = svg(handle).width(40).height(30);

        Row::new()
            .height(50)
            .spacing(10)
            .align_y(Alignment::Center)
            .push(svg_image.width(Length::Fixed(40.0)))
            .push(country_text.width(Length::FillPortion(6)))
            .push(currency_text.width(Length::FillPortion(2)))
            .push(price_text.width(Length::FillPortion(4)))
    }
}
