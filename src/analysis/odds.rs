use crate::{
    analysis::{
        hand_distribution::HandDistribution, math::SatisfactionRatio, search_space::combinations,
    },
    cards::{
        card::Card,
        cardset::CardSet,
        hand::{Hand, HandEvaluation},
    },
};

pub enum OddsError {
    MustHaveAtLeastOnePlayer,
    BoardCannotHaveMoreThan5Cards,
    CannotHaveDuplicateCards,
}

/*
pub struct OddsCalculation {
    pocket: &[Card; 2],
    winning_chance: SatisfactionRatio,
    hand_distribution: HandDistribution,
}

pub fn odds(players: &[[Card; 2]], board: &[Card]) -> Result<SatisfactionRatio, OddsError> {
    if players.is_empty() {
        return Err(OddsError::MustHaveAtLeastOnePlayer);
    }

    if board.len() > 5 {
        return Err(OddsError::BoardCannotHaveMoreThan5Cards);
    }

    let mut cardset = CardSet::new();
    let mut add_card = |card| {
        if cardset.has(card) {
            return Err(OddsError::CannotHaveDuplicateCards);
        }
        cardset.add(card);
        Ok(())
    };

    for hole in players {
        add_card(hole[0])?;
        add_card(hole[1])?;
    }

    for card in board {
        add_card(*card)?;
    }

    let left = 5 - board.len();
    let remaining_cards = CardSet::universe() - cardset;
    let combs = combinations(remaining_cards, left);

    let ratio = parallel_map_reduce(
        combs.as_slice(),
        |x| {
            let hands = x
                .into_iter()
                .map(|y| {
                    let hand = unsafe { Hand::from_cardset_unchecked(*y | cardset) };
                    HandEvaluation::evaluate(&hand)
                })
                .collect::<Vec<HandEvaluation>>();
        },
        |hs| hs.into_iter().fold(HandDistribution::new(), |a, b| a + b),
    );

    Ok(ratio)
}
*/
