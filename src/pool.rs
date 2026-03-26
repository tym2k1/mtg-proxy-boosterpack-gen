use crate::model::Card;

pub struct CardPool {

    pub commons: Vec<Card>,
    pub uncommons: Vec<Card>,
    pub rares: Vec<Card>,
    pub mythics: Vec<Card>,
    pub lands: Vec<Card>,
}

pub fn build_pool(cards: Vec<Card>) -> CardPool {

    let mut pool = CardPool {

        commons: Vec::new(),
        uncommons: Vec::new(),
        rares: Vec::new(),
        mythics: Vec::new(),
        lands: Vec::new(),
    };

    for c in &cards {  // borrow instead of move
        match c.rarity.as_str() {
            "common" => pool.commons.push(c.clone()),
            "uncommon" => pool.uncommons.push(c.clone()),
            "rare" => pool.rares.push(c.clone()),
            "mythic" => pool.mythics.push(c.clone()),
            _ => {}
        }

        if c.type_line.as_deref().map(|s| s.contains("Basic")).unwrap_or(false) {
            pool.lands.push(c.clone());
        }
    }

    pool
}
