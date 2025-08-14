use std::collections::HashMap;

use iced::{widget, Task};

use crate::api;
use crate::country::get_countries;
use crate::http_utils::download_svgs_to_memory;
use crate::message::{BitcoinMessage, CountryPrice};
use crate::views::bitcoin_view::BitcoinView;

/// State for the Bitcoin price page
#[derive(Default)]
pub struct BitcoinPage {
    /// Current USD price of Bitcoin
    price_usd: f64,
    /// Bitcoin prices in various currencies with country information
    vs_currencies: Vec<CountryPrice>,
    /// SVG flag data for countries
    svg_map: HashMap<String, Vec<u8>>,
    /// Loading state indicator
    is_loading: bool,
    /// Error message if any operation fails
    error_message: Option<String>,
}

impl BitcoinPage {
    /// Creates a new Bitcoin page instance
    pub fn new() -> (Self, Task<BitcoinMessage>) {
        let countries = get_countries();

        let codes = countries
            .iter()
            .map(|country| country.code.clone())
            .collect::<Vec<String>>();

        let flags = countries
            .iter()
            .map(|country| country.flag.clone())
            .collect::<Vec<String>>();

        (
            Self {
                price_usd: 0.0,
                vs_currencies: Vec::new(),
                svg_map: HashMap::new(),
                is_loading: true,
                error_message: None,
            },
            Task::batch([
                Task::perform(download_svgs_to_memory(codes, flags), |svg_map| {
                    BitcoinMessage::SvgLoaded(svg_map)
                }),
                widget::focus_next(),
            ]),
        )
    }

    /// Updates the page state based on received messages
    pub fn update(&mut self, message: BitcoinMessage) -> Task<BitcoinMessage> {
        match message {
            BitcoinMessage::Refetch => {
                self.is_loading = true;
                self.error_message = None;
                
                let countries = get_countries();
                let currencies = countries
                    .iter()
                    .map(|country| country.currency.clone())
                    .collect::<Vec<String>>();

                Task::perform(
                    async move {
                        match api::fetch_btc(currencies).await {
                            Ok((usd, response)) => {
                                let country_prices = countries
                                    .iter()
                                    .map(|country| {
                                        CountryPrice::new(
                                            country.clone(),
                                            response
                                                .bitcoin
                                                .get(&country.currency)
                                                .cloned()
                                                .unwrap_or(0.0),
                                        )
                                    })
                                    .collect::<Vec<CountryPrice>>();

                                BitcoinMessage::CurrentPrice((usd, country_prices))
                            }
                            Err(e) => BitcoinMessage::Error(format!("Failed to fetch Bitcoin prices: {}", e)),
                        }
                    },
                    |result| result,
                )
            }
            BitcoinMessage::CurrentPrice((usd, price_map)) => {
                self.price_usd = usd;
                self.vs_currencies = price_map;
                self.is_loading = false;
                self.error_message = None;
                Task::none()
            }
            BitcoinMessage::SvgLoaded(svg_map) => {
                self.svg_map = svg_map;
                self.is_loading = false;
                Task::none()
            }
            BitcoinMessage::Error(error) => {
                self.error_message = Some(error);
                self.is_loading = false;
                Task::none()
            }
        }
    }

    /// Renders the page view
    pub fn view(&self) -> iced::Element<'_, BitcoinMessage> {
        BitcoinView::new(
            self.price_usd,
            &self.vs_currencies,
            &self.svg_map,
            self.is_loading,
            self.error_message.as_deref(),
        )
        .view()
    }
}
