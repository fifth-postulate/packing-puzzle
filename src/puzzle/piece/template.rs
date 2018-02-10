//! template is a container to hold orientations of pieces.

use std::convert::From;
use super::{Position, Normalizable, Piece, CubeSymmetryIterator, Translatable, Transformable, MinimumPosition};

/// A `Template` is a container to hold a representation of a `Piece`. By
/// Iterating over a one gets a piece in all the possible orientations.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Template {
    positions: Vec<Position<(i8, i8, i8)>>,
    name: Option<String>
}

impl Template {
    /// Create a `Template` from a vector of `Position`s.
    pub fn new(positions: Vec<Position<(i8, i8, i8)>>) -> Template {
        Template { positions, name: None }
    }

    /// Create a named `Template` from this `Template`
    pub fn with_name<S>(self, name: S) -> Template where S : Into<String> {
        let name = Some(name.into());

        Template { positions: self.positions, name }
    }
}

impl IntoIterator for Template {
    type Item = Piece<(i8, i8, i8)>;
    type IntoIter = PieceIterator<(i8, i8, i8)>;

    fn into_iter(self) -> Self::IntoIter {
        PieceIterator::new(self)
    }
}


/// The `PieceIterator` will return `Piece`s  in all the orientations possible
/// from a `Template`
pub struct PieceIterator<T> {
    symmetry_iterator: CubeSymmetryIterator,
    seen_pieces: Vec<Piece<T>>,
    template: Template,
}

impl<T> PieceIterator<T> {
    /// Creates a `PieceIterator` for the `Template` that is passed as an argument
    pub fn new(template: Template) -> PieceIterator<T> {
        PieceIterator {
            symmetry_iterator: CubeSymmetryIterator::new(),
            seen_pieces: vec!(),
            template,
        }
    }
}

impl Iterator for PieceIterator<(i8, i8, i8)> {
    type Item = Piece<(i8, i8, i8)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut symmetry_option = self.symmetry_iterator.next();
        while symmetry_option.is_some() {
            let piece_option = symmetry_option.map(|symmetry|{
                let mut piece = Piece::from(self.template.clone());

                piece.transform(&symmetry);
                let minimum_position = piece.minimum_position();
                let translation = minimum_position.unwrap().to_reference();
                piece.translate(&translation);

                piece
            });

            if piece_option.is_some() {
                let piece = piece_option.unwrap();
                let clone = piece.clone();
                if !self.seen_pieces.contains(&clone) {
                    self.seen_pieces.push(clone);

                    return Some(piece)
                }
            }

            symmetry_option = self.symmetry_iterator.next();
        }
        None
    }
}

impl From<Template> for Piece<(i8, i8, i8)> {
    fn from(template: Template) -> Self {
        if template.name.is_some() {
            Piece::named(template.positions, template.name.unwrap())
        } else {
            Piece::new(template.positions)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::Iterator;
    use super::*;

    #[test]
    fn templates_are_equal_on_values() {
        let a = Template::new(vec!(
            Position::new(0,0,0),
            Position::new(1,0,0),
        ));
        let b = Template::new(vec!(
            Position::new(0,0,0),
            Position::new(1,0,0),
        ));

        assert_eq!(a, b);
    }


    #[test]
    fn templates_should_return_24_pieces_unsymmetric_templates() {
        let template = Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(1, 1, 1),
            Position::new(1, 1, 2),
        ));

        let iterator: PieceIterator<(i8, i8, i8)> = template.into_iter();

        assert_eq!(iterator.count(), 24);
    }

    #[test]
    fn templates_should_return_less_than_24_pieces_for_symmetric_templates() {
        let template = Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(0, 1, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
        ));

        let iterator: PieceIterator<(i8, i8, i8)>= template.into_iter();

        assert_eq!(iterator.count(), 3);
    }
}
