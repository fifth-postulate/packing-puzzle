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


impl Ord for Position<(i8, i8, i8)> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.base.2.cmp(&other.base.2) {
            Ordering::Equal => {
                match self.base.1.cmp(&other.base.1) {
                    Ordering::Equal => {
                        self.base.0.cmp(&other.base.0)
                    },
                    y_ordering => y_ordering,
                }
            },
            z_ordering => z_ordering,
        }
    }
}

impl PartialOrd for Position<(i8, i8, i8)> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Transformable for Position<(i8, i8, i8)> {
    fn transform(&mut self, symmetry: &CubeSymmetry) {
        let (x, y, z) = self.base;
        let sx: i8;
        let sy: i8;
        let sz: i8;
        match *symmetry {
            CubeSymmetry::E0123 => { sx =  x; sy =  y; sz =  z; },
            CubeSymmetry::E0132 => { sx = -z; sy = -y; sz = -x; },
            CubeSymmetry::E0213 => { sx = -x; sy = -z; sz = -y; },
            CubeSymmetry::E0231 => { sx =  y; sy =  z; sz =  x; },
            CubeSymmetry::E0312 => { sx =  z; sy =  x; sz =  y; },
            CubeSymmetry::E0321 => { sx = -y; sy = -x; sz = -z; },
            CubeSymmetry::E1023 => { sx =  z; sy = -y; sz =  x; },
            CubeSymmetry::E1032 => { sx = -x; sy =  y; sz = -z; },
            CubeSymmetry::E1203 => { sx = -z; sy = -x; sz =  y; },
            CubeSymmetry::E1230 => { sx = -y; sy =  x; sz =  z; },
            CubeSymmetry::E1302 => { sx =  x; sy =  z; sz = -y; },
            CubeSymmetry::E1320 => { sx =  y; sy = -z; sz = -x; },
            CubeSymmetry::E2013 => { sx = -y; sy =  z; sz = -x; },
            CubeSymmetry::E2031 => { sx =  x; sy = -z; sz =  y; },
            CubeSymmetry::E2103 => { sx =  y; sy =  x; sz = -z; },
            CubeSymmetry::E2130 => { sx =  z; sy = -x; sz = -y; },
            CubeSymmetry::E2301 => { sx = -x; sy = -y; sz =  z; },
            CubeSymmetry::E2310 => { sx = -z; sy =  y; sz =  x; },
            CubeSymmetry::E3012 => { sx =  y; sy = -x; sz =  z; },
            CubeSymmetry::E3021 => { sx = -z; sy =  x; sz = -y; },
            CubeSymmetry::E3102 => { sx = -y; sy = -z; sz =  x; },
            CubeSymmetry::E3120 => { sx = -x; sy =  z; sz =  y; },
            CubeSymmetry::E3201 => { sx =  z; sy =  y; sz = -x; },
            CubeSymmetry::E3210 => { sx =  x; sy = -y; sz = -z; },
         }
         self.base = (sx, sy, sz)
    }
}

impl<T> Translatable<T> for Position<T> where T: VectorAdd<T> {
    fn translate(&mut self, translation: &Translation<T>) {
        self.base.add(&translation.delta);
    }
}

/// Contract to find the minimal `Position`
pub trait MinimumPosition {
    /// Return the minimal `Position` for the entity.
    fn minimum_position(&self) -> Option<Position<(i8, i8, i8)>>;
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

impl MinimumPosition for Piece {
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
