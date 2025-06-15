use self::Hand::*;
use std::cmp::{self, Ordering, Reverse};

use crate::cards::{
    card::{ALL_RANKS, ALL_SUITS, Card, NUM_RANKS, NUM_SUITS, Rank, Suit},
    suit_grouping::SuitGrouping,
};

const BOARD_SIZE: usize = 5;
const HAND_SIZE: usize = 5;
const POCKET_SIZE: usize = 2;
pub(super) const HAND_EVALUATION_SIZE: usize = BOARD_SIZE + POCKET_SIZE;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Hand {
    HighCard {
        rank: Rank,
        kickers_sorted_desc: [Rank; HAND_SIZE - 1],
    },
    Pair {
        rank: Rank,
        kickers_sorted_desc: [Rank; HAND_SIZE - 2],
    },
    TwoPair {
        higher_rank: Rank,
        lower_rank: Rank,
        kicker: Rank,
    },
    ThreeOfAKind {
        rank: Rank,
        kickers_sorted_desc: [Rank; HAND_SIZE - 3],
    },
    Straight {
        highest_rank: Rank,
    },
    Flush {
        ranks_sorted_desc: [Rank; HAND_SIZE],
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

/*
fn count_ranks(cards: &[Card; HAND_EVALUATION_SIZE], counter: &mut RankCounter) {
    for card in cards {
        counter.inc(card.rank);
    }
}
*/

fn group_by_suit(cards: &[Card; HAND_EVALUATION_SIZE], groupings: &mut SuitGrouping) {
    for card in cards {
        groupings.insert(*card);
    }
}

// `cards` must be sorted in descending order
fn straight_high_rank(cards: &[Card]) -> Option<Rank> {
    if cards.len() < 5 {
        return None;
    }

    let mut high_rank = cards[0].rank;
    let mut straight_len = 1;
    let mut idx = 1;

    while idx + (5 - straight_len) < cards.len() {
        if high_rank > cards[idx].rank
            && (high_rank as usize) - (cards[idx].rank as usize) == straight_len
        {
            straight_len += 1;
            if straight_len == 5 {
                break;
            }
        } else {
            high_rank = cards[idx].rank;
            straight_len = 1;
        }
        idx += 1;
    }

    if straight_len == 5 {
        Some(high_rank)
    } else {
        None
    }
}

fn initialize_hand_evaluation_pool(pocket: &[Card; 2], board: &[Card; 5], pool: &mut [Card; 7]) {
    pool[0] = pocket[0];
    pool[1] = pocket[1];
    pool[2] = board[0];
    pool[3] = board[1];
    pool[4] = board[2];
    pool[5] = board[3];
    pool[6] = board[4];
    pool.sort_by_key(|w| Reverse(*w));
}

pub fn evaluate_hand(pocket: &[Card; 2], board: &[Card; 5]) -> Hand {
    let arbitrary_card = Card::TWO_CLUB;
    let mut hand_evaluation_pool = [arbitrary_card; 7];
    initialize_hand_evaluation_pool(pocket, board, &mut hand_evaluation_pool);

    let mut by_suit = SuitGrouping::new();
    group_by_suit(&hand_evaluation_pool, &mut by_suit);

    for suit in ALL_SUITS {
        if let Some(highest_rank) = straight_high_rank(by_suit.get(suit)) {
            return StraightFlush { highest_rank };
        }
    }

    /*
    let mut by_rank = RankCounter::new();
    count_ranks(&hand_evaluation_pool, &mut by_rank);
    */

    todo!();
}

// pub fn compare_hands(left: &[Card; 2], right: &[Card; 2], board: &[Card; 5]) -> Ordering {}
