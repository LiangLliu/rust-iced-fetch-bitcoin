use crate::api;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinGeckoResponse {
    pub bitcoin: HashMap<String, f64>,
}

static VS_CURRENCIES: &[&str] = &[
    "btc", "eth", "ltc", "bch", "bnb", "eos", "xrp", "xlm", "link", "dot", "yfi", "usd", "aed",
    "ars", "aud", "bdt", "bhd", "bmd", "brl", "cad", "chf", "clp", "cny", "czk", "dkk", "eur",
    "gbp", "gel", "hkd", "huf", "idr", "ils", "inr", "jpy", "krw", "kwd", "lkr", "mmk", "mxn",
    "myr", "ngn", "nok", "nzd", "php", "pkr", "pln", "rub", "sar", "sek", "sgd", "thb", "try",
    "twd", "uah", "vef", "vnd", "zar", "xdr", "xag", "xau", "bits", "sats",
];

pub async fn fetch_btc() -> (f64, api::CoinGeckoResponse) {
    let currencies_string = VS_CURRENCIES.join(",");
    let url =
        String::from("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=")
            + &*currencies_string;
    let response = reqwest::get(url)
        .await
        .unwrap()
        .json::<api::CoinGeckoResponse>()
        .await
        .unwrap();

    // (response.bitcoin.values.get("usd")?)
    let usd = response.bitcoin.get("usd").copied().unwrap_or(0.0);
    (usd, response)
}
