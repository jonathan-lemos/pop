use crate::{
    analysis::odds::OddsCalculation, cards::cardset::CardSet, util::array::MonomorphizedArray,
};

pub struct Showdown {
    pub pockets: MonomorphizedArray<CardSet>,
    pub board: CardSet,
}

pub fn calculate_odds_from_showdown(showdown: &Showdown) -> Vec<OddsCalculation> {
    match showdown.pockets {
        MonomorphizedArray::Len2(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len3(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len4(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len5(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len6(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len7(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len8(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len9(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len10(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len11(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len12(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len13(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len14(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len15(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len16(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len17(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len18(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len19(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len20(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len21(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len22(a) => OddsCalculation::calculate(&a, showdown.board),
        MonomorphizedArray::Len23(a) => OddsCalculation::calculate(&a, showdown.board),
        _ => panic!("Must have 2-23 pockets to calculate odds for showdown"),
    }
}

pub fn print_odds(odds: &OddsCalculation) {
    println!("{}", odds.pocket);
    println!("Win:  {:.2}%", odds.outcome.win_ratio().percentage());
    println!("Draw: {:.2}%", odds.outcome.draw_ratio().percentage());
    println!("Loss: {:.2}%", odds.outcome.loss_ratio().percentage());
    println!("");
    println!("Hand distribution:");
    println!(
        "Straight Flush:  {:.2}%",
        odds.hand_distribution.straight_flush_percentage()
    );
    println!(
        "Four of a Kind:  {:.2}%",
        odds.hand_distribution.four_of_a_kind_percentage()
    );
    println!(
        "Full House:      {:.2}%",
        odds.hand_distribution.full_house_percentage()
    );
    println!(
        "Flush:           {:.2}%",
        odds.hand_distribution.flush_percentage()
    );
    println!(
        "Straight:        {:.2}%",
        odds.hand_distribution.straight_percentage()
    );
    println!(
        "Three of a Kind: {:.2}%",
        odds.hand_distribution.three_of_a_kind_percentage()
    );
    println!(
        "Two Pair:        {:.2}%",
        odds.hand_distribution.two_pair_percentage()
    );
    println!(
        "Pair:            {:.2}%",
        odds.hand_distribution.pair_percentage()
    );
    println!(
        "High Card:       {:.2}%",
        odds.hand_distribution.high_card_percentage()
    );
}

pub fn print_showdown_help(executable_name: &str) {
    println!("{} showdown: Analyze odds for a showdown", executable_name);
    println!(
        "Usage: {} showdown <card><card> [vs <card><card>]* [on <card>+]",
        executable_name
    );
    println!();
    println!("Analyzes the given pockets on the given board.");
    println!("You must give at least two pockets and no more than 5 cards on the board.");
}
