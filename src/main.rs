use std::ops::Range;

use rand::SeedableRng;

use crate::{
    cards::{
        card::{ALL_CARDS, Card, Rank},
        deck::Deck,
        hand::*,
    },
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

fn enumerate_all_7_card_hands() -> Vec<[Card; 7]> {
    let cs = ALL_CARDS;
    let mut ret = Vec::new();

    for x1 in 0..52 {
        for x2 in (x1 + 1..52) {
            for x3 in (x2 + 1..52) {
                for x4 in (x3 + 1..52) {
                    for x5 in (x4 + 1..52) {
                        for x6 in (x5 + 1..52) {
                            for x7 in (x6 + 1..52) {
                                ret.push([cs[x1], cs[x2], cs[x3], cs[x4], cs[x5], cs[x6], cs[x7]])
                            }
                        }
                    }
                }
            }
        }
    }

    ret
}

fn count_hand_distribution(hands: &[[Card; 7]], r: Range<usize>) -> Counts {
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

    for i in r {
        let hand = hands[i];
        let ak_hand = evaluate_hand(
            &[hand[0], hand[1]],
            &[hand[2], hand[3], hand[4], hand[5], hand[6]],
        );
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
    let all_hands = enumerate_all_7_card_hands();

    let cts = divide_and_conquer(0..all_hands.len(), |r| {
        count_hand_distribution(&all_hands, r)
    })
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

    println!("sf: {}", cts.sf);
    println!("k4: {}", cts.k4);
    println!("fh: {}", cts.fh);
    println!("f: {}", cts.f);
    println!("s: {}", cts.s);
    println!("k3: {}", cts.k3);
    println!("p2: {}", cts.p2);
    println!("p1: {}", cts.p1);
    println!("hc: {}", cts.hc);
    println!(
        "total: {}",
        cts.sf + cts.k4 + cts.fh + cts.f + cts.s + cts.k3 + cts.p2 + cts.p1 + cts.hc
    );
}
