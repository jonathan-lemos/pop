use crate::analysis::odds::calculate_odds;
use crate::cards::{card::Card, cardset::CardSet};

mod analysis;
mod cards;
mod datastructures;
mod parallelism;
mod test_util;
mod util;

fn main() {
    let pockets = &[
        CardSet::from(&[Card::ACE_SPADE, Card::KING_SPADE]),
        CardSet::from(&[Card::QUEEN_CLUB, Card::QUEEN_DIAMOND]),
    ];

    let odds = calculate_odds(pockets, CardSet::new()).unwrap();
    for calc in odds {
        println!("{}", calc.pocket);
        println!("{:?}", calc.winning_chance);
        println!("{:?}", calc.hand_distribution);
        println!("")
    }
}
