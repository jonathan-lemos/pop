use std::ops::Range;

use rand::SeedableRng;

use crate::{
    cards::{card::Card, deck::Deck, hand::*},
    parallelism::divide_and_conquer::divide_and_conquer,
};

mod cards;
mod datastructures;
mod parallelism;
mod test_util;

struct Counts {
    sf: usize,
    k4: usize,
    fh: usize,
    f: usize,
    s: usize,
    k3: usize,
    p2: usize,
    p1: usize,
    hc: usize,
}

fn count_ak_distribution(r: Range<usize>) -> Counts {
    let mut starting_deck = Deck::new();
    let mut rng = rand_chacha::ChaCha20Rng::try_from_os_rng().unwrap();

    starting_deck.remove_card(Card::ACE_SPADE);
    starting_deck.remove_card(Card::KING_SPADE);
    starting_deck.remove_card(Card::QUEEN_CLUB);
    starting_deck.remove_card(Card::QUEEN_DIAMOND);

    let mut ret = Counts {
        sf: 0,
        k4: 0,
        fh: 0,
        f: 0,
        s: 0,
        k3: 0,
        p2: 0,
        p1: 0,
        hc: 0,
    };

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
        match ak_hand {
            Hand::StraightFlush { highest_rank } => ret.sf += 1,
            Hand::FourOfAKind { rank, kicker } => ret.k4 += 1,
            Hand::FullHouse { triple, pair } => ret.fh += 1,
            Hand::Flush { ranks_sorted_desc } => ret.f += 1,
            Hand::Straight { highest_rank } => ret.s += 1,
            Hand::ThreeOfAKind {
                rank,
                kickers_sorted_desc,
            } => ret.k3 += 1,
            Hand::TwoPair {
                higher_rank,
                lower_rank,
                kicker,
            } => ret.p2 += 1,
            Hand::Pair {
                rank,
                kickers_sorted_desc,
            } => ret.p1 += 1,
            Hand::HighCard {
                rank,
                kickers_sorted_desc,
            } => ret.hc += 1,
        }
    }

    ret
}

fn main() {
    let denom = 1_000_000;
    let num = denom * 100;

    let cts = divide_and_conquer(0..num, count_ak_distribution)
        .into_iter()
        .reduce(|a, b| Counts {
            sf: a.sf + b.sf,
            k4: a.k4 + b.k4,
            fh: a.fh + b.fh,
            f: a.f + b.f,
            s: a.s + b.s,
            k3: a.k3 + b.k3,
            p2: a.p2 + b.p2,
            p1: a.p1 + b.p1,
            hc: a.hc + b.hc,
        })
        .unwrap();

    println!("sf: {}", cts.sf as f64 / denom as f64);
    println!("k4: {}", cts.k4 as f64 / denom as f64);
    println!("fh: {}", cts.fh as f64 / denom as f64);
    println!("f: {}", cts.f as f64 / denom as f64);
    println!("s: {}", cts.s as f64 / denom as f64);
    println!("k3: {}", cts.k3 as f64 / denom as f64);
    println!("p2: {}", cts.p2 as f64 / denom as f64);
    println!("p1: {}", cts.p1 as f64 / denom as f64);
    println!("hc: {}", cts.hc as f64 / denom as f64);
}
