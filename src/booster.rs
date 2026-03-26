use rand::seq::SliceRandom;
use rand::Rng;

use crate::pool::CardPool;
use crate::model::Card;

use std::collections::{HashMap, HashSet};

fn generate_pack(pool: &CardPool) -> Vec<Card> {

    let mut rng = rand::thread_rng();

    let mut pack = Vec::new();

    pack.extend(
        pool.commons.choose_multiple(&mut rng, 10).cloned()
    );

    pack.extend(
        pool.uncommons.choose_multiple(&mut rng, 3).cloned()
    );

    if rng.gen_ratio(1,8) {

        pack.push(
            pool.mythics.choose(&mut rng).unwrap().clone()
        );

    } else {

        pack.push(
            pool.rares.choose(&mut rng).unwrap().clone()
        );
    }

    pack.push(
        pool.lands.choose(&mut rng).unwrap().clone()
    );

    pack
}

/// Returns true if the pack conforms to Reuben's rules
fn reuben_algorithm_check(pack: &[Card]) -> bool {
    let mut common_color_counts: HashMap<String, usize> = HashMap::new();
    let mut uncommon_color_counts: HashMap<String, usize> = HashMap::new();
    let mut seen_names: HashSet<String> = HashSet::new();
    let mut has_creature = false;
    let mut common_colors_present: HashSet<String> = HashSet::new();

    for card in pack {
        // Avoid repeated cards
        if !seen_names.insert(card.name.clone()) {
            return false; // duplicate card
        }

        // Count colors
        let colors = card.colors.clone().unwrap_or_default();

        match card.rarity.as_str() {
            "common" => {
                for c in &colors {
                    *common_color_counts.entry(c.clone()).or_default() += 1;
                    common_colors_present.insert(c.clone());
                }
                // Check if it's a creature
                if let Some(type_line) = &card.type_line {
                    if type_line.contains("Creature") {
                        has_creature = true;
                    }
                }
            }
            "uncommon" => {
                for c in &colors {
                    *uncommon_color_counts.entry(c.clone()).or_default() += 1;
                }
            }
            _ => {}
        }
    }

    // Rule 1: no more than 4 commons of same color
    if common_color_counts.values().any(|&count| count > 4) {
        return false;
    }

    // Rule 2: at least 1 common of each color
    let all_colors = ["W", "U", "B", "R", "G"];
    if !all_colors.iter().all(|c| common_colors_present.contains(*c)) {
        return false;
    }

    // Rule 3: at least 1 common creature
    if !has_creature {
        return false;
    }

    // Rule 4: no more than 2 uncommons of same color
    if uncommon_color_counts.values().any(|&count| count > 2) {
        return false;
    }

    // Rule 5: duplicates already checked
    true
}

pub fn generate_valid_pack(pool: &CardPool) -> Vec<Card> {
    loop {
        let pack = generate_pack(pool);
        if reuben_algorithm_check(&pack) {
            return pack;
        }
        // Optional: log retries
        eprintln!("Regenerating pack, did not meet Reuben's rules...");
    }
}
