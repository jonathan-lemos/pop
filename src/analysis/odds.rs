use crate::analysis::hand_distribution::HandDistribution;
use crate::analysis::outcomes::Outcome;
use crate::analysis::search_space::{combinations, undealt_cards};
use crate::cards::cardset::CardSet;
use crate::parallelism::algorithms::{into_parallel_map, parallel_map};
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

pub fn calculate_odds<const N_PLAYERS: usize>(
    pockets: &[CardSet; N_PLAYERS],
    board: CardSet,
) -> [OddsCalculation<N_PLAYERS>; N_PLAYERS] {
    let undealt = undealt_cards(pockets, board);

    let runouts = combinations(undealt, 5 - board.len());
    let boards = into_parallel_map(runouts, |x| x | board);

    let hand_distributions = array_map(pockets, |pocket| {
        let hands = parallel_map(boards.as_slice(), |runout| *runout | *pocket);
        HandDistribution::evaluate(hands.as_slice())
    });

    if hand_distributions.iter().any(|h| !h.is_complete()) {
        panic!("Internal error computing the hand distributions.");
    }

    let outcomes = Outcome::evaluate(pockets, boards.as_slice());

    into_array_map(indexes::<N_PLAYERS>(), |i| OddsCalculation {
        pocket: pockets[i],
        outcome: outcomes[i],
        hand_distribution: hand_distributions[i],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::card::Card;

    fn assert_roughly_eq(a: f64, b: f64) {
        assert_eq!(format!("{:.2}", a), format!("{:.2}", b));
    }

    #[test]
    #[ignore = "This test is computationally intensive. Run it with `cargo test -- --include-ignored`"]
    fn test_aks_vs_qq() {
        let aks = CardSet::from(&[Card::ACE_SPADE, Card::KING_SPADE]);
        let qq = CardSet::from(&[Card::QUEEN_CLUB, Card::QUEEN_DIAMOND]);

        let odds = calculate_odds(&[aks, qq], CardSet::new());
        let aks_odds = &odds[0];
        let qq_odds = &odds[1];

        // From https://www.pokernews.com/poker-tools/poker-odds-calculator.htm
        assert_roughly_eq(aks_odds.outcome.win_ratio().percentage(), 46.02);
        assert_roughly_eq(aks_odds.outcome.draw_ratio().percentage(), 0.39);
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

        assert_roughly_eq(qq_odds.outcome.win_ratio().percentage(), 53.59);
        assert_roughly_eq(qq_odds.outcome.draw_ratio().percentage(), 0.39);
        assert_roughly_eq(qq_odds.hand_distribution.straight_flush_percentage(), 0.02);
        assert_roughly_eq(qq_odds.hand_distribution.four_of_a_kind_percentage(), 0.91);
        assert_roughly_eq(qq_odds.hand_distribution.full_house_percentage(), 8.76);
        assert_roughly_eq(qq_odds.hand_distribution.flush_percentage(), 2.26);
        assert_roughly_eq(qq_odds.hand_distribution.straight_percentage(), 1.54);
        assert_roughly_eq(
            qq_odds.hand_distribution.three_of_a_kind_percentage(),
            12.19,
        );
        assert_roughly_eq(qq_odds.hand_distribution.two_pair_percentage(), 39.12);
        assert_roughly_eq(qq_odds.hand_distribution.pair_percentage(), 35.2);
        assert_roughly_eq(qq_odds.hand_distribution.high_card_percentage(), 0.0);
    }

    #[test]
    #[ignore = "This test is computationally intensive. Run it with `cargo test -- --include-ignored`"]
    fn test_aks_vs_jj_vs_98s() {
        let aks = CardSet::from(&[Card::ACE_SPADE, Card::KING_SPADE]);
        let jj = CardSet::from(&[Card::JACK_CLUB, Card::JACK_DIAMOND]);
        let s98 = CardSet::from(&[Card::NINE_HEART, Card::EIGHT_HEART]);

        let odds = calculate_odds(&[aks, jj, s98], CardSet::new());
        let aks_odds = &odds[0];
        let jj_odds = &odds[1];
        let s98_odds = &odds[2];

        // From https://www.pokernews.com/poker-tools/poker-odds-calculator.htm
        assert_roughly_eq(aks_odds.outcome.win_ratio().percentage(), 39.46);
        assert_roughly_eq(aks_odds.outcome.draw_ratio().percentage(), 0.18);
        assert_roughly_eq(aks_odds.hand_distribution.straight_flush_percentage(), 0.07);
        assert_roughly_eq(aks_odds.hand_distribution.four_of_a_kind_percentage(), 0.16);
        assert_roughly_eq(aks_odds.hand_distribution.full_house_percentage(), 2.62);
        assert_roughly_eq(aks_odds.hand_distribution.flush_percentage(), 8.12);
        assert_roughly_eq(aks_odds.hand_distribution.straight_percentage(), 2.34);
        assert_roughly_eq(
            aks_odds.hand_distribution.three_of_a_kind_percentage(),
            4.71,
        );
        assert_roughly_eq(aks_odds.hand_distribution.two_pair_percentage(), 23.34);
        assert_roughly_eq(aks_odds.hand_distribution.pair_percentage(), 42.15);
        assert_roughly_eq(aks_odds.hand_distribution.high_card_percentage(), 16.5);

        assert_roughly_eq(jj_odds.outcome.win_ratio().percentage(), 41.13);
        assert_roughly_eq(jj_odds.outcome.draw_ratio().percentage(), 0.18);
        assert_roughly_eq(jj_odds.hand_distribution.straight_flush_percentage(), 0.03);
        assert_roughly_eq(jj_odds.hand_distribution.four_of_a_kind_percentage(), 0.99);
        assert_roughly_eq(jj_odds.hand_distribution.full_house_percentage(), 8.93);
        assert_roughly_eq(jj_odds.hand_distribution.flush_percentage(), 2.61);
        assert_roughly_eq(jj_odds.hand_distribution.straight_percentage(), 1.67);
        assert_roughly_eq(
            jj_odds.hand_distribution.three_of_a_kind_percentage(),
            12.68,
        );
        assert_roughly_eq(jj_odds.hand_distribution.two_pair_percentage(), 38.46);
        assert_roughly_eq(jj_odds.hand_distribution.pair_percentage(), 34.63);
        assert_roughly_eq(jj_odds.hand_distribution.high_card_percentage(), 0.0);

        assert_roughly_eq(s98_odds.outcome.win_ratio().percentage(), 19.23);
        assert_roughly_eq(s98_odds.outcome.draw_ratio().percentage(), 0.18);
        assert_roughly_eq(s98_odds.hand_distribution.straight_flush_percentage(), 0.26);
        assert_roughly_eq(s98_odds.hand_distribution.four_of_a_kind_percentage(), 0.16);
        assert_roughly_eq(s98_odds.hand_distribution.full_house_percentage(), 2.62);
        assert_roughly_eq(s98_odds.hand_distribution.flush_percentage(), 7.93);
        assert_roughly_eq(s98_odds.hand_distribution.straight_percentage(), 8.1);
        assert_roughly_eq(
            s98_odds.hand_distribution.three_of_a_kind_percentage(),
            4.61,
        );
        assert_roughly_eq(s98_odds.hand_distribution.two_pair_percentage(), 22.88);
        assert_roughly_eq(s98_odds.hand_distribution.pair_percentage(), 39.22);
        assert_roughly_eq(s98_odds.hand_distribution.high_card_percentage(), 14.23);
    }

    #[test]
    #[ignore = "This test is computationally intensive. Run it with `cargo test -- --include-ignored`"]
    fn test_kqs_vs_tt() {
        let kqs = CardSet::from(&[Card::KING_SPADE, Card::QUEEN_SPADE]);
        let tt = CardSet::from(&[Card::TEN_CLUB, Card::TEN_DIAMOND]);

        let odds = calculate_odds(
            &[kqs, tt],
            CardSet::from(&[Card::JACK_SPADE, Card::TEN_SPADE, Card::SIX_DIAMOND]),
        );
        let kqs_odds = &odds[0];
        let tt_odds = &odds[1];

        // From https://www.pokernews.com/poker-tools/poker-odds-calculator.htm
        assert_roughly_eq(kqs_odds.outcome.win_ratio().percentage(), 42.12);
        assert_roughly_eq(kqs_odds.outcome.draw_ratio().percentage(), 0.0);
        assert_roughly_eq(kqs_odds.hand_distribution.straight_flush_percentage(), 8.79);
        assert_roughly_eq(kqs_odds.hand_distribution.four_of_a_kind_percentage(), 0.0);
        assert_roughly_eq(kqs_odds.hand_distribution.full_house_percentage(), 0.0);
        assert_roughly_eq(kqs_odds.hand_distribution.flush_percentage(), 27.58);
        assert_roughly_eq(kqs_odds.hand_distribution.straight_percentage(), 19.7);
        assert_roughly_eq(
            kqs_odds.hand_distribution.three_of_a_kind_percentage(),
            1.01,
        );
        assert_roughly_eq(kqs_odds.hand_distribution.two_pair_percentage(), 5.66);
        assert_roughly_eq(kqs_odds.hand_distribution.pair_percentage(), 23.64);
        assert_roughly_eq(kqs_odds.hand_distribution.high_card_percentage(), 13.64);

        assert_roughly_eq(tt_odds.outcome.win_ratio().percentage(), 57.88);
        assert_roughly_eq(tt_odds.outcome.draw_ratio().percentage(), 0.0);
        assert_roughly_eq(tt_odds.hand_distribution.straight_flush_percentage(), 0.0);
        assert_roughly_eq(tt_odds.hand_distribution.four_of_a_kind_percentage(), 4.44);
        assert_roughly_eq(tt_odds.hand_distribution.full_house_percentage(), 30.0);
        assert_roughly_eq(tt_odds.hand_distribution.flush_percentage(), 0.0);
        assert_roughly_eq(tt_odds.hand_distribution.straight_percentage(), 0.0);
        assert_roughly_eq(
            tt_odds.hand_distribution.three_of_a_kind_percentage(),
            65.56,
        );
        assert_roughly_eq(tt_odds.hand_distribution.two_pair_percentage(), 0.0);
        assert_roughly_eq(tt_odds.hand_distribution.pair_percentage(), 0.0);
        assert_roughly_eq(tt_odds.hand_distribution.high_card_percentage(), 0.0);
    }
}
