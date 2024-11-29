pub fn sort_rarity(
    (a_rarity, a_rarity_modifier): (&str, &Option<String>),
    (b_rarity, b_rarity_modifier): (&str, &Option<String>),
) -> std::cmp::Ordering {
    let a_r = match a_rarity_modifier {
        Some(rm) => rm,
        None => a_rarity,
    };

    let b_r = match b_rarity_modifier {
        Some(rm) => rm,
        None => b_rarity,
    };

    a_r.cmp(b_r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    #[derive(Debug, PartialEq, Eq)]
    struct DummyCard {
        id: i32,
        rarity: String,
        rarity_modifier: Option<String>,
    }

    static CARD_3: LazyLock<DummyCard> = LazyLock::new(|| DummyCard {
        id: 3,
        rarity: "3".to_string(),
        rarity_modifier: None,
    });

    static CARD_4: LazyLock<DummyCard> = LazyLock::new(|| DummyCard {
        id: 4,
        rarity: "4".to_string(),
        rarity_modifier: None,
    });

    static CARD_5: LazyLock<DummyCard> = LazyLock::new(|| DummyCard {
        id: 5,
        rarity: "5".to_string(),
        rarity_modifier: None,
    });

    static CARD_6: LazyLock<DummyCard> = LazyLock::new(|| DummyCard {
        id: 6,
        rarity: "6".to_string(),
        rarity_modifier: Some("6-1".to_string()),
    });

    static CARD_6S: LazyLock<DummyCard> = LazyLock::new(|| DummyCard {
        id: 16,
        rarity: "6".to_string(),
        rarity_modifier: Some("6-2".to_string()),
    });

    static CARD_7: LazyLock<DummyCard> = LazyLock::new(|| DummyCard {
        id: 7,
        rarity: "7".to_string(),
        rarity_modifier: None,
    });

    #[test]
    fn sorts_cards() {
        let mut cards = vec![
            &*CARD_6S,
            &*CARD_4,
            &*CARD_6,
            &*CARD_5,
            &*CARD_7,
            &*CARD_3,
        ];

        cards.sort_by(|a, b| {
            sort_rarity((&a.rarity, &a.rarity_modifier), (&b.rarity, &b.rarity_modifier))
        });

        let expected_sort = vec![
            &*CARD_3,
            &*CARD_4,
            &*CARD_5,
            &*CARD_6,
            &*CARD_6S,
            &*CARD_7,
        ];

        assert_eq!(cards, expected_sort);
    }
}