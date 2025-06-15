use crate::cards::card::{ALL_RANKS, Card, NUM_RANKS, Rank};

// Given a list of cards of HAND_EVALUATION_SIZE, lists how often each rank appears.
//
// This implementation avoids heap allocations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RankCounts {
    pub four: Option<Rank>,
    pub three: Option<Rank>,
    pub pairs: [Rank; 2],
    pub pairs_len: u8,
    pub kickers: [Rank; 5],
    pub kickers_len: u8,
}

impl RankCounts {
    // `cards` must be sorted in descending order.
    pub fn make(cards: &[Card; 7]) -> Self {
        let mut counts = [0 as usize; NUM_RANKS];
        for card in cards {
            counts[card.rank as usize] += 1;
        }

        let arbitrary_rank = Rank::Two;
        let mut counter = Self {
            four: None,
            three: None,
            pairs: [arbitrary_rank; 2],
            pairs_len: 0,
            kickers: [arbitrary_rank; 5],
            kickers_len: 0,
        };

        for (rank, count) in ALL_RANKS.iter().zip(counts.iter()) {
            match *count {
                4 => counter.four = Some(*rank),
                3 => counter.three = Some(*rank),
                2 => {
                    if counter.pairs_len < 2 {
                        counter.pairs[counter.pairs_len as usize] = *rank;
                        counter.pairs_len += 1;
                    }
                }
                1 => {
                    if counter.kickers_len < 5 {
                        counter.pairs[counter.kickers_len as usize] = *rank;
                        counter.kickers_len += 1;
                    }
                }
                0 => continue,
                _ => panic!(
                    "The cardinality of each card must be bounded by [0, 4] (was {})",
                    *count
                ),
            }
        }

        return counter;
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::rank_counter::*;

    #[test]
    fn test_rank_counter_quads_and_triple() {
        let counter = RankCounts::make(&[
            Card::SIX_CLUB,
            Card::SIX_DIAMOND,
            Card::SIX_HEART,
            Card::SIX_SPADE,
            Card::THREE_CLUB,
            Card::THREE_HEART,
            Card::THREE_DIAMOND,
        ]);

        assert_eq!(counter.four, Some(Rank::Six));
        assert_eq!(counter.three, Some(Rank::Three));
        assert_eq!(counter.pairs_len, 0);
        assert_eq!(counter.kickers_len, 0);
    }
}
