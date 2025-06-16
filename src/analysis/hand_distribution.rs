use std::ops::Add;

use crate::cards::hand::{Hand, HandEvaluation};

#[derive(Debug, PartialEq, Eq)]
pub struct HandDistribution {
    pub straight_flushes: usize,
    pub four_of_a_kinds: usize,
    pub full_houses: usize,
    pub flushes: usize,
    pub straights: usize,
    pub three_of_a_kinds: usize,
    pub two_pairs: usize,
    pub pairs: usize,
    pub high_cards: usize,
}

impl HandDistribution {
    pub fn new() -> Self {
        Self {
            straight_flushes: 0,
            four_of_a_kinds: 0,
            full_houses: 0,
            flushes: 0,
            straights: 0,
            three_of_a_kinds: 0,
            two_pairs: 0,
            pairs: 0,
            high_cards: 0,
        }
    }

    pub fn evaluate(hands: &[Hand]) -> Self {
        let mut dist = Self::new();

        for hand in hands {
            match HandEvaluation::evaluate(hand) {
                HandEvaluation::StraightFlush { highest_rank: _ } => dist.straight_flushes += 1,
                HandEvaluation::FourOfAKind { rank: _, kicker: _ } => dist.four_of_a_kinds += 1,
                HandEvaluation::FullHouse { triple: _, pair: _ } => dist.full_houses += 1,
                HandEvaluation::Flush {
                    ranks_sorted_desc: _,
                } => dist.flushes += 1,
                HandEvaluation::Straight { highest_rank: _ } => dist.straights += 1,
                HandEvaluation::ThreeOfAKind {
                    rank: _,
                    kickers_sorted_desc: _,
                } => dist.three_of_a_kinds += 1,
                HandEvaluation::TwoPair {
                    higher_rank: _,
                    lower_rank: _,
                    kicker: _,
                } => dist.two_pairs += 1,
                HandEvaluation::Pair {
                    rank: _,
                    kickers_sorted_desc: _,
                } => dist.pairs += 1,
                HandEvaluation::HighCard {
                    rank: _,
                    kickers_sorted_desc: _,
                } => dist.high_cards += 1,
            }
        }

        dist
    }
}

impl Add<HandDistribution> for HandDistribution {
    type Output = HandDistribution;

    fn add(self, rhs: HandDistribution) -> Self::Output {
        Self {
            straight_flushes: self.straight_flushes + rhs.straight_flushes,
            four_of_a_kinds: self.four_of_a_kinds + rhs.four_of_a_kinds,
            full_houses: self.full_houses + rhs.full_houses,
            flushes: self.flushes + rhs.flushes,
            straights: self.straights + rhs.straights,
            three_of_a_kinds: self.three_of_a_kinds + rhs.three_of_a_kinds,
            two_pairs: self.two_pairs + rhs.two_pairs,
            pairs: self.pairs + rhs.pairs,
            high_cards: self.high_cards + rhs.high_cards,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        analysis::search_space::all_seven_card_hands, cards::card::Card,
        parallelism::algorithms::divide_and_conquer,
    };

    #[test]
    fn test_hands_distribution() {
        let hands = &[
            [
                Card::TWO_CLUB,
                Card::THREE_CLUB,
                Card::FOUR_CLUB,
                Card::FIVE_CLUB,
                Card::SIX_CLUB,
                Card::KING_HEART,
                Card::JACK_DIAMOND,
            ],
            [
                Card::ACE_SPADE,
                Card::KING_SPADE,
                Card::QUEEN_SPADE,
                Card::JACK_SPADE,
                Card::TEN_SPADE,
                Card::KING_HEART,
                Card::JACK_DIAMOND,
            ],
            [
                Card::ACE_SPADE,
                Card::ACE_DIAMOND,
                Card::ACE_CLUB,
                Card::ACE_HEART,
                Card::KING_HEART,
                Card::SIX_HEART,
                Card::NINE_HEART,
            ],
            [
                Card::ACE_SPADE,
                Card::ACE_DIAMOND,
                Card::ACE_CLUB,
                Card::KING_CLUB,
                Card::KING_HEART,
                Card::SIX_HEART,
                Card::NINE_HEART,
            ],
            [
                Card::NINE_CLUB,
                Card::THREE_CLUB,
                Card::ACE_CLUB,
                Card::FIVE_CLUB,
                Card::SIX_CLUB,
                Card::KING_HEART,
                Card::JACK_DIAMOND,
            ],
            [
                Card::TWO_CLUB,
                Card::THREE_DIAMOND,
                Card::FOUR_HEART,
                Card::FIVE_SPADE,
                Card::SIX_CLUB,
                Card::KING_HEART,
                Card::JACK_DIAMOND,
            ],
            [
                Card::ACE_SPADE,
                Card::ACE_DIAMOND,
                Card::ACE_CLUB,
                Card::KING_CLUB,
                Card::EIGHT_HEART,
                Card::SIX_HEART,
                Card::NINE_HEART,
            ],
            [
                Card::ACE_SPADE,
                Card::ACE_DIAMOND,
                Card::KING_CLUB,
                Card::KING_HEART,
                Card::EIGHT_HEART,
                Card::SIX_HEART,
                Card::NINE_HEART,
            ],
            [
                Card::ACE_SPADE,
                Card::ACE_DIAMOND,
                Card::KING_CLUB,
                Card::TEN_HEART,
                Card::EIGHT_HEART,
                Card::SIX_HEART,
                Card::NINE_HEART,
            ],
            [
                Card::ACE_SPADE,
                Card::TWO_DIAMOND,
                Card::KING_CLUB,
                Card::TEN_HEART,
                Card::EIGHT_HEART,
                Card::SIX_HEART,
                Card::NINE_HEART,
            ],
        ];

        let dist = HandDistribution::evaluate(hands);

        // From https://en.wikipedia.org/wiki/Poker_probability#7-card_poker_hands
        assert_eq!(
            dist,
            HandDistribution {
                straight_flushes: 2,
                four_of_a_kinds: 1,
                full_houses: 1,
                flushes: 1,
                straights: 1,
                three_of_a_kinds: 1,
                two_pairs: 1,
                pairs: 1,
                high_cards: 1
            }
        );
    }

    #[test]
    #[ignore = "This test is computationally intensive. Run it with `cargo test -- --include-ignored`"]
    fn test_all_cards_distribution() {
        let hands = all_seven_card_hands();

        let count_hand_types = |range| HandDistribution::evaluate(&hands.as_slice()[range]);

        let counts = divide_and_conquer(0..hands.len(), count_hand_types)
            .into_iter()
            .reduce(|a, b| a + b)
            .unwrap();

        // From https://en.wikipedia.org/wiki/Poker_probability#7-card_poker_hands
        assert_eq!(
            counts,
            HandDistribution {
                straight_flushes: 41_584,
                four_of_a_kinds: 224_848,
                full_houses: 3_473_184,
                flushes: 4_047_644,
                straights: 6_180_020,
                three_of_a_kinds: 6_461_620,
                two_pairs: 31_433_400,
                pairs: 58_627_800,
                high_cards: 23_294_460
            }
        );
        assert_eq!(hands.len(), 133_784_560);
    }
}
