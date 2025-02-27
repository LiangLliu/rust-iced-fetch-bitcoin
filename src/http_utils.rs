use futures::stream::{FuturesUnordered, StreamExt};
use reqwest;
use reqwest::{Client, Error};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

pub async fn download_svgs_to_memory(
    codes: Vec<String>,
    flags: Vec<String>,
) -> HashMap<String, Vec<u8>> {
    let mut tasks = FuturesUnordered::new();
    let client = reqwest::Client::new();
    println!("download svg .....");
    for (code, flag) in codes.into_iter().zip(flags.into_iter()) {
        let client = client.clone();
        tasks.push(tokio::spawn(async move {
            let data = download_svg_to_memory(client, &flag).await;
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
                eprintln!("Failed to download SVG: {:?}", e);
            }
            Err(join_err) => {
                eprintln!("Task failed to execute: {:?}", join_err);
            }
        }
    }
    results
}

async fn download_svg_to_memory(client: reqwest::Client, url: &str) -> Result<Vec<u8>, Error> {
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

/// Downloads multiple SVG files from the provided URLs and saves them into the specified download directory.
///
/// # Arguments
///
/// * `urls` - A vector containing the SVG file URLs as strings.
/// * `download_dir` - A string slice that holds the path to the download directory.
///
/// # Errors
///
/// Returns an error if directory creation or any file operation fails.
#[allow(dead_code)]
pub async fn download_files(
    urls: Vec<String>,
    download_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the download directory exists; if not, create it.
    if !Path::new(download_dir).exists() {
        fs::create_dir_all(download_dir).await?;
    }

    let client = reqwest::Client::new();
    let mut tasks = FuturesUnordered::new();

    for url in urls {
        let client = client.clone();
        let download_dir = download_dir.to_string();
        tasks.push(tokio::spawn(async move {
            // Send GET request to download the file
            download_file(url.clone(), client, download_dir).await;
        }));
    }

    // Await all download tasks to finish.
    while let Some(_) = tasks.next().await {}

    Ok(())
}

#[allow(dead_code)]
async fn download_file(url: String, client: Client, download_dir: String) {
    match client.get(&url).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.bytes().await {
                    Ok(bytes) => {
                        // Extract filename from URL (using the last segment)
                        let segments: Vec<&str> = url.split('/').collect();
                        let filename = segments.last().unwrap_or(&"download.svg");
                        // Construct the full file path
                        let filepath = format!("{}/{}", download_dir, filename);

                        // Create the file asynchronously and write the downloaded bytes
                        match File::create(&filepath).await {
                            Ok(mut file) => {
                                if let Err(e) = file.write_all(&bytes).await {
                                    eprintln!("Failed to write file {}: {}", filepath, e);
                                } else {
                                    println!(
                                        "Successfully downloaded and saved file: {}",
                                        filepath
                                    );
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to create file {}: {}", filepath, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get response body for {}: {}", url, e);
                    }
                }
            } else {
                eprintln!("Failed to download {}. Status code: {}", url, resp.status());
            }
        }
        Err(e) => {
            eprintln!("Error requesting {}: {}", url, e);
        }
    }
}
