use reqwest::{Client, header};
use serde_json::Value;
use tokio::time::{sleep, Duration};
use crate::config::{API_DELAY_MS, USER_AGENT, ACCEPT};
use crate::model::Card;

/// Build a reqwest client with Scryfall-required headers
fn build_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, USER_AGENT.parse().unwrap());
    headers.insert(header::ACCEPT, ACCEPT.parse().unwrap());

    Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

/// Fetch the URI of the Scryfall default_cards bulk dataset
pub async fn get_bulk_uri() -> Result<String, Box<dyn std::error::Error>> {
    eprintln!("Fetching Scryfall bulk metadata...");

    let client = build_client();
    let resp = client.get("https://api.scryfall.com/bulk-data").send().await;

    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            eprintln!("HTTP request to /bulk-data failed: {}", e);
            return Err(Box::new(e));
        }
    };

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        eprintln!("Scryfall API returned HTTP {}: {}", status, text);
        return Err(format!("HTTP error {}", status).into());
    }

    let json: Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Failed to parse JSON from /bulk-data: {}", e);
            return Err(Box::new(e));
        }
    };

    let data = match json.get("data").and_then(|v| v.as_array()) {
        Some(d) => d,
        None => {
            eprintln!("Missing 'data' field in bulk-data response: {}", json);
            return Err("Missing 'data' field".into());
        }
    };

    for entry in data {
        if entry.get("type").and_then(|t| t.as_str()) == Some("default_cards") {
            if let Some(uri) = entry.get("download_uri").and_then(|v| v.as_str()) {
                eprintln!("Found default_cards bulk dataset: {}", uri);
                return Ok(uri.to_string());
            } else {
                eprintln!("default_cards entry missing 'download_uri': {}", entry);
                return Err("Missing 'download_uri'".into());
            }
        }
    }

    eprintln!("default_cards dataset not found in response: {}", json);
    Err("default_cards dataset not found".into())
}

/// Download the bulk dataset and return as Vec<Card>
/// Handles errors gracefully and prints them to stderr
pub async fn download_dataset() -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    eprintln!("Downloading Scryfall dataset...");

    let uri = match get_bulk_uri().await {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Could not fetch bulk URI: {}", e);
            return Err(e);
        }
    };

    // Respect the 75 ms delay before fetching the large dataset
    sleep(Duration::from_millis(API_DELAY_MS)).await;

    let client = build_client();
    let resp = client.get(&uri).send().await;

    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            eprintln!("HTTP request to bulk dataset failed: {}", e);
            return Err(Box::new(e));
        }
    };

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        eprintln!("Bulk dataset API returned HTTP {}: {}", status, text);
        return Err(format!("HTTP error {}", status).into());
    }

    let cards: Vec<Card> = match resp.json().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse JSON from bulk dataset: {}", e);
            return Err(Box::new(e));
        }
    };

    eprintln!("Successfully downloaded {} cards", cards.len());
    Ok(cards)
}
