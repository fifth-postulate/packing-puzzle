//! Containers that can dispense `Template`s.

use super::piece::Template;

/// A container for `Template`s. Iterating over a `Bag` provides access to a
/// tuple of a `Template` and the rest of the `Bag`.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Bag<T> {
    collection: Vec<Template<T>>,
}

impl<T> Bag<T> {
    /// Create a `Bag` from a collection of Templates
    pub fn new(collection: Vec<Template<T>>) -> Self {
        Self { collection }
    }
}

impl<T> IntoIterator for Bag<T> where T: Clone {
    type Item = (Template<T>, Bag<T>);
    type IntoIter = BagSelectionIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        BagSelectionIterator::new(self)
    }
}


/// Iterator over tuples of `Template`s and rest of `Bag`s.
pub struct BagSelectionIterator<T> {
    collection: Vec<Template<T>>,
    index: usize,
}

impl<T> BagSelectionIterator<T> {
    fn new(bag: Bag<T>) -> BagSelectionIterator<T> {
        BagSelectionIterator { index: 0, collection: bag.collection }
    }
}

impl<T> Iterator for BagSelectionIterator<T> where T: Clone {
    type Item = (Template<T>, Bag<T>);

    fn next(&mut self) -> Option<(Template<T>, Bag<T>)> {
        if self.index < self.collection.len() {
            let mut collection: Vec<Template<T>> = self.collection.to_vec();
            let template = collection.swap_remove(self.index);
            self.index += 1;
            Some((template.clone(), Bag::new(collection)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::IntoIterator;
    use super::super::piece::{Template, Position};
    use super::*;
    #[test]
    fn bag_should_iterate_over_collection() {
        let bag = Bag::new(vec!(
            Template::new(vec!(Position::new(0, 0, 0), Position::new(1, 0, 0))),
            Template::new(vec!(Position::new(0, 0, 0))),
        ));

        let mut iterator = bag.into_iter();
        let candidate = iterator.next();
        assert!(candidate.is_some());
        let (template, rest) = candidate.unwrap();
        assert_eq!(template, Template::new(vec!(Position::new(0, 0, 0), Position::new(1, 0, 0))));
        assert_eq!(rest, Bag::new(vec!(Template::new(vec!(Position::new(0, 0, 0))))));

        let candidate = iterator.next();
        assert!(candidate.is_some());
        let (template, rest) = candidate.unwrap();
        assert_eq!(template, Template::new(vec!(Position::new(0, 0, 0))));
        assert_eq!(rest, Bag::new(vec!(
            Template::new(vec!(
                Position::new(0, 0, 0),
                Position::new(1, 0, 0))))));

    }
}
