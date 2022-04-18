use crate::text::{Format, IndexableFormat};
use crate::Indexable;
use lazy_static::lazy_static;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::fmt::{Display, Formatter};

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, IntoPrimitive,
)]
#[repr(u8)]
pub enum Suit {
    #[num_enum(default)]
    Spades = 0,
    Hearts = 1,
    Diamonds = 2,
    Clubs = 3,
}

const UPPERCASE_VALUES: [&str; 4] = ["S", "H", "D", "C"];

lazy_static! {
    static ref DEFAULT_FORMAT: IndexableFormat<'static, 4> = IndexableFormat::new(UPPERCASE_VALUES);
}

impl Suit {
    pub fn new_upper_case_format() -> Box<dyn Format<Suit>> {
        return Box::new(IndexableFormat::new(UPPERCASE_VALUES));
    }

    pub fn new_lower_case_format() -> Box<dyn Format<Suit>> {
        return Box::new(IndexableFormat::new(["s", "h", "d", "c"]));
    }
}

impl Default for Suit {
    fn default() -> Suit {
        return Suit::Spades;
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return DEFAULT_FORMAT.fmt(self, f);
    }
}

impl Indexable for Suit {
    fn from_index(index: u8) -> Suit {
        return Suit::from(index);
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
        assert_eq!(Suit::Spades.to_string(), "S");
        assert_eq!(Suit::Diamonds.to_string(), "D");
    }

    #[test]
    fn new_upper_case_format() {
        let format = Suit::new_upper_case_format();

        assert_eq!(format.format(&Suit::Spades), "S");
        assert_eq!(format.format(&Suit::Diamonds), "D");

        assert_eq!(format.parse("S").unwrap(), Suit::Spades);
        assert_eq!(format.parse("D").unwrap(), Suit::Diamonds);
    }

    #[test]
    fn new_lower_case_format() {
        let format = Suit::new_lower_case_format();

        assert_eq!(format.format(&Suit::Spades), "s");
        assert_eq!(format.format(&Suit::Diamonds), "d");

        assert_eq!(format.parse("s").unwrap(), Suit::Spades);
        assert_eq!(format.parse("d").unwrap(), Suit::Diamonds);
    }
}
