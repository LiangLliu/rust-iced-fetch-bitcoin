use crate::country::Country;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CountryPrice {
    pub country: Country,
    pub price: f64,
}

impl CountryPrice {
    pub fn new(country: Country, price: f64) -> Self {
        Self { country, price }
    }
}

#[derive(Debug, Clone)]
pub enum BitcoinMessage {
    Refetch,
    CurrentPrice((f64, Vec<CountryPrice>)),
    SvgLoaded(HashMap<String, Vec<u8>>),
}
