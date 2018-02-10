//! Module related to locations in space.
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Error};

use super::super::vector::{VectorDifference, VectorAdd};
use super::{Transformable, CubeSymmetry, Translatable, Translation};

/// Position of a cubelet.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position<T> {
    base: T,
}

impl Position<(i8, i8, i8)> {
    /// Create  position at the given coordinates.
    pub fn new(x: i8, y: i8, z: i8) -> Position<(i8, i8, i8)> {
        Position { base: (x, y, z) }
    }
}

/// Move an entity to certain `Position`.
pub trait Positionable<T> {
    /// Determine the `Translation` which takes the entity to a `Position`.
    fn to(&self, other: &Self) -> Translation<T>;
}

impl<T> Positionable<T> for Position<T> where T: VectorDifference<T> {
    fn to(&self, other: &Self) -> Translation<T> {
        let translation: T = self.base.difference(&other.base);

        Translation::from(translation)
    }
}

/// Move an entity to a origin.
pub trait Normalizable<T> {
    /// Determine the `Translation` which takes the entity to standard reference `Position`.
    fn to_reference(&self) -> Translation<T>;
}

impl Normalizable<(i8, i8, i8)> for Position<(i8, i8, i8)> {
    fn to_reference(&self) -> Translation<(i8, i8, i8)> {
        let translation = self.base.difference(&(0, 0, 0));

        Translation::from(translation)
    }
}

impl Normalizable<(i8, i8)> for Position<(i8, i8)> {
    fn to_reference(&self) -> Translation<(i8, i8)> {
        let translation = self.base.difference(&(0, 0));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_are_equal_on_values() {
        let a = Position::new(0, 1, 2);
        let b = Position::new(0, 1, 2);

        assert_eq!(a, b);
    }
}
