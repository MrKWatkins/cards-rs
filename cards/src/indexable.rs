pub trait Indexable {
    #[must_use]
    fn from_index(index: u8) -> Self;

    #[must_use]
    fn to_index(&self) -> u8;

    #[must_use]
    fn maximum_index() -> u8;
}
