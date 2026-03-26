use std::fs;
use std::time::{SystemTime, Duration};

use crate::model::Card;
use crate::config::{CACHE_FILE, CACHE_MAX_AGE_HOURS};

pub fn cache_is_valid() -> bool {

    if let Ok(metadata) = fs::metadata(CACHE_FILE) {

        if let Ok(modified) = metadata.modified() {

            let age = SystemTime::now()
                .duration_since(modified)
                .unwrap();

            return age < Duration::from_secs(CACHE_MAX_AGE_HOURS * 3600);
        }
    }

    false
}

pub fn load_cache() -> Vec<Card> {

    let data = fs::read_to_string(CACHE_FILE).unwrap();

    serde_json::from_str(&data).unwrap()
}

pub fn save_cache(cards: &Vec<Card>) {

    let json = serde_json::to_string(cards).unwrap();

    fs::write(CACHE_FILE, json).unwrap();
}
