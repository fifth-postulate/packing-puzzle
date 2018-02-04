use super::piece::Template;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Bag {
    collection: Vec<Template>,
}

impl Bag {
    pub fn new(collection: Vec<Template>) -> Self {
        Self { collection }
    }
}

impl IntoIterator for Bag {
    type Item = (Template, Bag);
    type IntoIter = BagSelectionIterator;

    fn into_iter(self) -> Self::IntoIter {
        BagSelectionIterator::new(self)
    }
}

pub struct BagSelectionIterator {
    collection: Vec<Template>,
    index: usize,
}

impl BagSelectionIterator {
    fn new(bag: Bag) -> BagSelectionIterator {
        BagSelectionIterator { index: 0, collection: bag.collection }
    }
}

impl Iterator for BagSelectionIterator {
    type Item = (Template, Bag);

    fn next(&mut self) -> Option<(Template, Bag)> {
        if self.index < self.collection.len() {
            let mut collection: Vec<Template> = self.collection.iter().cloned().collect();
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
