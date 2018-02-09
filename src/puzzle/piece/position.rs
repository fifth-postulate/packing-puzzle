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
