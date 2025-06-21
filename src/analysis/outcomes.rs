use crate::cards::cardset::CardSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcomes {
    pub wins: usize,
    pub losses: usize,
    pub draws_with_n_other_players: Vec<usize>,
}

impl Outcomes {
    pub fn evaluate(players: &[CardSet]) -> Option<Outcomes> {
        if players.len() > 23 {
            return None;
        }
        todo!()
    }
}
