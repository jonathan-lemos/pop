use self::HandEvaluation::*;
use std::{cmp::Reverse, fmt::Display};

use crate::{
    cards::{
        card::{ALL_RANKS, ALL_SUITS, Card, Rank},
        cardset::CardSet,
        rank_counter::RankCounter,
        suit_grouping::SuitGrouping,
    },
    datastructures::stack_vec::StackVec,
    util::ui::format_comma_separated_values,
};

pub const HAND_SIZE: usize = 7;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Hand {
    cardset: CardSet,
}

impl Hand {
    pub fn new(cards: &[Card; HAND_SIZE]) -> Self {
        let cardset = cards.into_iter().map(|x| *x).collect::<CardSet>();
        debug_assert!(cardset.len() == HAND_SIZE);
        Self { cardset }
    }

    pub fn iter_desc(&self) -> impl Iterator<Item = Card> {
        self.cardset.iter_desc()
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_comma_separated_values(self.iter_desc(), f, |v, fmt| v.fmt(fmt))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum HandEvaluation {
    HighCard {
        rank: Rank,
        kickers_sorted_desc: [Rank; 4],
    },
    Pair {
        rank: Rank,
        kickers_sorted_desc: [Rank; 3],
    },
    TwoPair {
        higher_rank: Rank,
        lower_rank: Rank,
        kicker: Rank,
    },
    ThreeOfAKind {
        rank: Rank,
        kickers_sorted_desc: [Rank; 2],
    },
    Straight {
        highest_rank: Rank,
    },
    Flush {
        ranks_sorted_desc: [Rank; 5],
    },
    FullHouse {
        triple: Rank,
        pair: Rank,
    },
    FourOfAKind {
        rank: Rank,
        kicker: Rank,
    },
    StraightFlush {
        highest_rank: Rank,
    },
}

fn count_ranks(cards: &Hand) -> RankCounter {
    let mut counter = RankCounter::new();
    for card in cards.iter_desc() {
        counter.inc(card.rank);
    }
    return counter;
}

fn group_by_suit(cards: &Hand) -> SuitGrouping {
    let mut groupings = SuitGrouping::new();
    for card in cards.iter_desc() {
        groupings.insert(card);
    }
    return groupings;
}

// `ranks` must be sorted in descending order
fn straight_high_rank<I: Iterator<Item = Rank>>(mut ranks: I) -> Option<Rank> {
    let first = match ranks.next() {
        Some(f) => f,
        None => return None,
    };

    let mut high_rank = first;
    let mut straight_len = 1;
    let mut last_rank = first;

    while let Some(rank) = ranks.next() {
        if rank == last_rank {
            continue;
        }
        last_rank = rank;

        if high_rank > rank && (high_rank as usize) - (rank as usize) == straight_len {
            straight_len += 1;
            if straight_len == 5 {
                break;
            }
        } else {
            high_rank = rank;
            straight_len = 1;
        }
    }

    if straight_len == 5 || (straight_len == 4 && high_rank == Rank::Five && first == Rank::Ace) {
        Some(high_rank)
    } else {
        None
    }
}

struct Cardinalities {
    pub four: Option<Rank>,
    pub trips: StackVec<Rank, 2>,
    pub pairs: StackVec<Rank, 3>,
    pub kickers: StackVec<Rank, 5>,
}

impl Cardinalities {
    fn new(cards: &Hand) -> Self {
        let by_rank = count_ranks(cards);

        let mut cardinalities = Self {
            four: None,
            trips: StackVec::new(),
            pairs: StackVec::new(),
            kickers: StackVec::new(),
        };

        for rank in ALL_RANKS.iter().rev() {
            match by_rank.get(*rank) {
                4 => cardinalities.four = Some(*rank),
                3 => cardinalities.trips.push(*rank),
                2 => cardinalities.pairs.push(*rank),
                1 => cardinalities.kickers.push(*rank),
                0 => {}
                _ => panic!(
                    "The number of times that a rank appears must be between 0 and 4 (was {})",
                    by_rank.get(*rank)
                ),
            }
        }

        cardinalities
    }
}

// Each grouping must be sorted in descending order.
fn match_straight_flush(by_suit: &SuitGrouping) -> Option<HandEvaluation> {
    for suit in ALL_SUITS {
        if let Some(highest_rank) = straight_high_rank(by_suit.get(suit).into_iter().map(|x| *x)) {
            return Some(StraightFlush { highest_rank });
        }
    }
    return None;
}

fn match_four_of_a_kind(cardinalities: &Cardinalities) -> Option<HandEvaluation> {
    if let Some(rank) = cardinalities.four {
        let mut kickers = StackVec::<Rank, 3>::new();
        if !cardinalities.trips.is_empty() {
            kickers.push(cardinalities.trips[0]);
        }
        if !cardinalities.pairs.is_empty() {
            kickers.push(cardinalities.pairs[0]);
        }
        if !cardinalities.kickers.is_empty() {
            kickers.push(cardinalities.kickers[0]);
        }

        return Some(FourOfAKind {
            rank: rank,
            kicker: *kickers.as_slice().iter().max().unwrap(),
        });
    }
    return None;
}

fn match_full_house(cardinalities: &Cardinalities) -> Option<HandEvaluation> {
    if cardinalities.trips.len() >= 2 {
        Some(FullHouse {
            triple: cardinalities.trips[0],
            pair: cardinalities.trips[1],
        })
    } else if !cardinalities.trips.is_empty() && !cardinalities.pairs.is_empty() {
        Some(FullHouse {
            triple: cardinalities.trips[0],
            pair: cardinalities.pairs[0],
        })
    } else {
        None
    }
}

fn match_flush(by_suit: &SuitGrouping) -> Option<HandEvaluation> {
    for suit in ALL_SUITS {
        let ranks = by_suit.get(suit);
        if ranks.len() >= 5 {
            return Some(Flush {
                ranks_sorted_desc: [ranks[0], ranks[1], ranks[2], ranks[3], ranks[4]],
            });
        }
    }
    return None;
}

fn match_straight(cards: &Hand) -> Option<HandEvaluation> {
    if let Some(highest_rank) = straight_high_rank(cards.iter_desc().map(|x| x.rank)) {
        Some(Straight { highest_rank })
    } else {
        None
    }
}

fn match_trips(cardinalities: &Cardinalities) -> Option<HandEvaluation> {
    if cardinalities.trips.is_empty() {
        None
    } else {
        let mut kickers = StackVec::<Rank, 4>::new();
        if cardinalities.trips.len() >= 2 {
            kickers.push(cardinalities.trips[1]);
            kickers.push(cardinalities.trips[1]);
        }
        for kicker in cardinalities.kickers.as_slice().iter().take(2) {
            kickers.push(*kicker);
        }
        kickers.as_mut_slice().sort_unstable_by_key(|c| Reverse(*c));

        Some(ThreeOfAKind {
            rank: cardinalities.trips[0],
            kickers_sorted_desc: [kickers[0], kickers[1]],
        })
    }
}

fn match_two_pair(cardinalities: &Cardinalities) -> Option<HandEvaluation> {
    if cardinalities.pairs.len() < 2 {
        None
    } else {
        let mut kickers = StackVec::<Rank, 2>::new();
        if cardinalities.pairs.len() >= 3 {
            kickers.push(cardinalities.pairs[2]);
        }
        if cardinalities.kickers.len() >= 1 {
            kickers.push(cardinalities.kickers[0]);
        }

        Some(TwoPair {
            higher_rank: cardinalities.pairs[0],
            lower_rank: cardinalities.pairs[1],
            kicker: *kickers.as_slice().iter().max().unwrap(),
        })
    }
}

fn match_pair(cardinalities: &Cardinalities) -> Option<HandEvaluation> {
    if cardinalities.pairs.is_empty() {
        None
    } else {
        Some(Pair {
            rank: cardinalities.pairs[0],
            kickers_sorted_desc: [
                cardinalities.kickers[0],
                cardinalities.kickers[1],
                cardinalities.kickers[2],
            ],
        })
    }
}

fn match_high_card(cardinalities: &Cardinalities) -> HandEvaluation {
    HighCard {
        rank: cardinalities.kickers[0],
        kickers_sorted_desc: [
            cardinalities.kickers[1],
            cardinalities.kickers[2],
            cardinalities.kickers[3],
            cardinalities.kickers[4],
        ],
    }
}

impl HandEvaluation {
    pub fn evaluate(hand: &Hand) -> Self {
        let by_suit = group_by_suit(hand);
        let cardinalities = Cardinalities::new(hand);

        match_straight_flush(&by_suit)
            .or_else(|| match_four_of_a_kind(&cardinalities))
            .or_else(|| match_full_house(&cardinalities))
            .or_else(|| match_flush(&by_suit))
            .or_else(|| match_straight(hand))
            .or_else(|| match_trips(&cardinalities))
            .or_else(|| match_two_pair(&cardinalities))
            .or_else(|| match_pair(&cardinalities))
            .unwrap_or_else(|| match_high_card(&cardinalities))
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::hand::*;

    fn test_eval(arr: [Card; 7]) -> HandEvaluation {
        HandEvaluation::evaluate(&Hand::new(&arr))
    }

    #[test]
    fn test_evaluate_hand_straight_flush_normal() {
        let hand = test_eval([
            Card::EIGHT_SPADE,
            Card::NINE_SPADE,
            Card::SEVEN_SPADE,
            Card::JACK_DIAMOND,
            Card::TEN_SPADE,
            Card::SIX_SPADE,
            Card::ACE_HEART,
        ]);

        assert_eq!(
            hand,
            StraightFlush {
                highest_rank: Rank::Ten
            }
        );
    }

    #[test]
    fn test_evaluate_hand_straight_flush_six_matching() {
        let hand = test_eval([
            Card::EIGHT_SPADE,
            Card::NINE_SPADE,
            Card::SEVEN_SPADE,
            Card::JACK_SPADE,
            Card::TEN_SPADE,
            Card::SIX_SPADE,
            Card::ACE_SPADE,
        ]);

        assert_eq!(
            hand,
            StraightFlush {
                highest_rank: Rank::Jack
            }
        );
    }

    #[test]
    fn test_evaluate_hand_straight_flush_seven_matching() {
        let hand = test_eval([
            Card::EIGHT_SPADE,
            Card::NINE_SPADE,
            Card::SEVEN_SPADE,
            Card::JACK_SPADE,
            Card::TEN_SPADE,
            Card::SIX_SPADE,
            Card::FIVE_SPADE,
        ]);

        assert_eq!(
            hand,
            StraightFlush {
                highest_rank: Rank::Jack
            }
        );
    }

    #[test]
    fn test_evaluate_hand_straight_flush_hearts() {
        let hand = test_eval([
            Card::EIGHT_HEART,
            Card::NINE_HEART,
            Card::SEVEN_HEART,
            Card::JACK_HEART,
            Card::TEN_HEART,
            Card::SIX_HEART,
            Card::FIVE_HEART,
        ]);

        assert_eq!(
            hand,
            StraightFlush {
                highest_rank: Rank::Jack
            }
        );
    }

    #[test]
    fn test_evaluate_hand_straight_flush_beats_trips() {
        let hand = test_eval([
            Card::EIGHT_SPADE,
            Card::NINE_SPADE,
            Card::SEVEN_SPADE,
            Card::EIGHT_DIAMOND,
            Card::TEN_SPADE,
            Card::SIX_SPADE,
            Card::EIGHT_HEART,
        ]);

        assert_eq!(
            hand,
            StraightFlush {
                highest_rank: Rank::Ten
            }
        );
    }

    #[test]
    fn test_evaluate_hand_straight_flush_a2345() {
        let hand = test_eval([
            Card::ACE_SPADE,
            Card::TWO_SPADE,
            Card::THREE_SPADE,
            Card::FOUR_SPADE,
            Card::FIVE_SPADE,
            Card::NINE_HEART,
            Card::EIGHT_HEART,
        ]);

        assert_eq!(
            hand,
            StraightFlush {
                highest_rank: Rank::Five
            }
        );
    }

    #[test]
    fn test_evaluate_hand_four_of_a_kind_normal() {
        let hand = test_eval([
            Card::TEN_HEART,
            Card::TEN_SPADE,
            Card::SEVEN_HEART,
            Card::JACK_HEART,
            Card::TEN_CLUB,
            Card::TEN_DIAMOND,
            Card::ACE_HEART,
        ]);

        assert_eq!(
            hand,
            FourOfAKind {
                rank: Rank::Ten,
                kicker: Rank::Ace
            }
        );
    }

    #[test]
    fn test_evaluate_hand_four_of_a_kind_with_pair_and_higher_kicker() {
        let hand = test_eval([
            Card::TEN_HEART,
            Card::TEN_SPADE,
            Card::TEN_CLUB,
            Card::TEN_DIAMOND,
            Card::SEVEN_HEART,
            Card::SEVEN_CLUB,
            Card::ACE_HEART,
        ]);

        assert_eq!(
            hand,
            FourOfAKind {
                rank: Rank::Ten,
                kicker: Rank::Ace
            }
        );
    }

    #[test]
    fn test_evaluate_hand_four_of_a_kind_with_pair_and_lower_kicker() {
        let hand = test_eval([
            Card::TEN_HEART,
            Card::TEN_SPADE,
            Card::TEN_CLUB,
            Card::TEN_DIAMOND,
            Card::SEVEN_HEART,
            Card::SEVEN_CLUB,
            Card::FIVE_HEART,
        ]);

        assert_eq!(
            hand,
            FourOfAKind {
                rank: Rank::Ten,
                kicker: Rank::Seven
            }
        );
    }

    #[test]
    fn test_evaluate_hand_four_of_a_kind_with_pair_and_trips() {
        let hand = test_eval([
            Card::TEN_HEART,
            Card::TEN_SPADE,
            Card::TEN_CLUB,
            Card::TEN_DIAMOND,
            Card::SEVEN_HEART,
            Card::SEVEN_CLUB,
            Card::SEVEN_SPADE,
        ]);

        assert_eq!(
            hand,
            FourOfAKind {
                rank: Rank::Ten,
                kicker: Rank::Seven
            }
        );
    }

    #[test]
    fn test_evaluate_hand_boat_normal() {
        let hand = test_eval([
            Card::TEN_HEART,
            Card::TEN_SPADE,
            Card::TEN_CLUB,
            Card::SEVEN_HEART,
            Card::SEVEN_CLUB,
            Card::KING_SPADE,
            Card::QUEEN_SPADE,
        ]);

        assert_eq!(
            hand,
            FullHouse {
                triple: Rank::Ten,
                pair: Rank::Seven
            }
        );
    }

    #[test]
    fn test_evaluate_hand_boat_two_pairs() {
        let hand = test_eval([
            Card::TEN_HEART,
            Card::TEN_SPADE,
            Card::TEN_CLUB,
            Card::SEVEN_HEART,
            Card::SEVEN_CLUB,
            Card::KING_SPADE,
            Card::KING_HEART,
        ]);

        assert_eq!(
            hand,
            FullHouse {
                triple: Rank::Ten,
                pair: Rank::King
            }
        );
    }

    #[test]
    fn test_evaluate_hand_boat_two_trips() {
        let hand = test_eval([
            Card::TEN_HEART,
            Card::TEN_SPADE,
            Card::TEN_CLUB,
            Card::SEVEN_HEART,
            Card::SEVEN_CLUB,
            Card::SEVEN_SPADE,
            Card::KING_HEART,
        ]);

        assert_eq!(
            hand,
            FullHouse {
                triple: Rank::Ten,
                pair: Rank::Seven
            }
        );
    }

    #[test]
    fn test_evaluate_hand_flush_normal() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::NINE_CLUB,
            Card::KING_CLUB,
            Card::TWO_CLUB,
            Card::ACE_SPADE,
            Card::TEN_HEART,
            Card::FOUR_CLUB,
        ]);

        assert_eq!(
            hand,
            Flush {
                ranks_sorted_desc: [Rank::King, Rank::Nine, Rank::Eight, Rank::Four, Rank::Two],
            }
        );
    }

