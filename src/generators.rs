use crate::bulk::*;
use crate::pool::*;
use crate::booster::*;
use crate::model::*;
use crate::pdf::*;

pub fn find_set_name<'a>(sets: &'a [SetInfo], name: &str) -> Option<&'a SetInfo> {
    sets.iter().find(|s| s.name == name)
}


pub async fn generate_boosters(set_name: &str, pack_count: i32) {

    // Fetch datasets
    let cards = match fetch_cards().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not fetch Scryfall dataset: {}", e);
            return;
        }
    };

    let sets = match fetch_sets(false).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not fetch set list: {}", e);
            return;
        }
    };

    // Resolve requested set
    let set = match find_set_name(&sets, set_name) {
        Some(s) => s,
        None => {
            eprintln!("Unknown set name: {}", set_name);
            return;
        }
    };

    println!("Using set: {} ({})", set.name, set.code);

    // Build all pools (fast lookup)
    println!("Building card pools...");

    let set_pools = build_all_pools(&cards, &sets);

    let pool = match set_pools.get(&set.code) {
        Some(p) => p,
        None => {
            eprintln!("No cards found for set {}", set.code);
            return;
        }
    };

    let mut all_cards: Vec<Card> = Vec::new();

    // Generate packs
    println!("Generating {} booster packs...\n", pack_count);

    for i in 1..=pack_count {

        let pack = generate_valid_pack(pool);

        println!("=== Booster Pack {} ===", i);

        for card in &pack {
            println!("1x {}", card.name);
        }

        println!();
        all_cards.extend(pack);

    }
        println!("Generating printable PDF...");

        generate_pdf(&all_cards, "boosters.pdf").await;

        println!("Saved boosters.pdf");
}