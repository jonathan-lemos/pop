use std::{
    fmt::Display,
    ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Sub, SubAssign},
};

use crate::{
    cards::card::{ALL_CARDS, Card, card_index},
    util::ui::format_separated_values,
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
            bitset: 0x000FFFFFFFFFFFFF,
        }
    }

    pub fn add(&mut self, card: Card) {
        self.bitset |= (1 as u64) << card_index(card);
    }

    pub fn disjoint_with(&self, other: CardSet) -> bool {
        self.bitset & other.bitset == 0
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
}

impl Add<Card> for CardSet {
    type Output = CardSet;

    fn add(mut self, rhs: Card) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Card> for CardSet {
    fn add_assign(&mut self, rhs: Card) {
        self.bitset |= (1 as u64) << card_index(rhs);
    }
}

impl BitAnd<CardSet> for CardSet {
    type Output = CardSet;

    fn bitand(self, rhs: CardSet) -> Self::Output {
        Self {
            bitset: self.bitset & rhs.bitset,
        }
    }
}

impl BitAndAssign<CardSet> for CardSet {
    fn bitand_assign(&mut self, rhs: CardSet) {
        self.bitset &= rhs.bitset
    }
}

impl BitOr<CardSet> for CardSet {
    type Output = CardSet;

    fn bitor(mut self, rhs: CardSet) -> Self::Output {
        self |= rhs;
        self
    }
}

impl BitOrAssign<CardSet> for CardSet {
    fn bitor_assign(&mut self, rhs: CardSet) {
        self.bitset |= rhs.bitset
    }
}

impl Sub<Card> for CardSet {
    type Output = CardSet;

    fn sub(mut self, rhs: Card) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<Card> for CardSet {
    fn sub_assign(&mut self, rhs: Card) {
        self.bitset &= !((1 as u64) << card_index(rhs));
    }
}

impl Sub<CardSet> for CardSet {
    type Output = CardSet;

