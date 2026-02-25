use std::collections::HashMap;

use iced::widget::svg;
use iced::Task;

use crate::api;
use crate::country::{get_countries, Country, CountryPrice};
use crate::http_utils::download_svgs_to_memory;
use crate::message::BitcoinMessage;
use crate::views::bitcoin_view::BitcoinView;

/// State for the Bitcoin price page
pub struct BitcoinPage {
    /// Current USD price of Bitcoin
    price_usd: f64,
    /// Bitcoin prices in various currencies with country information
    vs_currencies: Vec<CountryPrice>,
    /// Pre-built SVG flag handles (clone is O(1))
    svg_map: HashMap<String, svg::Handle>,
    /// Loading state indicator
    is_loading: bool,
    /// Error message if any operation fails
    error_message: Option<String>,
}

impl BitcoinPage {
    /// Creates a new Bitcoin page and kicks off both SVG download AND initial price fetch
    pub fn new() -> (Self, Task<BitcoinMessage>) {
        let countries = get_countries();

        let codes: Vec<String> = countries.iter().map(|c| c.country_code.to_string()).collect();
        let flags: Vec<String> = countries.iter().map(|c| c.flag_url.clone()).collect();
        let currencies: Vec<String> = countries.iter().map(|c| c.currency.to_string()).collect();

        let svg_task = Task::perform(
            download_svgs_to_memory(codes, flags),
            BitcoinMessage::SvgLoaded,
        );

        let price_task = Task::perform(
            Self::fetch_prices(countries, currencies),
            |result| result,
        );

        (
            Self {
                price_usd: 0.0,
                vs_currencies: Vec::new(),
                svg_map: HashMap::new(),
                is_loading: true,
                error_message: None,
            },
            Task::batch([svg_task, price_task]),
        )
    }

    /// Updates the page state based on received messages
    pub fn update(&mut self, message: BitcoinMessage) -> Task<BitcoinMessage> {
        match message {
            BitcoinMessage::Refetch => {
                self.is_loading = true;
                self.error_message = None;

                let countries = get_countries();
                let currencies: Vec<String> =
                    countries.iter().map(|c| c.currency.to_string()).collect();

                Task::perform(Self::fetch_prices(countries, currencies), |r| r)
            }
            BitcoinMessage::CurrentPrice((usd, prices)) => {
                self.price_usd = usd;
                self.vs_currencies = prices;
                self.is_loading = false;
                self.error_message = None;
                Task::none()
            }
            BitcoinMessage::SvgLoaded(raw_svg_map) => {
                self.svg_map = raw_svg_map
                    .into_iter()
                    .map(|(code, data)| (code, svg::Handle::from_memory(data)))
                    .collect();
                // Keep loading true if prices haven't arrived yet
                if self.price_usd > 0.0 {
                    self.is_loading = false;
                }
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

    // ── Private helpers ─────────────────────────────────────────

    async fn fetch_prices(
        countries: &'static [Country],
        currencies: Vec<String>,
    ) -> BitcoinMessage {
        match api::fetch_btc(currencies).await {
            Ok((usd, response)) => {
                let prices = countries
                    .iter()
                    .map(|c| {
                        let price = response
                            .bitcoin
                            .get(c.currency)
                            .copied()
                            .unwrap_or(0.0);
                        CountryPrice::new(c.clone(), price)
                    })
                    .collect();
                BitcoinMessage::CurrentPrice((usd, prices))
            }
            Err(e) => BitcoinMessage::Error(format!("Failed to fetch prices: {e}")),
        }
    }
}
