use std::usize;

use crate::cards::{
    card::{Card, NUM_SUITS, Suit},
    hand::HAND_EVALUATION_SIZE,
};

// Groups cards by suit for hand evaluation.
//
// The implementation here avoids heap allocations.
pub struct SuitGrouping {
    groups: [[Card; HAND_EVALUATION_SIZE]; NUM_SUITS],
    lengths: [usize; NUM_SUITS],
}

impl SuitGrouping {
    pub fn new() -> Self {
        let arbitrary_card = Card::TWO_CLUB;
        Self {
            groups: [[arbitrary_card; HAND_EVALUATION_SIZE]; NUM_SUITS],
            lengths: [0; NUM_SUITS],
        }
    }

    // Panics if there are already HAND_EVALUATION_SIZE cards for the suit.
    pub fn insert(&mut self, card: Card) {
        self.groups[card.suit as usize][self.lengths[card.suit as usize]] = card;
        self.lengths[card.suit as usize] += 1;
    }

    pub fn get(&self, suit: Suit) -> &[Card] {
        return &self.groups[suit as usize][0..self.lengths[suit as usize]];
    }

    // Empties each suit grouping.
    pub fn reset(&mut self) {
        for length in &mut self.lengths {
            *length = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::suit_grouping::*;

    #[test]
    fn test_suit_grouping() {
        let mut grouping = SuitGrouping::new();

        grouping.insert(Card::KING_CLUB);
        grouping.insert(Card::TWO_CLUB);
        grouping.insert(Card::THREE_CLUB);

        grouping.insert(Card::ACE_SPADE);

        assert_eq!(
            grouping.get(Suit::Club),
            &[Card::KING_CLUB, Card::TWO_CLUB, Card::THREE_CLUB]
        );
        assert_eq!(grouping.get(Suit::Heart), &[]);
        assert_eq!(grouping.get(Suit::Spade), &[Card::ACE_SPADE]);
    }

    #[test]
    fn test_reset() {
        let mut grouping = SuitGrouping::new();

        grouping.insert(Card::KING_CLUB);
        grouping.insert(Card::TWO_CLUB);
        grouping.insert(Card::ACE_SPADE);

        grouping.reset();

        grouping.insert(Card::KING_CLUB);
        grouping.insert(Card::THREE_CLUB);
        grouping.insert(Card::THREE_HEART);

        assert_eq!(
            grouping.get(Suit::Club),
            &[Card::KING_CLUB, Card::THREE_CLUB]
        );
        assert_eq!(grouping.get(Suit::Heart), &[Card::THREE_HEART]);
        assert_eq!(grouping.get(Suit::Spade), &[]);
    }
}
