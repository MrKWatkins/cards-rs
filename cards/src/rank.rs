use crate::text::{Format, IndexableFormat};
use crate::Indexable;
use lazy_static::lazy_static;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::fmt::{Display, Formatter};

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, IntoPrimitive,
)]
#[repr(u8)]
pub enum Rank {
    #[num_enum(default)]
    Ace = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
}

const UPPERCASE_VALUES: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

lazy_static! {
    static ref DEFAULT_FORMAT: IndexableFormat<'static, 13> =
        IndexableFormat::new(UPPERCASE_VALUES);
}

impl Rank {
    pub fn new_upper_case_format() -> Box<dyn Format<Rank>> {
        return Box::new(IndexableFormat::new(UPPERCASE_VALUES));
    }

    pub fn new_lower_case_format() -> Box<dyn Format<Rank>> {
        return Box::new(IndexableFormat::new([
            "a", "2", "3", "4", "5", "6", "7", "8", "9", "10", "j", "q", "k",
        ]));
    }
}

impl Default for Rank {
    fn default() -> Rank {
        return Rank::Ace;
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return DEFAULT_FORMAT.fmt(self, f);
    }
}

impl Indexable for Rank {
    fn from_index(index: u8) -> Rank {
        return Rank::from(index);
    }

    fn to_index(&self) -> u8 {
        return *self as u8;
    }

    fn maximum_index() -> u8 {
        return 12;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Rank::Ace.to_string(), "A");
        assert_eq!(Rank::Ten.to_string(), "10");
    }

    #[test]
    fn new_upper_case_format() {
        let format = Rank::new_upper_case_format();

        assert_eq!(format.format(&Rank::Ace), "A");
        assert_eq!(format.format(&Rank::Ten), "10");

        assert_eq!(format.parse("A").unwrap(), Rank::Ace);
        assert_eq!(format.parse("10").unwrap(), Rank::Ten);
    }

    #[test]
    fn new_lower_case_format() {
        let format = Rank::new_lower_case_format();

        assert_eq!(format.format(&Rank::Ace), "a");
        assert_eq!(format.format(&Rank::Ten), "10");

        assert_eq!(format.parse("a").unwrap(), Rank::Ace);
        assert_eq!(format.parse("10").unwrap(), Rank::Ten);
    }
}
