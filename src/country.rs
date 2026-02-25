use std::sync::LazyLock;

static FLAG_API_URL: &str =
    "https://raw.githubusercontent.com/lipis/flag-icons/refs/heads/main/flags/4x3/";

/// Country with currency and flag information
#[derive(Debug, Clone)]
pub struct Country {
    /// ISO 4217 currency code (lowercase, e.g. "usd")
    pub currency: &'static str,
    /// Country display name
    pub name: &'static str,
    /// ISO 3166-1 alpha-2 country code (lowercase, e.g. "us")
    pub country_code: &'static str,
    /// Full URL to the SVG flag image
    pub flag_url: String,
}

/// A country paired with its Bitcoin price
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

impl Country {
    fn new(currency: &'static str, name: &'static str, country_code: &'static str) -> Country {
        Self {
            currency,
            name,
            country_code,
            flag_url: format!("{}{}.svg", FLAG_API_URL, country_code),
        }
    }
}

/// Cached list of supported countries. Allocated once on first access.
static COUNTRIES: LazyLock<Vec<Country>> = LazyLock::new(|| {
    vec![
        Country::new("aed", "United Arab Emirates", "ae"),
        Country::new("ars", "Argentina", "ar"),
        Country::new("aud", "Australia", "au"),
        Country::new("bdt", "Bangladesh", "bd"),
        Country::new("bhd", "Bahrain", "bh"),
        Country::new("bmd", "Bermuda", "bm"),
        Country::new("brl", "Brazil", "br"),
        Country::new("cad", "Canada", "ca"),
        Country::new("chf", "Switzerland", "ch"),
        Country::new("clp", "Chile", "cl"),
        Country::new("cny", "China", "cn"),
        Country::new("czk", "Czech Republic", "cz"),
        Country::new("dkk", "Denmark", "dk"),
        Country::new("gbp", "United Kingdom", "gb"),
        Country::new("gel", "Georgia", "ge"),
        Country::new("hkd", "China Hong Kong", "hk"),
        Country::new("huf", "Hungary", "hu"),
        Country::new("idr", "Indonesia", "id"),
        Country::new("ils", "Israel", "il"),
        Country::new("inr", "India", "in"),
        Country::new("jpy", "Japan", "jp"),
        Country::new("krw", "South Korea", "kr"),
        Country::new("kwd", "Kuwait", "kw"),
        Country::new("lkr", "Sri Lanka", "lk"),
        Country::new("mmk", "Myanmar", "mm"),
        Country::new("mxn", "Mexico", "mx"),
        Country::new("myr", "Malaysia", "my"),
        Country::new("ngn", "Nigeria", "ng"),
        Country::new("nok", "Norway", "no"),
        Country::new("nzd", "New Zealand", "nz"),
        Country::new("php", "Philippines", "ph"),
        Country::new("pkr", "Pakistan", "pk"),
        Country::new("pln", "Poland", "pl"),
        Country::new("rub", "Russia", "ru"),
        Country::new("sar", "Saudi Arabia", "sa"),
        Country::new("sek", "Sweden", "se"),
        Country::new("sgd", "Singapore", "sg"),
        Country::new("thb", "Thailand", "th"),
        Country::new("try", "Turkey", "tr"),
        Country::new("twd", "China Taiwan", "tw"),
        Country::new("uah", "Ukraine", "ua"),
        Country::new("usd", "United States", "us"),
        Country::new("vef", "Venezuela", "ve"),
        Country::new("vnd", "Vietnam", "vn"),
        Country::new("zar", "South Africa", "za"),
    ]
});

/// Returns a reference to the cached country list.
pub fn get_countries() -> &'static [Country] {
    &COUNTRIES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn country_count() {
        assert_eq!(get_countries().len(), 45);
    }

    #[test]
    fn country_code_is_two_ascii_chars() {
        for c in get_countries() {
            assert_eq!(c.country_code.len(), 2, "Bad country code for {}", c.name);
            assert!(c.country_code.is_ascii(), "Non-ASCII country code for {}", c.name);
        }
    }

    #[test]
    fn currency_is_three_ascii_chars() {
        for c in get_countries() {
            assert_eq!(c.currency.len(), 3, "Bad currency for {}", c.name);
            assert!(c.currency.is_ascii(), "Non-ASCII currency for {}", c.name);
        }
    }

    #[test]
    fn flag_url_contains_country_code() {
        for c in get_countries() {
            assert!(c.flag_url.ends_with(".svg"), "Bad flag URL for {}", c.name);
            assert!(c.flag_url.contains(c.country_code), "Flag URL missing code for {}", c.name);
        }
    }

    #[test]
    fn no_duplicate_currencies() {
        let mut currencies: Vec<_> = get_countries().iter().map(|c| c.currency).collect();
        currencies.sort();
        let len_before = currencies.len();
        currencies.dedup();
        assert_eq!(len_before, currencies.len(), "Duplicate currencies found");
    }
}
