use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response structure from CoinGecko API
#[derive(Serialize, Deserialize, Debug)]
pub struct CoinGeckoResponse {
    pub bitcoin: HashMap<String, f64>,
}

/// API-related errors
#[derive(Debug)]
pub enum ApiError {
    /// Network request failed
    NetworkError(reqwest::Error),
    /// Failed to parse JSON response
    ParseError(reqwest::Error),
    /// Invalid response format
    InvalidResponse(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NetworkError(err) => write!(f, "Network error: {}", err),
            ApiError::ParseError(err) => write!(f, "Failed to parse response: {}", err),
            ApiError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

/// Fetches Bitcoin prices in the specified currencies
/// 
/// # Arguments
/// * `currencies` - List of currency codes to fetch prices for
/// 
/// # Returns
/// * `Ok((usd_price, response))` on success
/// * `Err(ApiError)` on failure
pub async fn fetch_btc(currencies: Vec<String>) -> Result<(f64, CoinGeckoResponse), ApiError> {
    let currencies_string = currencies.join(",");
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies={}",
        currencies_string
    );
    
    let response = reqwest::get(&url)
        .await
        .map_err(ApiError::NetworkError)?
        .json::<CoinGeckoResponse>()
        .await
        .map_err(ApiError::ParseError)?;

    let usd = response.bitcoin.get("usd").copied().unwrap_or(0.0);
    
    if usd == 0.0 {
        return Err(ApiError::InvalidResponse(
            "USD price not found in response".to_string(),
        ));
    }
    
    Ok((usd, response))
}
