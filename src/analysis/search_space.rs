use std::{
    collections::HashMap,
    mem::MaybeUninit,
    ops::Range,
    sync::atomic::{AtomicUsize, Ordering},
    thread::{self},
};

use crossbeam_channel::{Receiver, Sender};

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

#[derive(Clone, Copy)]
struct MutPtr<T> {
    pub ptr: *mut T,
}

unsafe impl<T> Send for MutPtr<T> {}
unsafe impl<T> Sync for MutPtr<T> {}

unsafe fn parallel_combinations_of_slice_of_len_1(
    slice: &[Card],
    current: CardSet,
    output: MutPtr<CardSet>,
    amount_done: &AtomicUsize,
) {
    let mut ptr = output.ptr;
    let mut set = current;

    for x1 in 0..slice.len() {
        set.add(slice[x1]);
        unsafe {
            *ptr = set;
            ptr = ptr.add(1);
        }
        set.remove(slice[x1]);
    }

    amount_done.fetch_add(n_choose_r(slice.len(), 1), Ordering::Release);
}

unsafe fn parallel_combinations_of_slice_of_len_2(
    slice: &[Card],
    current: CardSet,
    output: MutPtr<CardSet>,
    amount_done: &AtomicUsize,
) {
    let mut ptr = output.ptr;
    let mut set = current;

    for x1 in 0..slice.len() {
        set.add(slice[x1]);
        for x2 in x1 + 1..slice.len() {
            set.add(slice[x2]);
            unsafe {
                *ptr = set;
                ptr = ptr.add(1);
            }
            set.remove(slice[x2]);
        }
        set.remove(slice[x1]);
    }

    amount_done.fetch_add(n_choose_r(slice.len(), 2), Ordering::Release);
}

unsafe fn parallel_combinations_of_slice_of_len_3(
    slice: &[Card],
    current: CardSet,
    output: MutPtr<CardSet>,
    amount_done: &AtomicUsize,
) {
    let mut ptr = output.ptr;
    let mut set = current;

    for x1 in 0..slice.len() {
        set.add(slice[x1]);
        for x2 in x1 + 1..slice.len() {
            set.add(slice[x2]);
            for x3 in x2 + 1..slice.len() {
                set.add(slice[x3]);
                unsafe {
                    *ptr = set;
                    ptr = ptr.add(1);
                }
                set.remove(slice[x3]);
            }
            set.remove(slice[x2]);
        }
        set.remove(slice[x1]);
    }

    amount_done.fetch_add(n_choose_r(slice.len(), 3), Ordering::Release);
}

unsafe fn parallel_combinations_of_slice_of_len_4(
    slice: &[Card],
    current: CardSet,
    output: MutPtr<CardSet>,
    amount_done: &AtomicUsize,
) {
    let mut ptr = output.ptr;
    let mut set = current;

    for x1 in 0..slice.len() {
        set.add(slice[x1]);
        for x2 in x1 + 1..slice.len() {
            set.add(slice[x2]);
            for x3 in x2 + 1..slice.len() {
                set.add(slice[x3]);
                for x4 in x3 + 1..slice.len() {
                    set.add(slice[x4]);
                    unsafe {
                        *ptr = set;
                        ptr = ptr.add(1);
                    }
                    set.remove(slice[x4]);
                }
                set.remove(slice[x3]);
            }
            set.remove(slice[x2]);
        }
        set.remove(slice[x1]);
    }

    amount_done.fetch_add(n_choose_r(slice.len(), 4), Ordering::Release);
}

unsafe fn parallel_combinations_of_slice_of_len_5(
    slice: &[Card],
    current: CardSet,
    output: MutPtr<CardSet>,
    amount_done: &AtomicUsize,
) {
    let mut ptr = output.ptr;
    let mut set = current;

    for x1 in 0..slice.len() {
        set.add(slice[x1]);
        for x2 in x1 + 1..slice.len() {
            set.add(slice[x2]);
            for x3 in x2 + 1..slice.len() {
                set.add(slice[x3]);
                for x4 in x3 + 1..slice.len() {
                    set.add(slice[x4]);
                    for x5 in x4 + 1..slice.len() {
                        set.add(slice[x5]);
                        unsafe {
                            *ptr = set;
                            ptr = ptr.add(1);
                        }
                        set.remove(slice[x5]);
                    }
                    set.remove(slice[x4]);
                }
                set.remove(slice[x3]);
            }
            set.remove(slice[x2]);
        }
        set.remove(slice[x1]);
    }

    amount_done.fetch_add(n_choose_r(slice.len(), 5), Ordering::Release);
}

