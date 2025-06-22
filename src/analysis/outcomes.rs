use std::cmp::Reverse;
use std::num::NonZero;

use crate::analysis::evaluate_hand::HandEvaluation;
use crate::cards::cardset::CardSet;
use crate::parallelism::algorithms::{into_parallel_reduce, parallel_map};
use crate::util::array::{array_map, indexes, into_array_zip};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct Outcome<const N_PLAYERS: usize> {
    pub draws_with: [usize; N_PLAYERS],
    pub losses: usize,
}

impl<const N_PLAYERS: usize> Outcome<N_PLAYERS> {
    pub fn evaluate(
        players: &[CardSet; N_PLAYERS],
        boards: &[CardSet],
    ) -> Option<[Outcome<N_PLAYERS>; N_PLAYERS]> {
        const { assert!(N_PLAYERS >= 2 && N_PLAYERS <= 23) }

        if players.iter().any(|x| x.len() != 2) {
            return None;
        }

        if boards.iter().any(|x| x.len() != 5) {
            return None;
        }

        if let Some(player_set) = CardSet::union_if_disjoint(players) {
            if boards.iter().any(|board| !player_set.disjoint_with(*board)) {
                return None;
            }
        } else {
            return None;
        }

        let outcomes = parallel_map(boards, |board| {
            let hand_evals = array_map(players, |pocket| {
                let hand = *board | *pocket;
                HandEvaluation::evaluate_postflop(hand).unwrap()
            });

            let mut outcomes = [Outcome {
                losses: 0,
                draws_with: [0; N_PLAYERS],
            }; N_PLAYERS];

            let mut indexes = indexes::<N_PLAYERS>();
            indexes.sort_unstable_by_key(|i| Reverse(hand_evals[*i]));

            let mut draw_len = NonZero::new(1);

            for i in 1..indexes.len() {
                if let Some(len) = draw_len {
                    if hand_evals[indexes[i]] == hand_evals[indexes[i - 1]] {
                        draw_len = Some(NonZero::new(len.get() + 1).unwrap());
                    } else {
                        for j in 0..len.get() {
                            outcomes[indexes[j]].draws_with[len.get() - 1] = 1;
                        }
                        draw_len = None;
                        outcomes[indexes[i]].losses = 1;
                    }
                } else {
                    outcomes[indexes[i]].losses = 1;
                }
            }
            if let Some(_) = draw_len {
                for outcome in &mut outcomes {
                    outcome.draws_with[N_PLAYERS - 1] = 1;
                }
            }

            outcomes
        });

        into_parallel_reduce(outcomes, |a, c| {
            into_array_zip(a, c, |mut x, y| {
                x.draws_with = into_array_zip(x.draws_with, y.draws_with, |t, u| t + u);
                x.losses += y.losses;

                x
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::card::Card;

    use super::*;

    #[test]
    fn test_outcomes_heads_up() {
        let players = &[
            CardSet::from(&[Card::ACE_SPADE, Card::KING_SPADE]),
            CardSet::from(&[Card::JACK_CLUB, Card::JACK_HEART]),
        ];

        let boards = &[
            // JJ wins
            CardSet::from(&[
                Card::EIGHT_HEART,
                Card::TWO_CLUB,
                Card::NINE_SPADE,
                Card::JACK_DIAMOND,
                Card::QUEEN_HEART,
            ]),
            // AKs wins
            CardSet::from(&[
                Card::EIGHT_HEART,
                Card::TWO_CLUB,
                Card::NINE_SPADE,
                Card::ACE_DIAMOND,
                Card::QUEEN_HEART,
            ]),
            // AKs wins
            CardSet::from(&[
                Card::EIGHT_HEART,
                Card::TWO_CLUB,
                Card::NINE_SPADE,
                Card::KING_DIAMOND,
                Card::QUEEN_HEART,
            ]),
            // both tie
            CardSet::from(&[
                Card::ACE_DIAMOND,
                Card::KING_DIAMOND,
                Card::QUEEN_DIAMOND,
                Card::JACK_DIAMOND,
                Card::TEN_DIAMOND,
            ]),
        ];

        let outcomes = Outcome::evaluate(players, boards).unwrap();

        let expected = [
            Outcome {
                draws_with: [2, 1],
                losses: 1,
            },
            Outcome {
                draws_with: [1, 1],
                losses: 2,
            },
        ];

        assert_eq!(expected, outcomes);
    }

    #[test]
    fn test_outcomes_three_way() {
        let players = &[
            CardSet::from(&[Card::TEN_CLUB, Card::NINE_CLUB]),
            CardSet::from(&[Card::NINE_SPADE, Card::EIGHT_SPADE]),
            CardSet::from(&[Card::ACE_HEART, Card::KING_HEART]),
        ];

        let boards = &[
            // AKs wins
            CardSet::from(&[
                Card::TWO_CLUB,
                Card::THREE_DIAMOND,
                Card::FOUR_SPADE,
                Card::FIVE_HEART,
                Card::SEVEN_CLUB,
            ]),
            // T9s and 98s tie
            CardSet::from(&[
                Card::EIGHT_DIAMOND,
                Card::TEN_SPADE,
                Card::JACK_DIAMOND,
                Card::SEVEN_DIAMOND,
                Card::ACE_SPADE,
            ]),
            // All tie
            CardSet::from(&[
                Card::ACE_DIAMOND,
                Card::KING_DIAMOND,
                Card::QUEEN_DIAMOND,
                Card::JACK_DIAMOND,
                Card::TEN_DIAMOND,
            ]),
            // 98s wins
            CardSet::from(&[
                Card::FIVE_DIAMOND,
                Card::SIX_DIAMOND,
                Card::SEVEN_DIAMOND,
                Card::TWO_CLUB,
                Card::THREE_HEART,
            ]),
        ];

        let outcomes = Outcome::evaluate(players, boards).unwrap();

        let expected = [
            Outcome {
                draws_with: [0, 1, 1],
                losses: 2,
            },
            Outcome {
                draws_with: [1, 1, 1],
                losses: 1,
            },
            Outcome {
                draws_with: [1, 0, 1],
                losses: 2,
            },
        ];

        assert_eq!(expected, outcomes);
    }
}
