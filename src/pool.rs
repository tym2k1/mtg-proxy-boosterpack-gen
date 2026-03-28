use std::collections::HashMap;
use crate::model::{Card, SetInfo};

#[allow(dead_code)]
pub struct CardPool {
    pub set: SetInfo,

    pub commons: Vec<Card>,
    pub uncommons: Vec<Card>,
    pub rares: Vec<Card>,
    pub mythics: Vec<Card>,
    pub lands: Vec<Card>,
}

pub fn build_all_pools(cards: &[Card], sets: &[SetInfo]) -> HashMap<String, CardPool> {
    let mut map = HashMap::new();

    for set in sets {
        let pool = build_pool(cards, set);
        if !pool.commons.is_empty() || !pool.uncommons.is_empty() {
            map.insert(set.code.clone(), pool);
        }
    }

    map
}

pub fn build_pool(cards: &[Card], set: &SetInfo) -> CardPool {

    let mut pool = CardPool {
        set: set.clone(),

        commons: Vec::new(),
        uncommons: Vec::new(),
        rares: Vec::new(),
        mythics: Vec::new(),
        lands: Vec::new(),
    };

    for c in cards.iter().filter(|c| c.set == set.code) {

        match c.rarity.as_str() {
            "common" => pool.commons.push(c.clone()),
            "uncommon" => pool.uncommons.push(c.clone()),
            "rare" => pool.rares.push(c.clone()),
            "mythic" => pool.mythics.push(c.clone()),
            _ => {}
        }

        if c.type_line
            .as_deref()
            .map(|s| s.contains("Basic"))
            .unwrap_or(false)
        {
            pool.lands.push(c.clone());
        }
    }

    pool
}
