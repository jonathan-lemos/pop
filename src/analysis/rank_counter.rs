use crate::cards::card::{NUM_RANKS, Rank};

// Counts the number of times a rank appears.
//
// This implementation avoids heap allocations.
#[derive(Debug, Clone, Copy)]
pub struct RankCounter {
    counts: [u8; NUM_RANKS],
}

impl RankCounter {
    pub fn new() -> Self {
        Self {
            counts: [0; NUM_RANKS],
        }
    }

    // Panics if the count for this rank exceeds 255.
    pub fn inc(&mut self, rank: Rank) {
        self.counts[rank as usize] += 1
    }

    pub fn get(&self, rank: Rank) -> u8 {
        self.counts[rank as usize]
    }

    // Sets all rank counts back to 0.
    pub fn reset(&mut self) {
        for count in &mut self.counts {
            *count = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_counter() {
        let mut counter = RankCounter::new();

        counter.inc(Rank::Two);
        counter.inc(Rank::Two);
        counter.inc(Rank::Two);
        counter.inc(Rank::Ace);

        assert_eq!(counter.get(Rank::Two), 3);
        assert_eq!(counter.get(Rank::Ace), 1);
        assert_eq!(counter.get(Rank::King), 0);
    }

    #[test]
    fn test_reset() {
        let mut counter = RankCounter::new();

        counter.inc(Rank::Two);
        counter.inc(Rank::Two);
        counter.inc(Rank::Ace);

        counter.reset();

        counter.inc(Rank::Two);
        counter.inc(Rank::Three);
        counter.inc(Rank::Ace);
        counter.inc(Rank::Ace);

        assert_eq!(counter.get(Rank::Two), 1);
        assert_eq!(counter.get(Rank::Three), 1);
        assert_eq!(counter.get(Rank::Ace), 2);
    }
}
