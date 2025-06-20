use std::{
    fmt::Display,
    ops::{BitOr, Sub},
};

use crate::{
    cards::card::{ALL_CARDS, Card, card_index},
    util::ui::format_comma_separated_values,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CardSet {
    bitset: u64,
}

impl CardSet {
    pub fn new() -> Self {
        Self { bitset: 0 }
    }

    pub fn universe() -> Self {
        Self {
            bitset: 0x0FFFFFFFFFFFFF,
        }
    }

    pub fn add(&mut self, card: Card) {
        self.bitset |= (1 as u64) << card_index(card);
    }

    pub fn add_all(&mut self, other: CardSet) {
        self.bitset |= other.bitset
    }

    pub fn has(&self, card: Card) -> bool {
        self.bitset & ((1 as u64) << card_index(card)) != 0
    }

    pub fn iter_desc(&self) -> impl Iterator<Item = Card> {
        CardSetIterator {
            bitset: self.bitset << (64 - 52),
            shifted: 64 - 52,
        }
    }

    pub fn len(&self) -> usize {
        self.bitset.count_ones() as usize
    }

    pub fn remove(&mut self, card: Card) {
        self.bitset &= !((1 as u64) << card_index(card));
    }

    pub fn remove_all(&mut self, other: CardSet) {
        self.bitset &= !other.bitset
    }
}

impl BitOr<CardSet> for CardSet {
    type Output = CardSet;

    fn bitor(mut self, rhs: CardSet) -> Self::Output {
        self.add_all(rhs);
        self
    }
}

impl Sub<CardSet> for CardSet {
    type Output = CardSet;

    fn sub(mut self, rhs: CardSet) -> Self::Output {
        self.remove_all(rhs);
        self
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

impl<const LENGTH: usize> From<&[Card; LENGTH]> for CardSet {
    fn from(value: &[Card; LENGTH]) -> Self {
        value.into_iter().map(|c| *c).collect::<CardSet>()
    }
}

impl From<&[Card]> for CardSet {
    fn from(value: &[Card]) -> Self {
        value.into_iter().map(|c| *c).collect::<CardSet>()
    }
}

impl Display for CardSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_comma_separated_values(self.iter_desc(), f, |v, fmt| v.fmt(fmt))
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

    #[test]
    fn test_card_set_add_all() {
        let mut set = vec![Card::ACE_SPADE, Card::SIX_HEART, Card::NINE_HEART]
            .into_iter()
            .collect::<CardSet>();

        let other = vec![
            Card::ACE_SPADE,
            Card::KING_SPADE,
            Card::QUEEN_SPADE,
            Card::NINE_HEART,
        ]
        .into_iter()
        .collect::<CardSet>();

        set.add_all(other);

        let contents = set.iter_desc().collect::<Vec<Card>>();
        assert_eq!(
            contents,
            vec![
                Card::ACE_SPADE,
                Card::KING_SPADE,
                Card::QUEEN_SPADE,
                Card::NINE_HEART,
                Card::SIX_HEART
            ]
        );
    }

    #[test]
    fn test_card_set_remove_all() {
        let mut set = vec![Card::ACE_SPADE, Card::SIX_HEART, Card::NINE_HEART]
            .into_iter()
            .collect::<CardSet>();

        let other = vec![
            Card::ACE_SPADE,
            Card::KING_SPADE,
            Card::QUEEN_SPADE,
            Card::NINE_HEART,
        ]
        .into_iter()
        .collect::<CardSet>();

        set.remove_all(other);

        let contents = set.iter_desc().collect::<Vec<Card>>();
        assert_eq!(contents, vec![Card::SIX_HEART]);
    }
}
