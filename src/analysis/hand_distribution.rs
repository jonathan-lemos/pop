use std::ops::Add;

use crate::analysis::evaluate_hand::HandEvaluation;
use crate::cards::cardset::CardSet;
use crate::parallelism::algorithms::{into_parallel_reduce, parallel_map};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
    pub discarded_hands: usize,
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
            discarded_hands: 0,
        }
    }

    pub fn evaluate(hands: &[CardSet]) -> Self {
        if hands.is_empty() {
            return HandDistribution::new();
        }
        let distributions = parallel_map(hands, |h| HandDistribution::from(*h));
        into_parallel_reduce(distributions, |a, b| a + b).unwrap()
    }

    pub fn is_complete(&self) -> bool {
        self.discarded_hands == 0
    }

    pub fn total_num_hands(&self) -> usize {
        self.straight_flushes
            + self.four_of_a_kinds
            + self.full_houses
            + self.flushes
            + self.straights
            + self.three_of_a_kinds
            + self.two_pairs
            + self.pairs
            + self.high_cards
            + self.discarded_hands
    }

    pub fn straight_flush_percentage(&self) -> f64 {
        (self.straight_flushes * 100) as f64 / self.total_num_hands() as f64
    }

    pub fn four_of_a_kind_percentage(&self) -> f64 {
        (self.four_of_a_kinds * 100) as f64 / self.total_num_hands() as f64
    }

    pub fn full_house_percentage(&self) -> f64 {
        (self.full_houses * 100) as f64 / self.total_num_hands() as f64
    }

    pub fn flush_percentage(&self) -> f64 {
        (self.flushes * 100) as f64 / self.total_num_hands() as f64
    }

    pub fn straight_percentage(&self) -> f64 {
        (self.straights * 100) as f64 / self.total_num_hands() as f64
    }

    pub fn three_of_a_kind_percentage(&self) -> f64 {
        (self.three_of_a_kinds * 100) as f64 / self.total_num_hands() as f64
    }

    pub fn two_pair_percentage(&self) -> f64 {
        (self.two_pairs * 100) as f64 / self.total_num_hands() as f64
    }

    pub fn pair_percentage(&self) -> f64 {
        (self.pairs * 100) as f64 / self.total_num_hands() as f64
    }

    pub fn high_card_percentage(&self) -> f64 {
        (self.high_cards * 100) as f64 / self.total_num_hands() as f64
    }
}

impl From<CardSet> for HandDistribution {
    fn from(value: CardSet) -> Self {
        let mut dist = Self::new();
        match HandEvaluation::evaluate_postflop(value) {
            None => {
                panic!("{} failed to evaluate", value);
                dist.discarded_hands += 1;
            }
            Some(HandEvaluation::StraightFlush { highest_rank: _ }) => dist.straight_flushes += 1,
            Some(HandEvaluation::FourOfAKind { rank: _, kicker: _ }) => dist.four_of_a_kinds += 1,
            Some(HandEvaluation::FullHouse { triple: _, pair: _ }) => dist.full_houses += 1,
            Some(HandEvaluation::Flush {
                ranks_sorted_desc: _,
            }) => dist.flushes += 1,
            Some(HandEvaluation::Straight { highest_rank: _ }) => dist.straights += 1,
            Some(HandEvaluation::ThreeOfAKind {
                rank: _,
                kickers_sorted_desc: _,
            }) => dist.three_of_a_kinds += 1,
            Some(HandEvaluation::TwoPair {
                higher_rank: _,
                lower_rank: _,
                kicker: _,
            }) => dist.two_pairs += 1,
            Some(HandEvaluation::Pair {
                rank: _,
                kickers_sorted_desc: _,
            }) => dist.pairs += 1,
            Some(HandEvaluation::HighCard {
                rank: _,
                kickers_sorted_desc: _,
            }) => dist.high_cards += 1,
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
            discarded_hands: self.discarded_hands + rhs.discarded_hands,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{analysis::search_space::all_seven_card_hands, cards::card::Card};

    #[test]
    fn test_hands_distribution() {
        let hands = [
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
        ]
        .iter()
        .map(CardSet::from)
        .collect::<Vec<CardSet>>();

        let dist = HandDistribution::evaluate(hands.as_slice());

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
                high_cards: 1,
                discarded_hands: 0
            }
        );
    }

    #[test]
    #[ignore = "This test is computationally intensive. Run it with `cargo test -- --include-ignored`"]
    fn test_all_cards_distribution() {
        let hands = all_seven_card_hands();

        // From https://en.wikipedia.org/wiki/Poker_probability#7-card_poker_hands
        assert_eq!(
            HandDistribution::evaluate(hands.as_slice()),
            HandDistribution {
                straight_flushes: 41_584,
                four_of_a_kinds: 224_848,
                full_houses: 3_473_184,
                flushes: 4_047_644,
                straights: 6_180_020,
                three_of_a_kinds: 6_461_620,
                two_pairs: 31_433_400,
                pairs: 58_627_800,
                high_cards: 23_294_460,
                discarded_hands: 0,
            }
        );
        assert_eq!(hands.len(), 133_784_560);
    }
}
