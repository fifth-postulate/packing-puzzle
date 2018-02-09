//! Entities that get packed

use std::fmt::{Formatter, Display, Error};

use super::super::vector::VectorAdd;
use super::{Position, MinimumPosition, Translatable, Translation, Transformable, CubeSymmetry};

/// A piece that get packed.
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

    /// Create a named `Piece` from a collection of `Position`s.
    pub fn named<S>(mut positions: Vec<Position<T>>, name: S) -> Piece<T> where S: Into<String> {
        positions.sort();
        Piece { positions, name: Some(name.into()) }
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
