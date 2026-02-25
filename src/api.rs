use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

use crate::http_client::CLIENT;

/// Response structure from CoinGecko API
#[derive(Serialize, Deserialize, Debug)]
pub struct CoinGeckoResponse {
    pub bitcoin: HashMap<String, f64>,
}

/// API-related errors
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Network request failed
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    /// Failed to parse JSON response
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    /// Invalid response format
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

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

    // Step 1: Send HTTP request (using shared client with connection pooling)
    let http_response = CLIENT
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_response() {
        let json = r#"{"bitcoin":{"usd":65497.0,"eur":60123.0,"gbp":51234.0}}"#;
        let response: CoinGeckoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.bitcoin["usd"], 65497.0);
        assert_eq!(response.bitcoin["eur"], 60123.0);
        assert_eq!(response.bitcoin.len(), 3);
    }

    #[test]
    fn parse_empty_bitcoin_map() {
        let json = r#"{"bitcoin":{}}"#;
        let response: CoinGeckoResponse = serde_json::from_str(json).unwrap();
        assert!(response.bitcoin.is_empty());
    }

    #[test]
    fn parse_invalid_json_fails() {
        let result: Result<CoinGeckoResponse, _> = serde_json::from_str("not json");
        assert!(result.is_err());
    }

    #[test]
    fn api_error_display() {
        let err = ApiError::ParseError("bad json".into());
        assert_eq!(err.to_string(), "Failed to parse response: bad json");

        let err = ApiError::InvalidResponse("HTTP 403".into());
        assert_eq!(err.to_string(), "Invalid response: HTTP 403");
    }
}
