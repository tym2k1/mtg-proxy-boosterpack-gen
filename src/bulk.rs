use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::time::{sleep, Duration};
use std::fs;

const CACHE_FILE: &str = "scryfall.cache";
pub const API_DELAY_MS: u64 = 75;

// Scryfall requires a User-Agent string
const USER_AGENT: &str = "tym2k1/mtg-proxy-boosterpack-gen (tymbur@gmail.com)";
const ACCEPT: &str = "application/json";

use crate::model::{Card, SetInfo};

#[derive(Default, Serialize, Deserialize)]
pub struct ApiCache {
    pub cards: Option<Vec<Card>>,
    pub sets: Option<Vec<SetInfo>>,
}

fn build_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, USER_AGENT.parse().unwrap());
    headers.insert(header::ACCEPT, ACCEPT.parse().unwrap());

    Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub fn load_cache() -> ApiCache {
    fs::read_to_string(CACHE_FILE)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_cache(cache: &ApiCache) {
    let json = serde_json::to_string(cache).unwrap();
    fs::write(CACHE_FILE, json).unwrap();
}

async fn fetch_json<T: DeserializeOwned>(
    client: &Client,
    url: &str,
    delay: bool,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {

    if delay {
        sleep(Duration::from_millis(API_DELAY_MS)).await;
    }

    let resp = client.get(url).send().await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, text).into());
    }

    let text = resp.text().await?;

    // Try parsing direct array first
    if let Ok(v) = serde_json::from_str::<Vec<T>>(&text) {
        return Ok(v);
    }

    // Otherwise parse `{ data: [...] }`
    let value: Value = serde_json::from_str(&text)?;

    let data = value.get("data")
        .ok_or("Missing data field")?;

    let parsed: Vec<T> = serde_json::from_value(data.clone())?;

    Ok(parsed)
}

pub async fn fetch_cards() -> Result<Vec<Card>, Box<dyn std::error::Error>> {

    let mut cache = load_cache();



    if let Some(cards) = cache.cards {
        eprintln!("Loaded cards from cache");
        return Ok(cards);
    }


    eprintln!("Fetching Scryfall bulk metadata...");

    let client = build_client();

    let bulk: Value = client
        .get("https://api.scryfall.com/bulk-data")
        .send()
        .await?
        .json()
        .await?;

    let uri = bulk["data"]
        .as_array()
        .unwrap()
        .iter()
        .find(|x| x["type"] == "default_cards")
        .unwrap()["download_uri"]
        .as_str()
        .unwrap()
        .to_string();

    eprintln!("Downloading Scryfall dataset...");

    let cards: Vec<Card> = fetch_json(&client, &uri, true).await?;

    eprintln!("Downloaded {} cards", cards.len());

    cache.cards = Some(cards.clone());
    save_cache(&cache);

    Ok(cards)
}

pub async fn fetch_sets(overwrite: bool) -> Result<Vec<SetInfo>, Box<dyn std::error::Error>> {

    let mut cache = load_cache();

    if !overwrite { //check if we want to find new sets
        if let Some(sets) = cache.sets {
            eprintln!("Loaded sets from cache");
            return Ok(sets);
        }
    }

    eprintln!("Fetching MTG set list...");

    let client = build_client();

    let sets: Vec<SetInfo> =
        fetch_json(&client, "https://api.scryfall.com/sets", false).await?;

    eprintln!("Found {} MTG sets", sets.len());

    cache.sets = Some(sets.clone());
    save_cache(&cache);

    Ok(sets)
}
