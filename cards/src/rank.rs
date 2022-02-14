use num_enum::{FromPrimitive, IntoPrimitive};

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

impl Default for Rank {
    fn default() -> Rank {
        return Rank::Ace;
    }
}
