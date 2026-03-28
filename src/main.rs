mod model;
mod bulk;
mod pool;
mod booster;

use bulk::*;
use pool::*;
use booster::*;
use model::*;

use std::env;


/// Find a set by code
pub fn find_set<'a>(sets: &'a [SetInfo], code: &str) -> Option<&'a SetInfo> {
    sets.iter().find(|s| s.code == code)
}

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <set_code> <num_packs>", args[0]);
        eprintln!("Example: {} blb 6", args[0]);
        return;
    }

    let set_code = &args[1];
    let pack_count: usize = args[2].parse().unwrap_or(1);

    // Fetch datasets
    let cards = match fetch_cards().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not fetch Scryfall dataset: {}", e);
            return;
        }
    };

    let sets = match fetch_sets().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not fetch set list: {}", e);
            return;
        }
    };

    // Resolve requested set
    let set = match find_set(&sets, set_code) {
        Some(s) => s,
        None => {
            eprintln!("Unknown set code: {}", set_code);
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

    // Generate packs
    println!("Generating {} booster packs...\n", pack_count);

    for i in 1..=pack_count {

        let pack = generate_valid_pack(pool);

        println!("=== Booster Pack {} ===", i);

        for card in pack {
            println!("1x {}", card.name);
        }

        println!();
    }
}
