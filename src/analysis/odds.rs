use std::ops::Range;

use crate::analysis::evaluate_hand::HandEvaluation;
use crate::analysis::hand_distribution::HandDistribution;
use crate::analysis::math::SatisfactionFraction;
use crate::analysis::outcomes::Outcome;
use crate::analysis::search_space::combinations;
use crate::cards::cardset::CardSet;
use crate::parallelism::algorithms::{SubrangeIterator, into_parallel_reduce, parallel_map};
use crate::parallelism::os::get_parallelism_from_os;
use crate::util::array::{array_map, indexes, into_array_map};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OddsError {
    MustHaveAtLeastOnePlayer,
    BoardCannotHaveMoreThan5Cards,
    CannotHaveDuplicateCards,
    PocketsMustHaveTwoCardsEach,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OddsCalculation<const N_PLAYERS: usize> {
    pub pocket: CardSet,
    pub outcome: Outcome<N_PLAYERS>,
    pub hand_distribution: HandDistribution,
}

fn get_all_taken_cards<const N_PLAYERS: usize>(
    pockets: &[CardSet; N_PLAYERS],
    board: CardSet,
) -> Result<CardSet, OddsError> {
    if pockets.is_empty() {
        return Err(OddsError::MustHaveAtLeastOnePlayer);
    }

    if pockets.iter().any(|p| p.len() != 2) {
        return Err(OddsError::PocketsMustHaveTwoCardsEach);
    }

    if board.len() > 5 {
        return Err(OddsError::BoardCannotHaveMoreThan5Cards);
    }

    let mut all_taken_cards = match CardSet::union_if_disjoint(pockets) {
        Some(v) => v,
        None => return Err(OddsError::CannotHaveDuplicateCards),
    };
    if !all_taken_cards.disjoint_with(board) {
        return Err(OddsError::CannotHaveDuplicateCards);
    }
    all_taken_cards |= board;

    Ok(all_taken_cards)
}

pub fn calculate_odds<const N_PLAYERS: usize>(
    pockets: &[CardSet; N_PLAYERS],
    board: CardSet,
) -> Result<[OddsCalculation<N_PLAYERS>; N_PLAYERS], OddsError> {
    let all_taken_cards = get_all_taken_cards(pockets, board)?;

    let runouts = combinations(CardSet::universe() - all_taken_cards, 5 - board.len());
    let hand_distributions = array_map(pockets, |pocket| {
        let hands = parallel_map(runouts.as_slice(), |runout| *runout | *pocket);
        HandDistribution::evaluate(hands.as_slice())
    });

    if hand_distributions.iter().any(|h| !h.is_complete()) {
        panic!("Internal error computing the hand distributions.");
    }

    let outcomes = Outcome::evaluate(pockets, runouts.as_slice()).unwrap();

    let result = into_array_map(indexes::<N_PLAYERS>(), |i| OddsCalculation {
        pocket: pockets[i],
        outcome: outcomes[i],
        hand_distribution: hand_distributions[i],
    });

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::card::Card;

    fn assert_roughly_eq(a: f64, b: f64) {
        assert_eq!(format!("{:.2}", a), format!("{:.2}", b));
    }

    #[test]
    fn test_aks_vs_qq() {
        let aks = CardSet::from(&[Card::ACE_SPADE, Card::KING_SPADE]);
        let qq = CardSet::from(&[Card::QUEEN_CLUB, Card::QUEEN_DIAMOND]);

        let odds = calculate_odds(&[aks, qq], CardSet::new()).unwrap();
        let aks_odds = &odds[0];
        let qq_odds = &odds[1];

        assert_roughly_eq(aks_odds.outcome.win_ratio().percentage(), 46.02);
        assert_roughly_eq(aks_odds.outcome.draw_ratio().percentage(), 0.39);
        assert_roughly_eq(qq_odds.outcome.win_ratio().percentage(), 53.59);
        assert_roughly_eq(qq_odds.outcome.draw_ratio().percentage(), 0.39);

        assert_roughly_eq(aks_odds.hand_distribution.straight_flush_percentage(), 0.06);
        assert_roughly_eq(aks_odds.hand_distribution.four_of_a_kind_percentage(), 0.14);
        assert_roughly_eq(aks_odds.hand_distribution.full_house_percentage(), 2.44);
        assert_roughly_eq(aks_odds.hand_distribution.flush_percentage(), 7.26);
        assert_roughly_eq(aks_odds.hand_distribution.straight_percentage(), 2.14);
        assert_roughly_eq(
            aks_odds.hand_distribution.three_of_a_kind_percentage(),
            4.56,
        );
        assert_roughly_eq(aks_odds.hand_distribution.two_pair_percentage(), 22.93);
        assert_roughly_eq(aks_odds.hand_distribution.pair_percentage(), 43.03);
        assert_roughly_eq(aks_odds.hand_distribution.high_card_percentage(), 17.43);
    }
}
