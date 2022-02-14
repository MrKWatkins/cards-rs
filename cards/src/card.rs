use crate::{Rank, Suit};

#[derive(Debug, Default, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    #[must_use]
    pub const fn new(rank: Rank, suit: Suit) -> Card {
        return Card { rank, suit };
    }

    #[must_use]
    pub fn create_full_deck() -> [Card; 52] {
        let mut deck = [Card::default(); 52];
        let mut index = 0;
        for suit in 0..4 {
            for rank in 0..13 {
                deck[index] = Card::new(rank.into(), suit.into());
                index += 1;
            }
        }
        return deck;
    }

    #[must_use]
    pub fn from_index(index: u8) -> Card {
        return Card::new((index % 13).into(), (index / 13).into());
    }

    #[must_use]
    pub fn to_index(&self) -> u8 {
        return self.suit as u8 * 13 + self.rank as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn new() {
        let card = Card::new(Rank::Five, Suit::Diamonds);

        assert_eq!(card, Card { rank: Rank::Five, suit: Suit::Diamonds });
    }

    #[test]
    fn create_full_deck() {
        let full_deck = Card::create_full_deck();

        assert_eq!(full_deck.len(), 52);
        assert_eq!(full_deck[0], Card::new(Rank::Ace, Suit::Spades));
        assert_eq!(full_deck[12], Card::new(Rank::King, Suit::Spades));
        assert_eq!(full_deck[13], Card::new(Rank::Ace, Suit::Hearts));
        assert_eq!(full_deck[51], Card::new(Rank::King, Suit::Clubs));

        let set: HashSet<Card> = full_deck.into();
        assert_eq!(set.len(), 52);
    }

    #[test]
    fn from_index() {
        let deck_from_index = (0..52).map(|i| Card::from_index(i)).collect::<Vec<Card>>();

        assert_eq!(deck_from_index, Card::create_full_deck().to_vec());
    }

    #[test]
    fn to_index() {
        let full_deck = Card::create_full_deck();

        let actual = full_deck.map(|c| c.to_index());

        assert_eq!(actual.to_vec(), (0..52).collect::<Vec<u8>>());
    }
}
