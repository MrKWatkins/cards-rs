use crate::Indexable;
use std::collections::HashMap;
use std::fmt::Formatter;

pub trait Format<T> {
    #[must_use]
    fn format(&self, value: &T) -> &str;

    fn fmt(&self, value: &T, f: &mut Formatter<'_>) -> std::fmt::Result;

    #[must_use]
    fn parse(&self, string: &str) -> Result<T, ()>;
}

pub struct IndexableFormat<'a, const N: usize> {
    values: [&'a str; N],
    lookup: HashMap<String, u8>,
}

impl<'a, const N: usize> IndexableFormat<'a, N> {
    pub fn new(values: [&str; N]) -> IndexableFormat<N> {
        let mut lookup = HashMap::new();

        for f in 0..N {
            lookup.insert(values[f].to_string(), f as u8);
        }

        return IndexableFormat { lookup, values };
    }
}

impl<'a, T: Indexable, const N: usize> Format<T> for IndexableFormat<'a, N> {
    fn format(&self, value: &T) -> &str {
        return &self.values[value.to_index() as usize];
    }

    fn fmt(&self, value: &T, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format(value))
    }

    fn parse(&self, string: &str) -> Result<T, ()> {
        match self.lookup.get(string) {
            Some(index) => Ok(T::from_index(*index)),
            None => Err(()),
        }
    }
}
