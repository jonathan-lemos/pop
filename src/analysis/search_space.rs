use crate::cards::{card::ALL_CARDS, hand::Hand};

pub fn all_seven_card_hands() -> Vec<Hand> {
    let cs = ALL_CARDS;
    let mut ret = Vec::new();

    for x1 in 0..ALL_CARDS.len() {
        for x2 in x1 + 1..ALL_CARDS.len() {
            for x3 in x2 + 1..ALL_CARDS.len() {
                for x4 in x3 + 1..ALL_CARDS.len() {
                    for x5 in x4 + 1..ALL_CARDS.len() {
                        for x6 in x5 + 1..ALL_CARDS.len() {
                            for x7 in x6 + 1..ALL_CARDS.len() {
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
