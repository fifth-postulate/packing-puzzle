//! Describes objects to be packed.
//!
//! At the moment only objects that are aligned with an ordinary rectangular grid can be defined.

mod symmetry;
mod translation;
mod template;

pub use self::symmetry::{Transformable, CubeSymmetry, CubeSymmetryIterator};
pub use self::translation::{Translatable, Translation};
pub use self::template::Template;

use super::vector::{VectorDifference, VectorAdd};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Error};

/// Position of a cubelet.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position<T> {
    base: T,
}

/// Move an entity to certain `Position`.
pub trait Positionable<T> {
    /// Determine the `Translation` which takes the entity to a `Position`.
    fn to(&self, other: &Self) -> Translation<T>;
}

impl Position<(i8, i8, i8)> {
    /// Create  position at the given coordinates.
    pub fn new(x: i8, y: i8, z: i8) -> Position<(i8, i8, i8)> {
        Position { base: (x, y, z) }
    }
}

impl<T> Positionable<T> for Position<T> where T: VectorDifference<T> {
    fn to(&self, other: &Self) -> Translation<T> {
        let translation: T = self.base.difference(&other.base);

        Translation::from(translation)
    }
}

impl Display for Position<(i8, i8, i8)> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {}, {})", self.base.0, self.base.1, self.base.2)
    }
}

impl Display for Position<(i8, i8)> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {})", self.base.0, self.base.1)
    }
}


impl<T> Ord for Position<T> where T: PartialOrd + Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.base.cmp(&other.base)
    }
}

impl<T> PartialOrd for Position<T> where T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.base.partial_cmp(&other.base)
    }
}

impl<T> Transformable for Position<T> where T: Transformable {
    fn transform(&mut self, symmetry: &CubeSymmetry) {
        self.base.transform(&symmetry);
    }
}

impl<T> Translatable<T> for Position<T> where T: VectorAdd<T> {
    fn translate(&mut self, translation: &Translation<T>) {
        self.base.add(&translation.delta);
    }
}

/// Contract to find the minimal `Position`
pub trait MinimumPosition<T> where T: PartialOrd + Ord {
    /// Return the minimal `Position` for the entity.
    fn minimum_position(&self) -> Option<Position<T>>;
}

/// Entities that get packed.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Piece {
    positions: Vec<Position<(i8, i8, i8)>>,
    name: Option<String>
}

impl Piece {
    /// Create a new `Piece` from a collection of `Position`s.
    pub fn new(mut positions: Vec<Position<(i8, i8, i8)>>) -> Piece {
        positions.sort();
        Piece { positions, name: None }
    }

    /// Determine if a `Position` is contained in this `Piece`.
    pub fn contains(&self, position: &Position<(i8, i8, i8)>) -> bool {
        self.positions.contains(position)
    }

    /// Create an `Iterator` that iterates over all `Position`s.
    pub fn iter(&self) -> PositionIterator {
        let positions: Vec<Position<(i8, i8, i8)>> = self.positions.to_vec();
        PositionIterator::new(positions)
    }
}

impl Display for Piece {
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

impl Transformable for Piece {
    fn transform(&mut self, symmetry: &CubeSymmetry) {
        for position in &mut self.positions {
            position.transform(symmetry);
        }
        self.positions.sort()
    }
}

impl Translatable<(i8, i8, i8)> for Piece {
    fn translate(&mut self, translation: &Translation<(i8, i8, i8)>) {
        for position in &mut self.positions {
            position.translate(translation);
        }
    }
}

impl MinimumPosition<(i8, i8, i8)> for Piece {
    fn minimum_position(&self) -> Option<Position<(i8, i8, i8)>> {
        self.positions.iter().min().cloned()
    }
}

/// Iterate over the `Position`s of entities.
pub struct PositionIterator {
    index: usize,
    positions: Vec<Position<(i8, i8, i8)>>,
}

impl PositionIterator {
    /// Create a `PositionIterator` that iterates over the provided positions.
    pub fn new(positions: Vec<Position<(i8, i8, i8)>>) -> PositionIterator {
        PositionIterator { index: 0, positions }
    }
}

impl Iterator for PositionIterator {
    type Item = Position<(i8, i8, i8)>;

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
