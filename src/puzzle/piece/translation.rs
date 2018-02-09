//! Translated entities through space.

use super::super::vector::{VectorDifference, VectorAdd};

/// Entities can be translated through space. This struct determines how.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Translation<T> {
    /// By how much an entity gets translated in space.
    pub delta : T,
}

impl<T> From<T> for Translation<T> where T: VectorDifference<T> {
    fn from(t: T) -> Self {
        Translation { delta: t }
    }
}

impl Translation<(i8, i8, i8)> {
    /// Create a Translation by stating how to move along each coordinate.
    pub fn new(x: i8, y: i8, z: i8) -> Translation<(i8, i8, i8)> {
        Translation { delta: (x, y, z) }
    }
}

/// Contract how to translate entities.
pub trait Translatable<T> where T: VectorAdd<T> {
    /// move entity by the `Translation`.
    fn translate(&mut self, translation: &Translation<T>);
}


