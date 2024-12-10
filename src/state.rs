use crate::api;
use crate::message::BitcoinMessage;
use iced::widget::{image, Column, Container, Scrollable, Text};
use iced::{widget, Task};
use std::collections::HashMap;

#[derive(Default)]
pub struct App {
    price_usd: f64,
    vs_currencies: HashMap<String, f64>,
}

impl App {
    pub fn new() -> Self {
        Self {
            price_usd: 0.0,
            vs_currencies: HashMap::new(),
        }
    }

    pub fn view(&self) -> iced::Element<BitcoinMessage> {
        let img = image("resources/Bitcoin.png");
        let img_content = Container::new(img)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink);

        let mut column = Column::new().padding(10);

        for (currency, price) in &self.vs_currencies {
            let text = format!(" {:<10}  {:<15.2}", currency.to_uppercase(), price);
            column = column.push(Text::new(text));
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
        .align_x(iced::Alignment::Center)
        .into();

        content
    }

    pub fn update(&mut self, message: BitcoinMessage) -> Task<BitcoinMessage> {
        match message {
            BitcoinMessage::Refetch => {
                return Task::perform(api::fetch_btc(), |(usd, response)| {
                    BitcoinMessage::CurrentPrice((usd, response.bitcoin))
                })
            }
            BitcoinMessage::CurrentPrice((usd, price_map)) => {
                self.price_usd = usd;
                self.vs_currencies = price_map;
            }
        }

        Task::none()
    }
}
