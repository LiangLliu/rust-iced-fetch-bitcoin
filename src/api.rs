use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

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
    ParseError(String),
    /// Invalid response format
    InvalidResponse(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NetworkError(err) => write!(f, "Network error: {}", err),
            ApiError::ParseError(msg) => write!(f, "Failed to parse response: {}", msg),
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

    info!("Fetching BTC prices from: {}", url);

    // Build a client with a proper User-Agent (required by CoinGecko)
    let client = reqwest::Client::builder()
        .user_agent("iced-fetch-bitcoin/0.2.0")
        .build()
        .map_err(|e| {
            error!("Failed to build HTTP client: {e}");
            ApiError::NetworkError(e)
        })?;

    // Step 1: Send HTTP request
    let http_response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            error!("HTTP request failed: {e}");
            ApiError::NetworkError(e)
        })?;

    let status = http_response.status();
    debug!("Response status: {status}");

    // Step 2: Read raw body text for debugging
    let body_text = http_response.text().await.map_err(|e| {
        error!("Failed to read response body: {e}");
        ApiError::NetworkError(e)
    })?;

    debug!("Raw response body (first 500 chars): {}", &body_text[..body_text.len().min(500)]);

    if !status.is_success() {
        warn!("API returned non-success status {status}: {body_text}");
        return Err(ApiError::InvalidResponse(format!(
            "HTTP {status}: {body_text}"
        )));
    }

    // Step 3: Parse JSON from raw text
    let response: CoinGeckoResponse = serde_json::from_str(&body_text).map_err(|e| {
        error!("JSON parse error: {e}");
        debug!("Full body that failed to parse: {body_text}");
        ApiError::ParseError(format!("{e} | body: {}", &body_text[..body_text.len().min(200)]))
    })?;

    debug!("Parsed response: {:?}", response);

    let usd = response.bitcoin.get("usd").copied().unwrap_or(0.0);

    if usd == 0.0 {
        warn!("USD price is 0 or missing in response");
        return Err(ApiError::InvalidResponse(
            "USD price not found in response".to_string(),
        ));
    }

    info!("BTC/USD = {usd:.2}");
    Ok((usd, response))
}