    fn sub(mut self, rhs: CardSet) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<CardSet> for CardSet {
    fn sub_assign(&mut self, rhs: CardSet) {
        self.bitset &= !rhs.bitset
    }
}

impl FromIterator<Card> for CardSet {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut set = CardSet::new();
        for card in iter {
            set += card;
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
        format_separated_values(self.iter_desc(), "", f, |v, fmt| v.fmt(fmt))
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

    fn set_vec(set: CardSet) -> Vec<Card> {
        set.iter_desc().collect::<Vec<Card>>()
    }

    #[test]
    fn test_cardset_add() {
        let mut set = CardSet::new();

        set += Card::ACE_SPADE;
        set += Card::SIX_HEART;
        set += Card::NINE_HEART;
        set = set + Card::NINE_HEART;

        assert_eq!(
            set_vec(set),
            vec![Card::ACE_SPADE, Card::NINE_HEART, Card::SIX_HEART]
        )
    }

    #[test]
    fn test_cardset_collect() {
        let set = CardSet::from(&[
            Card::ACE_SPADE,
            Card::SIX_HEART,
            Card::NINE_HEART,
            Card::NINE_HEART,
        ]);

        assert_eq!(
            set_vec(set),
            vec![Card::ACE_SPADE, Card::NINE_HEART, Card::SIX_HEART]
        )
    }

    #[test]
    fn test_cardset_remove() {
        let mut set = CardSet::from(&[Card::ACE_SPADE, Card::SIX_HEART, Card::NINE_HEART]);

        set -= Card::NINE_HEART;
        set -= Card::TWO_DIAMOND;
        set = set - Card::ACE_SPADE;

        assert_eq!(set_vec(set), vec![Card::SIX_HEART]);
    }

    #[test]
    fn test_cardset_has() {
        let set = CardSet::from(&[Card::KING_CLUB, Card::SEVEN_DIAMOND]);

        assert!(set.has(Card::KING_CLUB));
        assert!(set.has(Card::SEVEN_DIAMOND));
        assert!(!set.has(Card::ACE_SPADE));
        assert!(!set.has(Card::SIX_HEART));
    }

    #[test]
    fn test_cardset_iter_desc() {
        let set = CardSet::from(&[Card::ACE_SPADE, Card::SIX_HEART, Card::NINE_HEART]);

        let iterated = set.iter_desc().collect::<Vec<Card>>();

        assert_eq!(
            iterated,
            [Card::ACE_SPADE, Card::NINE_HEART, Card::SIX_HEART]
        );
    }

    #[test]
    fn test_cardset_universe() {
        let expected = ALL_CARDS.into_iter().collect::<CardSet>();
        assert_eq!(CardSet::universe(), expected);
    }

    #[test]
    fn test_cardset_add_all() {
        let mut set = CardSet::from(&[Card::ACE_SPADE, Card::SIX_HEART, Card::NINE_HEART]);

        let other = CardSet::from(&[
            Card::ACE_SPADE,
            Card::KING_SPADE,
            Card::QUEEN_SPADE,
            Card::NINE_HEART,
        ]);

        let set_clone = set | other;
        set |= other;

        assert_eq!(
            set_vec(set),
            vec![
                Card::ACE_SPADE,
                Card::KING_SPADE,
                Card::QUEEN_SPADE,
                Card::NINE_HEART,
                Card::SIX_HEART
            ]
        );
        assert_eq!(set, set_clone);
    }

    #[test]
    fn test_cardset_remove_all() {
        let mut set = CardSet::from(&[Card::ACE_SPADE, Card::SIX_HEART, Card::NINE_HEART]);

        let other = CardSet::from(&[
            Card::ACE_SPADE,
            Card::KING_SPADE,
            Card::QUEEN_SPADE,
            Card::NINE_HEART,
        ]);

        let set_clone = set - other;
        set -= other;

        assert_eq!(set_vec(set), vec![Card::SIX_HEART]);
        assert_eq!(set, set_clone);
    }

    #[test]
    fn test_cardset_union() {
        let set1 = CardSet::from(&[Card::ACE_SPADE, Card::KING_SPADE, Card::QUEEN_SPADE]);
        let set2 = CardSet::from(&[
            Card::KING_SPADE,
            Card::QUEEN_SPADE,
            Card::TEN_SPADE,
            Card::NINE_DIAMOND,
        ]);

        let expected = CardSet::from(&[
            Card::ACE_SPADE,
            Card::KING_SPADE,
            Card::QUEEN_SPADE,
            Card::TEN_SPADE,
            Card::NINE_DIAMOND,
        ]);

        assert_eq!(set1 | set2, expected);
    }

    #[test]
    fn test_cardset_intersection() {
        let set1 = CardSet::from(&[Card::ACE_SPADE, Card::KING_SPADE, Card::QUEEN_SPADE]);
        let set2 = CardSet::from(&[
            Card::KING_SPADE,
            Card::QUEEN_SPADE,
            Card::TEN_SPADE,
            Card::NINE_DIAMOND,
        ]);

        let expected = CardSet::from(&[Card::QUEEN_SPADE, Card::KING_SPADE]);

        assert_eq!(set1 & set2, expected);
    }

    #[test]
    fn test_disjoint_with() {
        let set1 = CardSet::from(&[Card::ACE_SPADE, Card::KING_SPADE]);
        let set2 = CardSet::from(&[Card::KING_SPADE, Card::QUEEN_SPADE, Card::TEN_SPADE]);
        let set3 = CardSet::from(&[
            Card::NINE_DIAMOND,
            Card::TEN_SPADE,
            Card::JACK_SPADE,
            Card::QUEEN_SPADE,
        ]);

        assert!(!set1.disjoint_with(set1));
        assert!(!set1.disjoint_with(set2));
        assert!(!set2.disjoint_with(set1));
        assert!(!set2.disjoint_with(set2));
        assert!(!set2.disjoint_with(set3));
        assert!(!set3.disjoint_with(set2));
        assert!(!set3.disjoint_with(set3));

        assert!(set1.disjoint_with(set3));
        assert!(set3.disjoint_with(set1));
    }
}
