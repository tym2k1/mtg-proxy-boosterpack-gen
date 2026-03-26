mod model;
mod config;
mod cache;
mod bulk;
mod pool;
mod booster;

use cache::*;
use bulk::*;
use pool::*;
use booster::*;

#[tokio::main]
async fn main() {

    let cards = if cache_is_valid() {

        println!("Loading cached dataset...");
        load_cache()

    } else {

        println!("Downloading Scryfall dataset...");

        let cards = match download_dataset().await {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not fetch Scryfall dataset, aborting.");
                return;
            }
        };

        save_cache(&cards);

        cards
    };

    println!("Building card pool...");

    let pool = build_pool(cards);

    println!("Generating booster pack...\n");

    let pack = generate_pack(&pool);

    println!("=== Booster Pack ===");

    for card in pack {
        println!("{} ({})", card.name, card.rarity);
    }
}
