use crate::{
    analysis::math::n_choose_r,
    cards::{card::Card, cardset::CardSet},
    parallelism::{os::get_parallelism_from_os, send_sync_raw_ptr::SendSyncRawPtr},
};
use crossbeam_channel::Sender;
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread::{self},
};

unsafe fn parallel_combinations_of_slice_of_len_1(
    slice: &[Card],
    current: CardSet,
    output: SendSyncRawPtr<CardSet>,
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
        set -= slice[x1];
    }

    amount_done.fetch_add(n_choose_r(slice.len(), 1), Ordering::Release);
}

unsafe fn parallel_combinations_of_slice_of_len_2(
    slice: &[Card],
    current: CardSet,
    output: SendSyncRawPtr<CardSet>,
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
            set -= slice[x2];
        }
        set -= slice[x1];
    }

    amount_done.fetch_add(n_choose_r(slice.len(), 2), Ordering::Release);
}

unsafe fn parallel_combinations_of_slice_of_len_3(
    slice: &[Card],
    current: CardSet,
    output: SendSyncRawPtr<CardSet>,
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
                set -= slice[x3];
            }
            set -= slice[x2];
        }
        set -= slice[x1];
    }

    amount_done.fetch_add(n_choose_r(slice.len(), 3), Ordering::Release);
}

unsafe fn parallel_combinations_of_slice_of_len_4(
    slice: &[Card],
    current: CardSet,
    output: SendSyncRawPtr<CardSet>,
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
                    set -= slice[x4];
                }
                set -= slice[x3];
            }
            set -= slice[x2];
        }
        set -= slice[x1];
    }

    amount_done.fetch_add(n_choose_r(slice.len(), 4), Ordering::Release);
}

unsafe fn parallel_combinations_of_slice_of_len_5(
    slice: &[Card],
    current: CardSet,
    output: SendSyncRawPtr<CardSet>,
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
                        set -= slice[x5];
                    }
                    set -= slice[x4];
                }
                set -= slice[x3];
            }
            set -= slice[x2];
        }
        set -= slice[x1];
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

        let current_mut_ptr = SendSyncRawPtr { ptr: current_ptr };

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
    pub output: SendSyncRawPtr<CardSet>,
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
            output: SendSyncRawPtr {
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

pub fn assert_input_is_well_formed(pockets: &[CardSet], board: CardSet) {
    let _ = undealt_cards(pockets, board);
}

pub fn undealt_cards(pockets: &[CardSet], board: CardSet) -> CardSet {
    for pocket in pockets {
        if pocket.len() != 2 {
            panic!("All pockets must have 2 cards, but {} doesn't", pocket);
        }
    }

    if board.len() > 5 {
        panic!("The board cannot have more than 5 cards, but has {}", board);
    }

    let mut set = board;
    for pocket in pockets {
        let intersection = set & *pocket;
        if intersection.len() != 0 {
            panic!(
                "Cannot have duplicate cards, but {} appears multiple times",
                intersection.iter_desc().next().unwrap()
            )
        }
        set |= *pocket;
    }

    CardSet::universe() - set
}

pub fn all_seven_card_hands() -> Vec<CardSet> {
    combinations(CardSet::universe(), 7)
}

pub fn all_boards<const N_PLAYERS: usize>(pockets: &[CardSet; N_PLAYERS]) {
    if pockets.iter().any(|p| p.len() != 2) {
        panic!("All pockets must have 2 cards each");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