unsafe fn parallel_combinations_of_slice<'a>(
    sender: &Sender<Option<ParallelCombinationsWorkItem<'a>>>,
    amount_done: &AtomicUsize,
    work_item: ParallelCombinationsWorkItem<'a>,
) {
    let slice = work_item.slice;
    let current = work_item.current;
    let needed = work_item.needed;
    let output = work_item.output;

    match work_item.needed {
        0 => {
            unsafe { *output.ptr = current };
            amount_done.fetch_add(1, Ordering::Release);
            return;
        }
        1 => {
            unsafe { parallel_combinations_of_slice_of_len_1(slice, current, output, amount_done) };
            return;
        }
        2 => {
            unsafe { parallel_combinations_of_slice_of_len_2(slice, current, output, amount_done) };
            return;
        }
        3 => {
            unsafe { parallel_combinations_of_slice_of_len_3(slice, current, output, amount_done) };
            return;
        }
        4 => {
            unsafe { parallel_combinations_of_slice_of_len_4(slice, current, output, amount_done) };
            return;
        }
        5 => {
            unsafe { parallel_combinations_of_slice_of_len_5(slice, current, output, amount_done) };
            return;
        }
        _ => {}
    };

    let mut current_ptr = output.ptr;

    for i in 0..slice.len() - needed + 1 {
        let amount = n_choose_r(slice.len() - i - 1, needed - 1);
        debug_assert!(amount > 0);

        let current_mut_ptr = MutPtr { ptr: current_ptr };

        let mut new_current = current;
        new_current.add(slice[i]);

        sender
            .send(Some(ParallelCombinationsWorkItem {
                slice: &slice[i + 1..],
                current: new_current,
                needed: needed - 1,
                output: current_mut_ptr,
            }))
            .unwrap();

        current_ptr = unsafe { current_ptr.add(amount) };
    }
}

#[derive(Clone, Copy)]
struct ParallelCombinationsWorkItem<'a> {
    pub slice: &'a [Card],
    pub current: CardSet,
    pub needed: usize,
    pub output: MutPtr<CardSet>,
}

pub fn combinations(pool: CardSet, size: usize) -> Vec<CardSet> {
    if pool.len() < size {
        return vec![];
    }

    let amount = n_choose_r(pool.len(), size);
    let mut ret = Vec::new();
    ret.reserve_exact(amount);

    let cards = pool.iter_desc().collect::<Vec<Card>>();

    let (sender, receiver) = crossbeam_channel::unbounded::<Option<ParallelCombinationsWorkItem>>();
    sender
        .send(Some(ParallelCombinationsWorkItem {
            slice: cards.as_slice(),
            current: CardSet::new(),
            needed: size,
            output: MutPtr {
                ptr: ret.as_mut_ptr(),
            },
        }))
        .unwrap();

    let amount_done = AtomicUsize::new(0);

    let n_threads = get_parallelism_from_os().get();

    thread::scope(|s| unsafe {
        for _ in 0..n_threads {
            s.spawn(|| {
                while let Some(work_item) = receiver.recv().unwrap() {
                    parallel_combinations_of_slice(&sender, &amount_done, work_item);
                    if amount_done.load(Ordering::Acquire) == amount {
                        for _ in 0..n_threads {
                            sender.send(None).unwrap();
                        }
                    }
                }
            });
        }
    });

    unsafe { ret.set_len(amount) };

    ret
}

pub fn legacy() -> Vec<CardSet> {
    let mut ret = Vec::new();

    for x1 in 0..ALL_CARDS.len() {
        for x2 in x1 + 1..ALL_CARDS.len() {
            let mut cs = CardSet::new();
            cs.add(ALL_CARDS[x1]);
            cs.add(ALL_CARDS[x2]);
            ret.push(cs);
        }
    }

    ret
}

pub fn all_seven_card_hands_legacy() -> Vec<Hand> {
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

pub fn all_seven_card_hands() -> Vec<Hand> {
    combinations(CardSet::universe(), 7)
        .into_iter()
        .map(|x| unsafe { Hand::from_cardset(x) })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn debug_print(sets: &Vec<CardSet>) {
        for (i, set) in sets.iter().enumerate() {
            println!("{}: {}", i, set);
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

    /*
    #[test]
    fn test_same() {
        let expected = legacy();
        let actual = combinations(CardSet::universe(), 2);

        println!("expected:");
        debug_print(&expected);
        println!("actual:");
        debug_print(&actual);

        let eset = expected.iter().map(|x| *x).collect::<HashSet<CardSet>>();
        let aset = actual.iter().map(|x| *x).collect::<HashSet<CardSet>>();

        println!("difference:");
        debug_print(&eset.difference(&aset).map(|x| *x).collect::<Vec<CardSet>>());

        assert_eq!(eset, aset);
    }
    */
}
