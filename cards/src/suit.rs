use num_enum::{FromPrimitive, IntoPrimitive};

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

impl Default for Suit {
    fn default() -> Suit {
        return Suit::Spades;
    }
}
