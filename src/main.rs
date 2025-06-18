use crate::analysis::search_space::{all_seven_card_hands, all_seven_card_hands_legacy};

mod analysis;
mod cards;
mod datastructures;
mod parallelism;
mod test_util;
mod util;

fn main() {
    all_seven_card_hands();
}
