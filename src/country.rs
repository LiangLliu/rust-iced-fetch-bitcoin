static FLAG_API_URL: &str =
    "https://raw.githubusercontent.com/lipis/flag-icons/refs/heads/main/flags/4x3/";

#[derive(Debug, Clone)]
pub struct Country {
    pub currency: String,
    pub name: String,
    pub flag: String,
    pub code: String,
}

impl Country {
    fn new(currency: &str, name: &str) -> Country {
        Self {
            currency: String::from(currency),
            name: String::from(name),
            flag: String::from(FLAG_API_URL.to_owned() + (&currency[0..2]) + ".svg"),
            code: String::from(&currency[0..2]),
        }
    }
}

pub fn get_countries() -> Vec<Country> {
    Vec::from([
        Country::new("aed", "United Arab Emirates"),
        Country::new("ars", "Argentina"),
        Country::new("aud", "Australia"),
        Country::new("bdt", "Bangladesh"),
        Country::new("bhd", "Bahrain"),
        Country::new("bmd", "Bermuda"),
        Country::new("brl", "Brazil"),
        Country::new("cad", "Canada"),
        Country::new("chf", "Switzerland"),
        Country::new("clp", "Chile"),
        Country::new("cny", "China"),
        Country::new("czk", "Czech Republic"),
        Country::new("dkk", "Denmark"),
        Country::new("gbp", "United Kingdom"),
        Country::new("gel", "Georgia"),
        Country::new("hkd", "China Hong Kong"),
        Country::new("huf", "Hungary"),
        Country::new("idr", "Indonesia"),
        Country::new("ils", "Israel"),
        Country::new("inr", "India"),
        Country::new("jpy", "Japan"),
        Country::new("krw", "South Korea"),
        Country::new("kwd", "Kuwait"),
        Country::new("lkr", "Sri Lanka"),
        Country::new("mmk", "Myanmar"),
        Country::new("mxn", "Mexico"),
        Country::new("myr", "Malaysia"),
        Country::new("ngn", "Nigeria"),
        Country::new("nok", "Norway"),
        Country::new("nzd", "New Zealand"),
        Country::new("php", "Philippines"),
        Country::new("pkr", "Pakistan"),
        Country::new("pln", "Poland"),
        Country::new("rub", "Russia"),
        Country::new("sar", "Saudi Arabia"),
        Country::new("sek", "Sweden"),
        Country::new("sgd", "Singapore"),
        Country::new("thb", "Thailand"),
        Country::new("try", "Turkey"),
        Country::new("twd", "China Taiwan"),
        Country::new("uah", "Ukraine"),
        Country::new("usd", "United States"),
        Country::new("vef", "Venezuela"),
        Country::new("vnd", "Vietnam"),
        Country::new("zar", "South Africa"),
    ])
}
