use std::ops::Range;

use crate::analysis::evaluate_hand::HandEvaluation;
use crate::analysis::hand_distribution::HandDistribution;
use crate::analysis::math::SatisfactionFraction;
use crate::analysis::search_space::combinations;
use crate::cards::{card::Card, cardset::CardSet};
use crate::parallelism::algorithms::{SubrangeIterator, into_parallel_reduce, parallel_map};
use crate::parallelism::os::get_parallelism_from_os;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OddsError {
    MustHaveAtLeastOnePlayer,
    BoardCannotHaveMoreThan5Cards,
    CannotHaveDuplicateCards,
    PocketsMustHaveTwoCardsEach,
}

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
        if all_taken_cards & *pocket != CardSet::new() {
            return Err(OddsError::CannotHaveDuplicateCards);
        }
        all_taken_cards.add_all(*pocket);
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
