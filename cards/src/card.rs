use crate::text::{Format, IndexableFormat};
use crate::{Indexable, Rank, Suit};
use lazy_static::lazy_static;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

const UPPERCASE_VALUES: [&str; 52] = [
    "AS", "2S", "3S", "4S", "5S", "6S", "7S", "8S", "9S", "10S", "JS", "QS", "KS", "AH", "2H",
    "3H", "4H", "5H", "6H", "7H", "8H", "9H", "10H", "JH", "QH", "KH", "AD", "2D", "3D", "4D",
    "5D", "6D", "7D", "8D", "9D", "10D", "JD", "QD", "KD", "AC", "2C", "3C", "4C", "5C", "6C",
    "7C", "8C", "9C", "10C", "JC", "QC", "KC",
];

lazy_static! {
    static ref DEFAULT_FORMAT: IndexableFormat<'static, 52> =
        IndexableFormat::new(UPPERCASE_VALUES);
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
    pub fn new_upper_case_format() -> Box<dyn Format<Card>> {
        return Box::new(IndexableFormat::new(UPPERCASE_VALUES));
    }

    #[must_use]
    pub fn new_lower_case_format() -> Box<dyn Format<Card>> {
        return Box::new(IndexableFormat::new([
            "as", "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", "10s", "js", "qs", "ks", "ah",
            "2h", "3h", "4h", "5h", "6h", "7h", "8h", "9h", "10h", "jh", "qh", "kh", "ad", "2d",
            "3d", "4d", "5d", "6d", "7d", "8d", "9d", "10d", "jd", "qd", "kd", "ac", "2c", "3c",
            "4c", "5c", "6c", "7c", "8c", "9c", "10c", "jc", "qc", "kc",
        ]));
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return DEFAULT_FORMAT.fmt(self, f);
    }
}

impl Indexable for Card {
    fn from_index(index: u8) -> Card {
        return Card::new((index % 13).into(), (index / 13).into());
    }

    fn to_index(&self) -> u8 {
        return self.suit as u8 * 13 + self.rank as u8;
    }

    fn maximum_index() -> u8 {
        return 51;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn new() {
        let card = Card::new(Rank::Five, Suit::Diamonds);

        assert_eq!(
            card,
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds
            }
        );
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

    #[test]
    fn to_string() {
        assert_eq!(Card::new(Rank::Ace, Suit::Spades).to_string(), "AS");
        assert_eq!(Card::new(Rank::Ten, Suit::Diamonds).to_string(), "10D");
    }

    #[test]
    fn new_upper_case_format() {
        let format = Card::new_upper_case_format();

        assert_eq!(format.format(&Card::new(Rank::Ace, Suit::Spades)), "AS");
        assert_eq!(format.format(&Card::new(Rank::Ten, Suit::Diamonds)), "10D");

        assert_eq!(
            format.parse("AS").unwrap(),
            Card::new(Rank::Ace, Suit::Spades)
        );
        assert_eq!(
            format.parse("10D").unwrap(),
            Card::new(Rank::Ten, Suit::Diamonds)
        );
    }

    #[test]
    fn new_lower_case_format() {
        let format = Card::new_lower_case_format();

        assert_eq!(format.format(&Card::new(Rank::Ace, Suit::Spades)), "as");
        assert_eq!(format.format(&Card::new(Rank::Ten, Suit::Diamonds)), "10d");

        assert_eq!(
            format.parse("as").unwrap(),
            Card::new(Rank::Ace, Suit::Spades)
        );
        assert_eq!(
            format.parse("10d").unwrap(),
            Card::new(Rank::Ten, Suit::Diamonds)
        );
    }
}
