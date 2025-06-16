use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(u8)]
pub enum Suit {
    Club = 0,
    Diamond = 1,
    Heart = 2,
    Spade = 3,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        })
    }
}

pub const NUM_SUITS: usize = 4;

pub const ALL_SUITS: [Suit; NUM_SUITS] = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[repr(u8)]
pub enum Rank {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        })
    }
}

pub const NUM_RANKS: usize = 13;

pub const ALL_RANKS: [Rank; NUM_RANKS] = [
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.rank.fmt(f)?;
        self.suit.fmt(f)
    }
}

impl Card {
    pub const TWO_CLUB: Card = Card {
        rank: Rank::Two,
        suit: Suit::Club,
    };
    pub const THREE_CLUB: Card = Card {
        rank: Rank::Three,
        suit: Suit::Club,
    };
    pub const FOUR_CLUB: Card = Card {
        rank: Rank::Four,
        suit: Suit::Club,
    };
    pub const FIVE_CLUB: Card = Card {
        rank: Rank::Five,
        suit: Suit::Club,
    };
    pub const SIX_CLUB: Card = Card {
        rank: Rank::Six,
        suit: Suit::Club,
    };
    pub const SEVEN_CLUB: Card = Card {
        rank: Rank::Seven,
        suit: Suit::Club,
    };
    pub const EIGHT_CLUB: Card = Card {
        rank: Rank::Eight,
        suit: Suit::Club,
    };
    pub const NINE_CLUB: Card = Card {
        rank: Rank::Nine,
        suit: Suit::Club,
    };
    pub const TEN_CLUB: Card = Card {
        rank: Rank::Ten,
        suit: Suit::Club,
    };
    pub const JACK_CLUB: Card = Card {
        rank: Rank::Jack,
        suit: Suit::Club,
    };
    pub const QUEEN_CLUB: Card = Card {
        rank: Rank::Queen,
        suit: Suit::Club,
    };
    pub const KING_CLUB: Card = Card {
        rank: Rank::King,
        suit: Suit::Club,
    };
    pub const ACE_CLUB: Card = Card {
        rank: Rank::Ace,
        suit: Suit::Club,
    };
    pub const TWO_DIAMOND: Card = Card {
        rank: Rank::Two,
        suit: Suit::Diamond,
    };
    pub const THREE_DIAMOND: Card = Card {
        rank: Rank::Three,
        suit: Suit::Diamond,
    };
    pub const FOUR_DIAMOND: Card = Card {
        rank: Rank::Four,
        suit: Suit::Diamond,
    };
    pub const FIVE_DIAMOND: Card = Card {
        rank: Rank::Five,
        suit: Suit::Diamond,
    };
    pub const SIX_DIAMOND: Card = Card {
        rank: Rank::Six,
        suit: Suit::Diamond,
    };
    pub const SEVEN_DIAMOND: Card = Card {
        rank: Rank::Seven,
        suit: Suit::Diamond,
    };
    pub const EIGHT_DIAMOND: Card = Card {
        rank: Rank::Eight,
        suit: Suit::Diamond,
    };
    pub const NINE_DIAMOND: Card = Card {
        rank: Rank::Nine,
        suit: Suit::Diamond,
    };
    pub const TEN_DIAMOND: Card = Card {
        rank: Rank::Ten,
        suit: Suit::Diamond,
    };
    pub const JACK_DIAMOND: Card = Card {
        rank: Rank::Jack,
        suit: Suit::Diamond,
    };
    pub const QUEEN_DIAMOND: Card = Card {
        rank: Rank::Queen,
        suit: Suit::Diamond,
    };
    pub const KING_DIAMOND: Card = Card {
        rank: Rank::King,
        suit: Suit::Diamond,
    };
    pub const ACE_DIAMOND: Card = Card {
        rank: Rank::Ace,
        suit: Suit::Diamond,
    };
    pub const TWO_HEART: Card = Card {
        rank: Rank::Two,
        suit: Suit::Heart,
    };
    pub const THREE_HEART: Card = Card {
        rank: Rank::Three,
        suit: Suit::Heart,
    };
    pub const FOUR_HEART: Card = Card {
        rank: Rank::Four,
        suit: Suit::Heart,
    };
    pub const FIVE_HEART: Card = Card {
        rank: Rank::Five,
        suit: Suit::Heart,
    };
    pub const SIX_HEART: Card = Card {
        rank: Rank::Six,
        suit: Suit::Heart,
    };
    pub const SEVEN_HEART: Card = Card {
        rank: Rank::Seven,
        suit: Suit::Heart,
    };
    pub const EIGHT_HEART: Card = Card {
        rank: Rank::Eight,
        suit: Suit::Heart,
    };
    pub const NINE_HEART: Card = Card {
        rank: Rank::Nine,
        suit: Suit::Heart,
    };
    pub const TEN_HEART: Card = Card {
        rank: Rank::Ten,
        suit: Suit::Heart,
    };
    pub const JACK_HEART: Card = Card {
        rank: Rank::Jack,
        suit: Suit::Heart,
    };
    pub const QUEEN_HEART: Card = Card {
        rank: Rank::Queen,
        suit: Suit::Heart,
    };
    pub const KING_HEART: Card = Card {
        rank: Rank::King,
        suit: Suit::Heart,
    };
    pub const ACE_HEART: Card = Card {
        rank: Rank::Ace,
        suit: Suit::Heart,
    };
    pub const TWO_SPADE: Card = Card {
        rank: Rank::Two,
        suit: Suit::Spade,
    };
    pub const THREE_SPADE: Card = Card {
        rank: Rank::Three,
        suit: Suit::Spade,
    };
    pub const FOUR_SPADE: Card = Card {
        rank: Rank::Four,
        suit: Suit::Spade,
    };
    pub const FIVE_SPADE: Card = Card {
        rank: Rank::Five,
        suit: Suit::Spade,
    };
    pub const SIX_SPADE: Card = Card {
        rank: Rank::Six,
        suit: Suit::Spade,
    };
    pub const SEVEN_SPADE: Card = Card {
        rank: Rank::Seven,
        suit: Suit::Spade,
    };
    pub const EIGHT_SPADE: Card = Card {
        rank: Rank::Eight,
        suit: Suit::Spade,
    };
    pub const NINE_SPADE: Card = Card {
        rank: Rank::Nine,
        suit: Suit::Spade,
    };
    pub const TEN_SPADE: Card = Card {
        rank: Rank::Ten,
        suit: Suit::Spade,
    };
    pub const JACK_SPADE: Card = Card {
        rank: Rank::Jack,
        suit: Suit::Spade,
    };
    pub const QUEEN_SPADE: Card = Card {
        rank: Rank::Queen,
        suit: Suit::Spade,
    };
    pub const KING_SPADE: Card = Card {
        rank: Rank::King,
        suit: Suit::Spade,
    };
    pub const ACE_SPADE: Card = Card {
        rank: Rank::Ace,
        suit: Suit::Spade,
    };
}

pub const NUM_CARDS: usize = NUM_RANKS * NUM_SUITS;

const fn gen_all_cards() -> [Card; NUM_CARDS] {
    let mut ret = [Card {
        rank: Rank::Two,
        suit: Suit::Club,
    }; 52];
    let mut rank_index = 0;
    let mut suit_index = 0;

    while rank_index * 4 + suit_index < NUM_CARDS {
        ret[rank_index * 4 + suit_index] = Card {
            rank: ALL_RANKS[rank_index],
            suit: ALL_SUITS[suit_index],
        };
        suit_index += 1;
        if suit_index >= 4 {
            suit_index = 0;
            rank_index += 1;
        }
    }

    return ret;
}

pub const ALL_CARDS: [Card; NUM_CARDS] = gen_all_cards();

#[cfg(test)]
mod tests {
    use crate::cards::card::ALL_CARDS;
    use std::collections::HashSet;

    #[test]
    fn test_all_cards_are_unique() {
        assert_eq!(HashSet::from(ALL_CARDS).len(), ALL_CARDS.len());
    }

    #[test]
    fn test_all_cards_has_52_cards() {
        assert_eq!(ALL_CARDS.len(), 52);
    }
}
