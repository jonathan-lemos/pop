#![allow(dead_code)]
use std::process::ExitCode;

use crate::operations::showdown::{calculate_odds_from_showdown, print_odds};
use crate::ui::input::parse_input;

mod analysis;
mod cards;
mod datastructures;
mod operations;
mod parallelism;
mod ui;
mod util;

fn main() -> ExitCode {
    let input = match parse_input(std::env::args()) {
        Ok(v) => v,
        Err(code) => return code,
    };

    let odds = calculate_odds_from_showdown(&input);
    for player in odds.iter() {
        print_odds(player);
        println!();
    }

    ExitCode::SUCCESS
}
