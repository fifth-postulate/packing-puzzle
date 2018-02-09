//! Describes objects to be packed.
//!
//! At the moment only objects that are aligned with an ordinary rectangular grid can be defined.

mod symmetry;
mod translation;
mod position;
mod template;

pub use self::symmetry::{Transformable, CubeSymmetry, CubeSymmetryIterator};
pub use self::translation::{Translatable, Translation};
pub use self::position::{Position, Positionable, MinimumPosition};
pub use self::template::Template;

use super::vector::VectorAdd;
use std::fmt::{Display, Formatter, Error};

/// Entities that get packed.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Piece<T> {
    positions: Vec<Position<T>>,
    name: Option<String>
}

impl<T> Piece<T> where T: PartialOrd + Ord + Clone {
    /// Create a new `Piece` from a collection of `Position`s.
    pub fn new(mut positions: Vec<Position<T>>) -> Piece<T> {
        positions.sort();
        Piece { positions, name: None }
    }

    /// Determine if a `Position` is contained in this `Piece`.
    pub fn contains(&self, position: &Position<T>) -> bool {
        self.positions.contains(position)
    }

    /// Create an `Iterator` that iterates over all `Position`s.
    pub fn iter(&self) -> PositionIterator<T> {
        let positions: Vec<Position<T>> = self.positions.to_vec();
        PositionIterator::new(positions)
    }
}

impl Display for Piece<(i8,i8,i8)> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[")?;
        let name = self.name.clone().unwrap_or_else(|| String::from(""));
        write!(f, "{}", name)?;
        for position in &self.positions {
            write!(f, "{}", position)?
        }
        write!(f, "]")
    }
}


impl Display for Piece<(i8, i8)> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[")?;
        let name = self.name.clone().unwrap_or_else(|| String::from(""));
        write!(f, "{}", name)?;
        for position in &self.positions {
            write!(f, "{}", position)?
        }
        write!(f, "]")
    }
}

impl<T> Transformable for Piece<T> where T: Transformable + PartialOrd + Ord {
    fn transform(&mut self, symmetry: &CubeSymmetry) {
        for position in &mut self.positions {
            position.transform(symmetry);
        }
        self.positions.sort()
    }
}

impl<T> Translatable<T> for Piece<T> where T: VectorAdd<T>  {
    fn translate(&mut self, translation: &Translation<T>) {
        for position in &mut self.positions {
            position.translate(translation);
        }
    }
}

impl<T> MinimumPosition<T> for Piece<T> where T: PartialOrd + Ord + Clone {
    fn minimum_position(&self) -> Option<Position<T>> {
        self.positions.iter().min().cloned()
    }
}

/// Iterate over the `Position`s of entities.
pub struct PositionIterator<T> {
    index: usize,
    positions: Vec<Position<T>>,
}

impl<T> PositionIterator<T> {
    /// Create a `PositionIterator` that iterates over the provided positions.
    pub fn new(positions: Vec<Position<T>>) -> PositionIterator<T> {
        PositionIterator { index: 0, positions }
    }
}

impl<T> Iterator for PositionIterator<T> where T: Clone {
    type Item = Position<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.positions.len() {
            let position = self.positions[self.index].clone();
            self.index += 1;
            Some(position)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_are_equal_on_values() {
        let a = Position::new(0, 1, 2);
        let b = Position::new(0, 1, 2);

        assert_eq!(a, b);
    }

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
    fn piece_should_translate() {
        let mut piece = Piece::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(1, 1, 1),
        ));
        let translation = Translation::new(5, -3, 0);

        piece.translate(&translation);

        assert_eq!(piece, Piece::new(vec!(
            Position::new(5, -3, 0),
            Position::new(6, -3, 0),
            Position::new(6, -2, 0),
            Position::new(6, -2, 1),
        )));
    }

    #[test]
    fn piece_should_tranform() {
        let mut piece = Piece::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(1, 1, 1),
        ));

        piece.transform(&CubeSymmetry::E2103);

        assert_eq!(piece, Piece::new(vec!(
            Position::new(0, 0, 0),
            Position::new(0, 1, 0),
            Position::new(1, 1, 0),
            Position::new(1, 1, -1),
        )));
    }
}
