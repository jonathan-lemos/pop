use std::process::ExitCode;

use crate::cards::card::{Card, Rank, Suit};
use crate::cards::cardset::CardSet;
use crate::operations::showdown::Showdown;
use crate::ui::argparser::{ArgStream, TakeMode};
use crate::ui::output::{print_help, print_unrecognized_operation};
use crate::util::array::monomorphize;

pub fn stream_card(take_char: &mut dyn FnMut(TakeMode) -> Option<char>) -> Option<Card> {
    let rank = match take_char(TakeMode::Consume)?.to_ascii_uppercase() {
        '2' => Rank::Two,
        '3' => Rank::Three,
        '4' => Rank::Four,
        '5' => Rank::Five,
        '6' => Rank::Six,
        '7' => Rank::Seven,
        '8' => Rank::Eight,
        '9' => Rank::Nine,
        'T' => Rank::Ten,
        'J' => Rank::Jack,
        'Q' => Rank::Queen,
        'K' => Rank::King,
        'A' => Rank::Ace,
        '1' => match take_char(TakeMode::Consume)? {
            '0' => Rank::Ten,
            _ => return None,
        },
        _ => return None,
    };

    let suit = match take_char(TakeMode::Consume)?.to_ascii_lowercase() {
        'c' => Suit::Club,
        '♣' => Suit::Club,
        'd' => Suit::Diamond,
        '♦' => Suit::Diamond,
        'h' => Suit::Heart,
        '♥' => Suit::Heart,
        's' => Suit::Spade,
        '♠' => Suit::Spade,
        _ => return None,
    };

    Some(Card { rank, suit })
}

pub fn stream_literal_ignorecase(
    literal: &str,
    take_char: &mut dyn FnMut(TakeMode) -> Option<char>,
) -> Option<String> {
    for char in literal.chars() {
        let next = take_char(TakeMode::Consume)?;
        if next != char {
            return None;
        }
    }
    Some(literal.to_string())
}

pub fn stream_whitespace(take_char: &mut dyn FnMut(TakeMode) -> Option<char>) -> Option<String> {
    let mut ret = String::new();
    while let Some(c) = take_char(TakeMode::Peek) {
        if !c.is_whitespace() {
            return if ret.is_empty() { None } else { Some(ret) };
        }
        ret.push(c);
        take_char(TakeMode::Consume);
    }
    if ret.is_empty() { None } else { Some(ret) }
}

pub fn stream_token(take_char: &mut dyn FnMut(TakeMode) -> Option<char>) -> Option<String> {
    let mut ret = String::new();
    while let Some(c) = take_char(TakeMode::Peek) {
        if c.is_whitespace() {
            return if ret.is_empty() { None } else { Some(ret) };
        }
        ret.push(c);
        take_char(TakeMode::Consume);
    }
    if ret.is_empty() { None } else { Some(ret) }
}

pub fn stream_drain(take_char: &mut dyn FnMut(TakeMode) -> Option<char>) -> Option<String> {
    let mut ret = String::new();
    while let Some(c) = take_char(TakeMode::Consume) {
        ret.push(c);
    }
    Some(ret)
}

pub fn parse_pockets(stream: &mut ArgStream) -> Result<Vec<CardSet>, ExitCode> {
    let mut pockets = Vec::<CardSet>::new();

    loop {
        if !pockets.is_empty() {
            match stream.try_parse(|t| stream_literal_ignorecase("vs", t)) {
                Some(_) => {}
                None => {
                    break;
                }
            }
            stream.try_parse(stream_whitespace);
        }

        let first_card = match stream.try_parse(stream_card) {
            Some(c) => c,
            None => {
                println!(
                    "Expected a card, but got {}",
                    stream.try_parse(stream_token).unwrap_or("EOF".to_string())
                );
                return Err(ExitCode::FAILURE);
            }
        };

        stream.try_parse(stream_whitespace);
        stream.try_parse(|t| stream_literal_ignorecase(",", t));
        stream.try_parse(stream_whitespace);

        let second_card = match stream.try_parse(stream_card) {
            Some(c) => c,
            None => {
                println!(
                    "Each pocket must have 2 cards (got {})",
                    stream.try_parse(stream_token).unwrap_or("EOF".to_string())
                );
                return Err(ExitCode::FAILURE);
            }
        };

        stream.try_parse(stream_whitespace);

        pockets.push(CardSet::from(&[first_card, second_card]));
    }

    if pockets.len() > 23 {
        println!("Cannot have more than 23 pockets (have {})", pockets.len());
        Err(ExitCode::FAILURE)
    } else if pockets.len() < 2 {
        println!("Cannot have less than 2 pockets (have {})", pockets.len());
        Err(ExitCode::FAILURE)
    } else {
        Ok(pockets)
    }
}

pub fn parse_board(stream: &mut ArgStream) -> Result<CardSet, ExitCode> {
    let mut board = CardSet::new();

    loop {
        let empty_is_acceptable = if !board.is_empty() {
            let comma_present = stream
                .try_parse(|t| stream_literal_ignorecase(",", t))
                .is_some();
            stream.try_parse(stream_whitespace);
            !comma_present
        } else {
            true
        };

        let card = match stream.try_parse(stream_card) {
            Some(c) => c,
            None => {
                if empty_is_acceptable {
                    break;
                } else {
                    println!(
                        "Expected a card, but got {}",
                        stream.try_parse(stream_token).unwrap_or("EOF".to_string())
                    );
                    return Err(ExitCode::FAILURE);
                }
            }
        };

        if board.has(card) {
            println!("Cannot have duplicate cards ({})", card);
            return Err(ExitCode::FAILURE);
        }

        board += card;
    }

    if board.len() > 5 {
        println!(
            "The board cannot have more than 5 cards (has {}: {})",
            board.len(),
            board
        );
        return Err(ExitCode::FAILURE);
    }

    Ok(board)
}

pub fn parse_showdown(mut stream: &mut ArgStream) -> Result<Showdown, ExitCode> {
    let pockets = parse_pockets(&mut stream)?;
    stream.try_parse(stream_whitespace);
    let board = match stream.try_parse(|t| stream_literal_ignorecase("on", t)) {
        Some(_) => parse_board(&mut stream)?,
        None => CardSet::new(),
    };

    Ok(Showdown {
        pockets: monomorphize(pockets.into_iter()).unwrap(),
        board: board,
    })
}

pub fn parse_input<I: Iterator<Item = String>>(args: I) -> Result<Showdown, ExitCode> {
    let mut args = args.peekable();

    let executable_name = match args.next() {
        Some(arg) => arg,
        None => "pop".to_string(),
    };

    let operation = match args.next() {
        Some(op) => op,
        None => {
            print_help(&executable_name, None);
            return Err(ExitCode::SUCCESS);
        }
    };

    if args.peek() == Some(&"--help".to_string()) || args.peek() == Some(&"-h".to_string()) {
        print_help(executable_name.as_str(), Some(operation.as_str()));
        return Err(ExitCode::SUCCESS);
    }

    let mut stream = ArgStream::from(args);

    let value = match operation.as_str() {
        "showdown" => parse_showdown(&mut stream),
        _ => {
            print_unrecognized_operation(&executable_name, &operation);
            Err(ExitCode::FAILURE)
        }
    };

    if stream.is_empty() {
        value
    } else {
        println!(
            "Unexpected trailing input: '{}'",
            stream.try_parse(stream_drain).unwrap()
        );
        Err(ExitCode::FAILURE)
    }
}
