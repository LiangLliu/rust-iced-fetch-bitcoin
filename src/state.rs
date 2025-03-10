use crate::api;
use crate::country::get_countries;
use crate::http_utils::download_svgs_to_memory;
use crate::message::{BitcoinMessage, CountryPrice};
use iced::widget::{image, svg, Column, Container, Row, Scrollable, Text};
use iced::{widget, Alignment, Task};
use std::collections::HashMap;

const DEFAULT_SVG: &[u8] = br#"<svg width="40" height="30" xmlns="http://www.w3.org/2000/svg"><rect width="100%" height="100%" fill="gray"/></svg>"#;

#[derive(Default)]
pub struct App {
    price_usd: f64,
    vs_currencies: Vec<CountryPrice>,
    svg_map: HashMap<String, Vec<u8>>,
}

impl App {
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
            },
            Task::batch([
                Task::perform(download_svgs_to_memory(codes, flags), |svg_map| {
                    BitcoinMessage::SvgLoaded(svg_map)
                }),
                widget::focus_next(),
            ]),
        )
    }

    pub fn view(&self) -> iced::Element<BitcoinMessage> {
        let img = image("resources/Bitcoin.png");
        let img_content = Container::new(img)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink);

        let mut column = Column::new().padding(10);

        for country_price in &self.vs_currencies {
            let country_text = Text::new(&country_price.country.name).size(20);
            let currency_text = Text::new(country_price.country.currency.to_uppercase()).size(20);
            let price_test = Text::new(format!("{:.2}", country_price.price)).size(20);

            let svg_data = self
                .svg_map
                .get(&country_price.country.code)
                .cloned()
                .unwrap_or_else(|| DEFAULT_SVG.to_vec());

            let handle = svg::Handle::from_memory(svg_data);

            let svg_image = svg(handle).width(40).height(30);

            let country_row = Row::new()
                .height(50)
                .spacing(10)
                .align_y(Alignment::Center)
                .push(svg_image.width(iced::Length::Fixed(40f32)))
                .push(country_text.width(iced::Length::FillPortion(6)))
                .push(currency_text.width(iced::Length::FillPortion(2)))
                .push(price_test.width(iced::Length::FillPortion(4)));

            column = column.push(country_row);
        }

        let scrollable = Scrollable::new(column)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill);

        let content = widget::column![
            img_content,
            iced::widget::text(format!("USD     {:.2}", self.price_usd)),
            iced::widget::button("Fetch Current Price").on_press(BitcoinMessage::Refetch),
            scrollable,
        ]
        .width(iced::Fill)
        .spacing(15)
        .padding([10, 50])
        .align_x(Alignment::Center)
        .into();

        content
    }

    pub fn update(&mut self, message: BitcoinMessage) -> Task<BitcoinMessage> {
        match message {
            BitcoinMessage::Refetch => {
                let countries = get_countries();

                let currencies = countries
                    .iter()
                    .map(|country| country.currency.clone())
                    .collect::<Vec<String>>();

                return Task::perform(api::fetch_btc(currencies), move |(usd, response)| {
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
                });
            }
            BitcoinMessage::CurrentPrice((usd, price_map)) => {
                self.price_usd = usd;
                self.vs_currencies = price_map;
            }

            BitcoinMessage::SvgLoaded(svg_map) => {
                self.svg_map = svg_map;
            }
        }

        Task::none()
    }
}
