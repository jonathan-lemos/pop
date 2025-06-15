use rand::Rng;

use crate::cards::card::{ALL_CARDS, Card};

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    // Creates a deck with all 52 cards in it.
    pub fn new() -> Deck {
        return Deck {
            cards: ALL_CARDS.to_vec(),
        };
    }

    // Returns a random remaining card in the deck, using the given RNG.
    //
    // Panics if the deck is empty.
    pub fn deal<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Card {
        let index = rng.random_range(0..self.cards.len());
        return self.cards.swap_remove(index);
    }

    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }

    // The number of remaining cards in the deck.
    pub fn len(&self) -> usize {
        return self.cards.len();
    }

    pub fn num_satisfying<P: FnMut(Card) -> bool>(&self, mut predicate: P) -> usize {
        let mut count = 0;
        for card in &self.cards {
            if predicate(*card) {
                count += 1;
            }
        }
        return count;
    }

    // Removes a specific card from the deck,
    // or does nothing if that card wasn't present.
    //
    // Returns true if and only if the card was present.
    pub fn remove_card(&mut self, card: Card) -> bool {
        if let Some(index) = self.cards.iter().position(|c| *c == card) {
            self.cards.swap_remove(index);
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cards::card::*, cards::deck::*, test_util::zero_rng::ZeroRng};
    use std::collections::HashSet;

    #[test]
    fn test_deal_entire_deck() {
        let expected = HashSet::from(ALL_CARDS);

        let mut deck = Deck::new();
        let mut actual = HashSet::new();
        let mut rng = ZeroRng::new();

        while !deck.is_empty() {
            actual.insert(deck.deal(&mut rng));
        }

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_len() {
        let mut deck = Deck::new();
        let mut rng = ZeroRng::new();

        assert_eq!(deck.len(), 52);

        deck.deal(&mut rng);
        deck.deal(&mut rng);
        assert_eq!(deck.len(), 50);
    }

    #[test]
    fn test_num_satisfying() {
        let ace_of_spades = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };
        let mut deck = Deck::new();

        deck.remove_card(ace_of_spades);

        assert_eq!(deck.num_satisfying(|c| c.suit == Suit::Heart), 13);
        assert_eq!(deck.num_satisfying(|c| c.suit == Suit::Spade), 12);
    }

    #[test]
    fn test_remove_card() {
        let card1 = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };
        let card2 = Card {
            rank: Rank::Queen,
            suit: Suit::Heart,
        };

        let mut expected = HashSet::from(ALL_CARDS);
        expected.remove(&card1);
        expected.remove(&card2);

        let mut actual = HashSet::new();
        let mut rng = ZeroRng::new();

        let mut deck = Deck::new();
        assert!(deck.remove_card(card1));
        assert!(!deck.remove_card(card1));
        assert!(deck.remove_card(card2));

        while !deck.is_empty() {
            actual.insert(deck.deal(&mut rng));
        }

        assert_eq!(expected, actual);
    }
}
