use sdk::models::Card;

use crate::util::sort_rarity::sort_rarity;

pub fn rarest_card(cards: &Vec<Card>) -> Option<Card> {
    let mut cards_sorted = cards.clone();
    cards_sorted.sort_by(|a, b| {
        sort_rarity(
            (&a.rarity, &a.rarity_modifier),
            (&b.rarity, &b.rarity_modifier),
        )
    });

    cards.last().cloned()
}
