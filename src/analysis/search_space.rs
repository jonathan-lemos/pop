use std::{
    collections::HashMap,
    mem::MaybeUninit,
    ops::Range,
    thread::{self},
};

use crate::{
    analysis::math::n_choose_r,
    cards::{
        card::{ALL_CARDS, Card},
        cardset::CardSet,
        hand::Hand,
    },
    parallelism::{
        algorithms::divide_and_conquer, concurrency_limiter::ConcurrencyLimiter,
        os::get_parallelism_from_os,
    },
};

struct MutPtr<T> {
    pub ptr: *mut T,
}

unsafe impl<T> Send for MutPtr<T> {}
unsafe impl<T> Sync for MutPtr<T> {}

unsafe fn parallel_combinations_of_slice<'scope, 'env>(
    slice: &'env [Card],
    current: CardSet,
    needed: usize,
    concurrency_limiter: &'env ConcurrencyLimiter,
    thread_scope: &'scope thread::Scope<'scope, 'env>,
    output: MutPtr<CardSet>,
) {
    if needed == 0 {
        unsafe { *output.ptr = current };
        return;
    }

    let mut current_ptr = output.ptr;

    for i in 0..slice.len() - needed + 1 {
        let amount = n_choose_r(slice.len() - i - 1, needed - 1);
        debug_assert!(amount > 0);

        let current_mut_ptr = MutPtr { ptr: current_ptr };

        let mut new_current = current;
        new_current.add(slice[i]);

        concurrency_limiter.run_scoped(thread_scope, move || unsafe {
            new_current.add(slice[i]);

            parallel_combinations_of_slice(
                &slice[i + 1..],
                new_current,
                needed - 1,
                concurrency_limiter,
                thread_scope,
                current_mut_ptr,
            )
        });

        current_ptr = unsafe { current_ptr.add(amount) };
    }
}

pub fn combinations(pool: CardSet, size: usize) -> Vec<CardSet> {
    if pool.len() < size {
        return vec![];
    }

    let amount = n_choose_r(pool.len(), size);
    let mut ret = Vec::new();
    ret.reserve_exact(amount);

    let cards = pool.iter_desc().collect::<Vec<Card>>();
    let concurrency_limiter = ConcurrencyLimiter::new(get_parallelism_from_os().get() - 1);
    let mut_ptr = MutPtr {
        ptr: ret.as_mut_ptr(),
    };

    thread::scope(|s| unsafe {
        parallel_combinations_of_slice(
            cards.as_slice(),
            CardSet::new(),
            size,
            &concurrency_limiter,
            s,
            mut_ptr,
        );
    });

    unsafe { ret.set_len(amount) };

    ret
}

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
                                ret.push(Hand::new(&[
                                    cs[x1], cs[x2], cs[x3], cs[x4], cs[x5], cs[x6], cs[x7],
                                ]))
                            }
                        }
                    }
                }
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    fn debug_print(sets: &Vec<Vec<Card>>) {
        for (i, set) in sets.iter().enumerate() {
            let cardset = set.iter().map(|x| *x).collect::<CardSet>();
            println!("{}: {}", i, cardset);
        }
    }

    fn to_vec(cardset: CardSet) -> Vec<Card> {
        cardset.iter_desc().collect()
    }

    #[test]
    fn test_combinations_empty() {
        let set = CardSet::from(&[Card::ACE_SPADE, Card::KING_HEART, Card::QUEEN_DIAMOND]);
        let expected: Vec<CardSet> = vec![CardSet::new()];

        let actual = combinations(set, 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_combinations_of_slice_single() {
        let set = CardSet::from(&[Card::ACE_SPADE, Card::KING_HEART, Card::QUEEN_DIAMOND]);
        let expected: Vec<CardSet> = vec![
            CardSet::from(&[Card::ACE_SPADE]),
            CardSet::from(&[Card::KING_HEART]),
            CardSet::from(&[Card::QUEEN_DIAMOND]),
        ];

        let actual = combinations(set, 1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_combinations_of_slice_small() {
        let set = CardSet::from(&[Card::ACE_SPADE, Card::KING_HEART, Card::QUEEN_DIAMOND]);
        let expected = vec![
            CardSet::from(&[Card::ACE_SPADE, Card::KING_HEART]),
            CardSet::from(&[Card::ACE_SPADE, Card::QUEEN_DIAMOND]),
            CardSet::from(&[Card::KING_HEART, Card::QUEEN_DIAMOND]),
        ];

        let actual = combinations(set, 2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_combinations_of_slice_big() {
        let set = CardSet::from(&[
            Card::ACE_SPADE,
            Card::KING_HEART,
            Card::QUEEN_DIAMOND,
            Card::JACK_CLUB,
            Card::TEN_SPADE,
        ]);
        let expected = vec![
            CardSet::from(&[Card::ACE_SPADE, Card::KING_HEART, Card::QUEEN_DIAMOND]),
            CardSet::from(&[Card::ACE_SPADE, Card::KING_HEART, Card::JACK_CLUB]),
            CardSet::from(&[Card::ACE_SPADE, Card::KING_HEART, Card::TEN_SPADE]),
            CardSet::from(&[Card::ACE_SPADE, Card::QUEEN_DIAMOND, Card::JACK_CLUB]),
            CardSet::from(&[Card::ACE_SPADE, Card::QUEEN_DIAMOND, Card::TEN_SPADE]),
            CardSet::from(&[Card::ACE_SPADE, Card::JACK_CLUB, Card::TEN_SPADE]),
            CardSet::from(&[Card::KING_HEART, Card::QUEEN_DIAMOND, Card::JACK_CLUB]),
            CardSet::from(&[Card::KING_HEART, Card::QUEEN_DIAMOND, Card::TEN_SPADE]),
            CardSet::from(&[Card::KING_HEART, Card::JACK_CLUB, Card::TEN_SPADE]),
            CardSet::from(&[Card::QUEEN_DIAMOND, Card::JACK_CLUB, Card::TEN_SPADE]),
        ];

        let actual = combinations(set, 3);
        assert_eq!(actual, expected);
    }
}
