use crate::analysis::odds::calculate_odds;
use crate::util::ui::{parse_input, print_odds};

mod analysis;
mod cards;
mod datastructures;
mod parallelism;
mod test_util;
mod util;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let input = parse_input(&args[1..]);

    let odds = calculate_odds(&input.pockets, input.board);
    for player in odds {
        print_odds(player);
        println!();
    }
}
