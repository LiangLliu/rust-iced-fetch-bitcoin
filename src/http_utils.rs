use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashMap;

use crate::http_client::CLIENT;

/// Downloads multiple SVG flag images concurrently and returns them as in-memory byte vectors.
///
/// # Arguments
/// * `codes` - Country codes used as keys in the result map
/// * `flags` - Corresponding SVG URLs to download
pub async fn download_svgs_to_memory(
    codes: Vec<String>,
    flags: Vec<String>,
) -> HashMap<String, Vec<u8>> {
    let client = CLIENT.clone();
    let mut tasks = FuturesUnordered::new();

    for (code, flag) in codes.into_iter().zip(flags.into_iter()) {
        let client = client.clone();
        tasks.push(tokio::spawn(async move {
            let data = download_svg(&client, &flag).await;
            (code, data)
        }));
    }

    let mut results = HashMap::new();
    while let Some(task_result) = tasks.next().await {
        match task_result {
            Ok((code, Ok(svg_data))) => {
                results.insert(code, svg_data);
            }
            Ok((_code, Err(e))) => {
                tracing::warn!("Failed to download SVG: {e}");
            }
            Err(e) => {
                tracing::warn!("SVG download task panicked: {e}");
            }
        }
    }
    results
}

async fn download_svg(client: &reqwest::Client, url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let bytes = client.get(url).send().await?.bytes().await?;
    Ok(bytes.to_vec())
}
