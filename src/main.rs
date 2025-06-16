use std::ops::Range;

use crate::{
    cards::{card::Card, deck::Deck, hand::evaluate_hand},
    parallelism::divide_and_conquer::divide_and_conquer,
};

mod cards;
mod datastructures;
mod parallelism;
mod test_util;

fn count_ak_half_wins_vs_qq(r: Range<usize>) -> usize {
    let mut starting_deck = Deck::new();
    let mut rng = rand::rng();

    starting_deck.remove_card(Card::ACE_SPADE);
    starting_deck.remove_card(Card::KING_SPADE);
    starting_deck.remove_card(Card::QUEEN_CLUB);
    starting_deck.remove_card(Card::QUEEN_DIAMOND);

    let mut half_wins = 0;

    for _ in r {
        let mut deck = starting_deck.clone();
        let board = [
            deck.deal(&mut rng),
            deck.deal(&mut rng),
            deck.deal(&mut rng),
            deck.deal(&mut rng),
            deck.deal(&mut rng),
        ];

        let ak_hand = evaluate_hand(&[Card::ACE_SPADE, Card::KING_SPADE], &board);
        let qq_hand = evaluate_hand(&[Card::QUEEN_CLUB, Card::QUEEN_DIAMOND], &board);

        if ak_hand > qq_hand {
            half_wins += 2;
        } else if ak_hand == qq_hand {
            half_wins += 1;
        }
    }

    half_wins
}

fn main() {
    let half_wins = divide_and_conquer(0..100_000_000, count_ak_half_wins_vs_qq)
        .into_iter()
        .reduce(|a, b| a + b)
        .unwrap();

    println!(
        "AK wins {} of the time",
        half_wins as f64 / 200_000_000 as f64,
    );
}
