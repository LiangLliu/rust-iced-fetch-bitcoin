use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinGeckoResponse {
    pub bitcoin: HashMap<String, f64>,
}

pub async fn fetch_btc(currencies: Vec<String>) -> (f64, CoinGeckoResponse) {
    let currencies_string = currencies.join(",");
    let url =
        String::from("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=")
            + &*currencies_string;
    let response = reqwest::get(url)
        .await
        .unwrap()
        .json::<CoinGeckoResponse>()
        .await
        .unwrap();

    let usd = response.bitcoin.get("usd").copied().unwrap_or(0.0);
    (usd, response)
}
