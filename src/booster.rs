use rand::seq::SliceRandom;
use rand::Rng;

use crate::pool::CardPool;
use crate::model::Card;

pub fn generate_pack(pool: &CardPool) -> Vec<Card> {

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
