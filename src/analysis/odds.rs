use std::ops::Range;

use crate::analysis::evaluate_hand::HandEvaluation;
use crate::analysis::hand_distribution::HandDistribution;
use crate::analysis::math::SatisfactionFraction;
use crate::analysis::search_space::combinations;
use crate::cards::cardset::CardSet;
use crate::parallelism::algorithms::{SubrangeIterator, into_parallel_reduce, parallel_map};
use crate::parallelism::os::get_parallelism_from_os;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OddsError {
    MustHaveAtLeastOnePlayer,
    BoardCannotHaveMoreThan5Cards,
    CannotHaveDuplicateCards,
    PocketsMustHaveTwoCardsEach,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OddsCalculation {
    pub pocket: CardSet,
    pub winning_chance: SatisfactionFraction,
    pub hand_distribution: HandDistribution,
}

pub fn calculate_odds(
    pockets: &[CardSet],
    board: CardSet,
) -> Result<Vec<OddsCalculation>, OddsError> {
    if pockets.is_empty() {
        return Err(OddsError::MustHaveAtLeastOnePlayer);
    }

    if pockets.iter().any(|p| p.len() != 2) {
        return Err(OddsError::PocketsMustHaveTwoCardsEach);
    }

    if board.len() > 5 {
        return Err(OddsError::BoardCannotHaveMoreThan5Cards);
    }

    let mut all_taken_cards = board;
    for pocket in pockets {
        if !all_taken_cards.disjoint_with(*pocket) {
            return Err(OddsError::CannotHaveDuplicateCards);
        }
        all_taken_cards |= *pocket;
    }

    let runouts = combinations(CardSet::universe() - all_taken_cards, 5 - board.len());
    let hand_distributions = pockets
        .iter()
        .map(|p| {
            let hands = parallel_map(runouts.as_slice(), |r| *r | *p);
            HandDistribution::evaluate(hands.as_slice())
        })
        .collect::<Vec<HandDistribution>>();

    let subranges = SubrangeIterator::from_range(0..runouts.len(), get_parallelism_from_os())
        .collect::<Vec<Range<usize>>>();
    let pocket_win_chunks = parallel_map(subranges.as_slice(), |range| {
        let mut ret = (0..pockets.len()).map(|_| 0).collect::<Vec<usize>>();

        for runout in &runouts[range.clone()] {
            let winning_index = pockets
                .iter()
                .enumerate()
                .map(|(i, pocket)| (HandEvaluation::evaluate_postflop(*pocket | *runout), i))
                .max()
                .unwrap()
                .1;
            ret[winning_index] += 1;
        }
        ret
    });
    let pocket_wins = into_parallel_reduce(pocket_win_chunks, |a, b| {
        a.into_iter()
            .zip(b.into_iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<usize>>()
    })
    .unwrap();

    Ok(hand_distributions
        .into_iter()
        .zip(pocket_wins.into_iter())
        .enumerate()
        .map(|(i, (h, p))| OddsCalculation {
            winning_chance: SatisfactionFraction {
                satisfying: p,
                total: runouts.len(),
            },
            hand_distribution: h,
            pocket: pockets[i],
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::cards::card::Card;

    use super::*;

    #[test]
    fn test_aks_vs_qq() {
        let aks = CardSet::from(&[Card::ACE_SPADE, Card::KING_SPADE]);
        let qq = CardSet::from(&[Card::QUEEN_CLUB, Card::QUEEN_DIAMOND]);

        let expected = [
            OddsCalculation {
                pocket: aks,
                winning_chance: SatisfactionFraction {
                    satisfying: 787966,
                    total: 1712304,
                },
                hand_distribution: HandDistribution {
                    straight_flushes: 1063,
                    four_of_a_kinds: 2420,
                    full_houses: 41716,
                    flushes: 124370,
                    straights: 36669,
                    three_of_a_kinds: 78056,
                    two_pairs: 392692,
                    pairs: 736792,
                    high_cards: 298526,
                    discarded_hands: 0,
                },
            },
            OddsCalculation {
                pocket: qq,
                winning_chance: SatisfactionFraction {
                    satisfying: 924338,
                    total: 1712304,
                },
                hand_distribution: HandDistribution {
                    straight_flushes: 289,
                    four_of_a_kinds: 15620,
                    full_houses: 149956,
                    flushes: 38684,
                    straights: 26313,
                    three_of_a_kinds: 208787,
                    two_pairs: 669894,
                    pairs: 602761,
                    high_cards: 0,
                    discarded_hands: 0,
                },
            },
        ]
        .into_iter()
        .collect::<HashSet<OddsCalculation>>();
    }
}
