use crate::{
    analysis::{odds::OddsCalculation, outcomes::Outcome},
    cards::{card::Card, cardset::CardSet},
};

pub fn format_separated_values<
    T,
    I: Iterator<Item = T>,
    F: FnMut(T, &mut std::fmt::Formatter<'_>) -> std::fmt::Result,
>(
    values: I,
    separator: &str,
    fmt: &mut std::fmt::Formatter<'_>,
    mut f: F,
) -> std::fmt::Result {
    let mut first = true;
    for value in values {
        if !first {
            fmt.write_str(separator)?;
        } else {
            first = false;
        }
        f(value, fmt)?;
    }
    Ok(())
}

pub fn as_percentage(p: f64) -> String {
    format!("{:.2}%", p)
}

pub fn print_odds<const N_PLAYERS: usize>(odds: OddsCalculation<N_PLAYERS>) {
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

pub struct Input {
    pub pockets: [CardSet; 2],
    pub board: CardSet,
}

pub fn parse_input(args: &[String]) -> Input {
    let first_card = Card::parse(&args[0]).unwrap();
    let second_card = Card::parse(&args[1]).unwrap();
    let third_card = Card::parse(&args[2]).unwrap();
    let fourth_card = Card::parse(&args[3]).unwrap();

    Input {
        pockets: [
            CardSet::from(&[first_card, second_card]),
            CardSet::from(&[third_card, fourth_card]),
        ],
        board: CardSet::new(),
    }
}
