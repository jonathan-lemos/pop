use std::usize;

use crate::{
    analysis::evaluate_hand::MAX_HAND_SIZE,
    cards::card::{Card, NUM_SUITS, Rank, Suit},
    datastructures::stack_vec::StackVec,
};

// Groups cards by suit for hand evaluation.
//
// The implementation here avoids heap allocations.
#[derive(Clone, Copy)]
pub struct SuitGrouping {
    groups: [StackVec<Rank, MAX_HAND_SIZE>; NUM_SUITS],
}

impl SuitGrouping {
    pub fn new() -> Self {
        Self {
            groups: [StackVec::new(); NUM_SUITS],
        }
    }

    // No-op if there are already 7 cards for the suit.
    pub fn insert(&mut self, card: Card) {
        self.groups[card.suit as usize].push(card.rank)
    }

    pub fn get(&self, suit: Suit) -> &[Rank] {
        return self.groups[suit as usize].as_slice();
    }

    // Empties each suit grouping.
    pub fn reset(&mut self) {
        for group in &mut self.groups {
            group.reset();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suit_grouping() {
        let mut grouping = SuitGrouping::new();

        grouping.insert(Card::KING_CLUB);
        grouping.insert(Card::TWO_CLUB);
        grouping.insert(Card::THREE_CLUB);

        grouping.insert(Card::ACE_SPADE);

        assert_eq!(
            grouping.get(Suit::Club),
            &[Rank::King, Rank::Two, Rank::Three]
        );
        assert_eq!(grouping.get(Suit::Heart), &[]);
        assert_eq!(grouping.get(Suit::Spade), &[Rank::Ace]);
    }

    #[test]
    fn test_suit_grouping_filled_up() {
        let mut grouping = SuitGrouping::new();

        grouping.insert(Card::TWO_CLUB);
        grouping.insert(Card::THREE_CLUB);
        grouping.insert(Card::FOUR_CLUB);
        grouping.insert(Card::FIVE_CLUB);
        grouping.insert(Card::SIX_CLUB);
        grouping.insert(Card::SEVEN_CLUB);
        grouping.insert(Card::EIGHT_CLUB);
        grouping.insert(Card::NINE_CLUB);

        assert_eq!(
            grouping.get(Suit::Club),
            &[
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight
            ]
        );
    }

    #[test]
    fn test_reset() {
        let mut grouping = SuitGrouping::new();

        grouping.insert(Card::KING_CLUB);
        grouping.insert(Card::TWO_SPADE);
        grouping.insert(Card::ACE_SPADE);

        grouping.reset();

        grouping.insert(Card::KING_CLUB);
        grouping.insert(Card::THREE_HEART);
        grouping.insert(Card::THREE_CLUB);

        assert_eq!(grouping.get(Suit::Club), &[Rank::King, Rank::Three]);
        assert_eq!(grouping.get(Suit::Heart), &[Rank::Three]);
        assert_eq!(grouping.get(Suit::Spade), &[]);
    }
}