    #[test]
    fn test_evaluate_hand_flush_six_matching() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::NINE_CLUB,
            Card::KING_CLUB,
            Card::TWO_CLUB,
            Card::ACE_CLUB,
            Card::TEN_HEART,
            Card::FOUR_CLUB,
        ]);

        assert_eq!(
            hand,
            Flush {
                ranks_sorted_desc: [Rank::Ace, Rank::King, Rank::Nine, Rank::Eight, Rank::Four],
            }
        );
    }

    #[test]
    fn test_evaluate_hand_flush_seven_matching() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::NINE_CLUB,
            Card::KING_CLUB,
            Card::TWO_CLUB,
            Card::ACE_CLUB,
            Card::TEN_CLUB,
            Card::FOUR_CLUB,
        ]);

        assert_eq!(
            hand,
            Flush {
                ranks_sorted_desc: [Rank::Ace, Rank::King, Rank::Ten, Rank::Nine, Rank::Eight],
            }
        );
    }

    #[test]
    fn test_evaluate_hand_flush_beats_straight() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::NINE_CLUB,
            Card::KING_CLUB,
            Card::SEVEN_HEART,
            Card::TEN_CLUB,
            Card::JACK_DIAMOND,
            Card::FOUR_CLUB,
        ]);

        assert_eq!(
            hand,
            Flush {
                ranks_sorted_desc: [Rank::King, Rank::Ten, Rank::Nine, Rank::Eight, Rank::Four],
            }
        );
    }

    #[test]
    fn test_evaluate_hand_flush_beats_trips() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::NINE_CLUB,
            Card::KING_CLUB,
            Card::EIGHT_HEART,
            Card::TEN_CLUB,
            Card::EIGHT_SPADE,
            Card::FOUR_CLUB,
        ]);

        assert_eq!(
            hand,
            Flush {
                ranks_sorted_desc: [Rank::King, Rank::Ten, Rank::Nine, Rank::Eight, Rank::Four],
            }
        );
    }

    #[test]
    fn test_evaluate_hand_flush_beats_two_pair() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::NINE_CLUB,
            Card::KING_CLUB,
            Card::EIGHT_HEART,
            Card::TEN_CLUB,
            Card::NINE_SPADE,
            Card::FOUR_CLUB,
        ]);

        assert_eq!(
            hand,
            Flush {
                ranks_sorted_desc: [Rank::King, Rank::Ten, Rank::Nine, Rank::Eight, Rank::Four],
            }
        );
    }

    #[test]
    fn test_evaluate_hand_straight_normal() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::NINE_CLUB,
            Card::SEVEN_DIAMOND,
            Card::TEN_HEART,
            Card::ACE_SPADE,
            Card::TWO_CLUB,
            Card::JACK_CLUB,
        ]);

        assert_eq!(
            hand,
            Straight {
                highest_rank: Rank::Jack,
            }
        );
    }

    #[test]
    fn test_evaluate_hand_straight_six() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::NINE_CLUB,
            Card::SEVEN_DIAMOND,
            Card::TEN_HEART,
            Card::ACE_SPADE,
            Card::SIX_CLUB,
            Card::JACK_CLUB,
        ]);

        assert_eq!(
            hand,
            Straight {
                highest_rank: Rank::Jack,
            }
        );
    }

    #[test]
    fn test_evaluate_hand_straight_seven() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::NINE_CLUB,
            Card::SEVEN_DIAMOND,
            Card::TEN_HEART,
            Card::QUEEN_SPADE,
            Card::SIX_CLUB,
            Card::JACK_CLUB,
        ]);

        assert_eq!(
            hand,
            Straight {
                highest_rank: Rank::Queen,
            }
        );
    }

    #[test]
    fn test_evaluate_hand_straight_beats_trips() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::NINE_CLUB,
            Card::SEVEN_DIAMOND,
            Card::TEN_HEART,
            Card::EIGHT_SPADE,
            Card::EIGHT_DIAMOND,
            Card::JACK_CLUB,
        ]);

        assert_eq!(
            hand,
            Straight {
                highest_rank: Rank::Jack,
            }
        );
    }

    #[test]
    fn test_evaluate_hand_straight_beats_two_pair() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::NINE_CLUB,
            Card::SEVEN_DIAMOND,
            Card::TEN_HEART,
            Card::EIGHT_SPADE,
            Card::NINE_DIAMOND,
            Card::JACK_CLUB,
        ]);

        assert_eq!(
            hand,
            Straight {
                highest_rank: Rank::Jack,
            }
        );
    }

    #[test]
    fn test_evaluate_hand_straight_a2345() {
        let hand = test_eval([
            Card::ACE_SPADE,
            Card::TWO_HEART,
            Card::THREE_CLUB,
            Card::FOUR_DIAMOND,
            Card::FIVE_SPADE,
            Card::NINE_HEART,
            Card::EIGHT_HEART,
        ]);

        assert_eq!(
            hand,
            Straight {
                highest_rank: Rank::Five
            }
        );
    }

    #[test]
    fn test_evaluate_hand_trips_normal() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::EIGHT_SPADE,
            Card::SEVEN_DIAMOND,
            Card::TWO_HEART,
            Card::EIGHT_HEART,
            Card::FOUR_DIAMOND,
            Card::JACK_CLUB,
        ]);

        assert_eq!(
            hand,
            ThreeOfAKind {
                rank: Rank::Eight,
                kickers_sorted_desc: [Rank::Jack, Rank::Seven]
            }
        );
    }

    #[test]
    fn test_evaluate_hand_two_pair_normal() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::EIGHT_SPADE,
            Card::JACK_DIAMOND,
            Card::JACK_HEART,
            Card::FIVE_HEART,
            Card::KING_CLUB,
            Card::ACE_CLUB,
        ]);

        assert_eq!(
            hand,
            TwoPair {
                higher_rank: Rank::Jack,
                lower_rank: Rank::Eight,
                kicker: Rank::Ace,
            }
        );
    }

    #[test]
    fn test_evaluate_hand_two_pair_three_pairs_higher_kicker() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::EIGHT_SPADE,
            Card::JACK_DIAMOND,
            Card::JACK_HEART,
            Card::FIVE_HEART,
            Card::FIVE_CLUB,
            Card::ACE_CLUB,
        ]);

        assert_eq!(
            hand,
            TwoPair {
                higher_rank: Rank::Jack,
                lower_rank: Rank::Eight,
                kicker: Rank::Ace,
            }
        );
    }

    #[test]
    fn test_evaluate_hand_two_pair_three_pairs_lower_kicker() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::EIGHT_SPADE,
            Card::JACK_DIAMOND,
            Card::JACK_HEART,
            Card::FIVE_HEART,
            Card::FIVE_CLUB,
            Card::TWO_CLUB,
        ]);

        assert_eq!(
            hand,
            TwoPair {
                higher_rank: Rank::Jack,
                lower_rank: Rank::Eight,
                kicker: Rank::Five,
            }
        );
    }

    #[test]
    fn test_evaluate_hand_two_pair_all_kickers_lower() {
        let hand = test_eval([
            Card::EIGHT_CLUB,
            Card::EIGHT_SPADE,
            Card::JACK_DIAMOND,
            Card::JACK_HEART,
            Card::FIVE_HEART,
            Card::FOUR_CLUB,
            Card::TWO_CLUB,
        ]);

        assert_eq!(
            hand,
            TwoPair {
                higher_rank: Rank::Jack,
                lower_rank: Rank::Eight,
                kicker: Rank::Five,
            }
        );
    }

    #[test]
    fn test_evaluate_hand_two_pair_not_pocket_pair() {
        let hand = test_eval([
            Card::JACK_CLUB,
            Card::EIGHT_SPADE,
            Card::JACK_DIAMOND,
            Card::EIGHT_HEART,
            Card::FIVE_HEART,
            Card::FOUR_CLUB,
            Card::TWO_CLUB,
        ]);

        assert_eq!(
            hand,
            TwoPair {
                higher_rank: Rank::Jack,
                lower_rank: Rank::Eight,
                kicker: Rank::Five,
            }
        );
    }

    #[test]
    fn test_evaluate_hand_pair_normal() {
        let hand = test_eval([
            Card::JACK_CLUB,
            Card::EIGHT_SPADE,
            Card::JACK_DIAMOND,
            Card::ACE_HEART,
            Card::FIVE_HEART,
            Card::FOUR_CLUB,
            Card::TWO_CLUB,
        ]);

        assert_eq!(
            hand,
            Pair {
                rank: Rank::Jack,
                kickers_sorted_desc: [Rank::Ace, Rank::Eight, Rank::Five],
            }
        );
    }

    #[test]
    fn test_evaluate_hand_pair_higher_kickers() {
        let hand = test_eval([
            Card::FIVE_CLUB,
            Card::EIGHT_SPADE,
            Card::FIVE_DIAMOND,
            Card::ACE_HEART,
            Card::KING_HEART,
            Card::QUEEN_CLUB,
            Card::JACK_CLUB,
        ]);

        assert_eq!(
            hand,
            Pair {
                rank: Rank::Five,
                kickers_sorted_desc: [Rank::Ace, Rank::King, Rank::Queen],
            }
        );
    }

    #[test]
    fn test_evaluate_hand_pair_lower_kickers() {
        let hand = test_eval([
            Card::ACE_CLUB,
            Card::ACE_SPADE,
            Card::QUEEN_CLUB,
            Card::JACK_CLUB,
            Card::TEN_DIAMOND,
            Card::FIVE_DIAMOND,
            Card::THREE_DIAMOND,
        ]);

        assert_eq!(
            hand,
            Pair {
                rank: Rank::Ace,
                kickers_sorted_desc: [Rank::Queen, Rank::Jack, Rank::Ten],
            }
        );
    }

    #[test]
    fn test_evaluate_hand_high_card() {
        let hand = test_eval([
            Card::KING_CLUB,
            Card::QUEEN_CLUB,
            Card::JACK_DIAMOND,
            Card::FIVE_HEART,
            Card::EIGHT_CLUB,
            Card::NINE_HEART,
            Card::THREE_DIAMOND,
        ]);

        assert_eq!(
            hand,
            HighCard {
                rank: Rank::King,
                kickers_sorted_desc: [Rank::Queen, Rank::Jack, Rank::Nine, Rank::Eight],
            }
        );
    }

    #[test]
    fn test_evaluate_hand_high_card_one_card_from_pocket() {
        let hand = test_eval([
            Card::KING_CLUB,
            Card::TWO_CLUB,
            Card::JACK_DIAMOND,
            Card::FIVE_HEART,
            Card::EIGHT_CLUB,
            Card::NINE_HEART,
            Card::THREE_DIAMOND,
        ]);

        assert_eq!(
            hand,
            HighCard {
                rank: Rank::King,
                kickers_sorted_desc: [Rank::Jack, Rank::Nine, Rank::Eight, Rank::Five],
            }
        );
    }

    #[test]
    fn test_evaluate_hand_high_card_nothing_from_pocket() {
        let hand = test_eval([
            Card::THREE_CLUB,
            Card::TWO_CLUB,
            Card::JACK_DIAMOND,
            Card::FIVE_HEART,
            Card::EIGHT_CLUB,
            Card::NINE_HEART,
            Card::KING_DIAMOND,
        ]);

        assert_eq!(
            hand,
            HighCard {
                rank: Rank::King,
                kickers_sorted_desc: [Rank::Jack, Rank::Nine, Rank::Eight, Rank::Five],
            }
        );
    }

    #[test]
    fn test_hand_cmp_straight_flush_vs_other() {
        let h1 = StraightFlush {
            highest_rank: Rank::Jack,
        };
        let h2 = FourOfAKind {
            rank: Rank::Ace,
            kicker: Rank::King,
        };

        assert!(h1 > h2);
    }

    #[test]
    fn test_hand_cmp_three_of_a_kind() {
        let h1 = ThreeOfAKind {
            rank: Rank::King,
            kickers_sorted_desc: [Rank::Ace, Rank::Queen],
        };
        let h2 = ThreeOfAKind {
            rank: Rank::Ace,
            kickers_sorted_desc: [Rank::King, Rank::Queen],
        };

        assert!(h1 < h2);
    }

    #[test]
    fn test_hand_cmp_three_of_a_kind_first_kicker() {
        let h1 = ThreeOfAKind {
            rank: Rank::King,
            kickers_sorted_desc: [Rank::Ace, Rank::Ten],
        };
        let h2 = ThreeOfAKind {
            rank: Rank::King,
            kickers_sorted_desc: [Rank::Queen, Rank::Ten],
        };

        assert!(h1 > h2);
    }

    #[test]
    fn test_hand_cmp_three_of_a_kind_second_kicker() {
        let h1 = ThreeOfAKind {
            rank: Rank::King,
            kickers_sorted_desc: [Rank::Ace, Rank::Jack],
        };
        let h2 = ThreeOfAKind {
            rank: Rank::King,
            kickers_sorted_desc: [Rank::Ace, Rank::Ten],
        };

        assert!(h1 > h2);
    }

    #[test]
    fn test_hand_cmp_three_of_a_kind_equal() {
        let h1 = ThreeOfAKind {
            rank: Rank::King,
            kickers_sorted_desc: [Rank::Ace, Rank::Jack],
        };
        let h2 = ThreeOfAKind {
            rank: Rank::King,
            kickers_sorted_desc: [Rank::Ace, Rank::Jack],
        };

        assert!(h1 == h2);
    }
}
