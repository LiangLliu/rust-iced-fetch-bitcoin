use std::sync::LazyLock;
use std::time::Duration;

/// Shared HTTP client with connection pooling and proper User-Agent.
/// Created once on first access and reused across all requests.
pub static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .user_agent(format!(
            "iced-fetch-bitcoin/{}",
            env!("CARGO_PKG_VERSION")
        ))
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client")
});
