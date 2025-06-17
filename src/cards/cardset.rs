use std::fmt::Display;

use crate::{
    cards::card::{ALL_CARDS, Card, Rank, Suit, card_index},
    util::ui::format_comma_separated_values,
};

#[derive(Debug, PartialEq, Eq)]
pub struct CardSet {
    bitset: u64,
}

impl CardSet {
    pub fn new() -> Self {
        Self { bitset: 0 }
    }

    pub fn add(&mut self, card: Card) {
        self.bitset |= (1 as u64) << card_index(card);
    }

    pub fn has(&self, card: Card) -> bool {
        self.bitset & ((1 as u64) << card_index(card)) != 0
    }

    pub fn iter_desc(&self) -> impl Iterator<Item = Card> {
        CardSetIterator {
            bitset: self.bitset,
            shifted: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.bitset.count_ones() as usize
    }

    pub fn remove(&mut self, card: Card) {
        self.bitset &= !((1 as u64) << card_index(card));
    }
}

impl FromIterator<Card> for CardSet {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut set = CardSet::new();
        for card in iter {
            set.add(card);
        }
        set
    }
}

impl Display for CardSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        format_comma_separated_values(self.iter_desc(), f, |v, fmt| v.fmt(fmt))?;
        f.write_str("}")
    }
}

pub struct CardSetIterator {
    bitset: u64,
    shifted: usize,
}

impl Iterator for CardSetIterator {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bitset == 0 {
            return None;
        }

        while self.bitset & ((1 as u64) << 63) == 0 {
            self.bitset <<= 1;
            self.shifted += 1;
        }

        let card = ALL_CARDS[63 - self.shifted];

        self.bitset <<= 1;
        self.shifted += 1;

        Some(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_set_add() {
        let mut set = CardSet::new();

        set.add(Card::ACE_SPADE);
        set.add(Card::SIX_HEART);
        set.add(Card::NINE_HEART);
        set.add(Card::NINE_HEART);

        assert!(set.has(Card::ACE_SPADE));
        assert!(set.has(Card::SIX_HEART));
        assert!(set.has(Card::NINE_HEART));
        assert!(!set.has(Card::TWO_DIAMOND));
    }

    #[test]
    fn test_card_set_collect() {
        let set = vec![
            Card::ACE_SPADE,
            Card::SIX_HEART,
            Card::NINE_HEART,
            Card::NINE_HEART,
        ]
        .into_iter()
        .collect::<CardSet>();

        assert!(set.has(Card::ACE_SPADE));
        assert!(set.has(Card::SIX_HEART));
        assert!(set.has(Card::NINE_HEART));
        assert!(!set.has(Card::TWO_DIAMOND));
    }

    #[test]
    fn test_card_set_remove() {
        let mut set = vec![Card::ACE_SPADE, Card::SIX_HEART, Card::NINE_HEART]
            .into_iter()
            .collect::<CardSet>();

        set.remove(Card::NINE_HEART);
        set.remove(Card::TWO_DIAMOND);

        assert!(set.has(Card::ACE_SPADE));
        assert!(set.has(Card::SIX_HEART));
        assert!(!set.has(Card::NINE_HEART));
        assert!(!set.has(Card::TWO_DIAMOND));
    }

    #[test]
    fn test_card_set_iter_desc() {
        let set = vec![Card::ACE_SPADE, Card::SIX_HEART, Card::NINE_HEART]
            .into_iter()
            .collect::<CardSet>();

        let iterated = set.iter_desc().collect::<Vec<Card>>();

        assert_eq!(
            iterated,
            [Card::ACE_SPADE, Card::NINE_HEART, Card::SIX_HEART]
        );
    }

    #[test]
    fn test_card_set_iter_desc_full() {
        let set = ALL_CARDS.into_iter().collect::<CardSet>();

        let iterated = set.iter_desc().collect::<Vec<Card>>();

        assert_eq!(iterated, ALL_CARDS.into_iter().rev().collect::<Vec<Card>>());
    }
}
