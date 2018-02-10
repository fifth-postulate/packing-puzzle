//! Describes objects to be packed.
//!
//! At the moment only objects that are aligned with an ordinary rectangular grid can be defined.

mod symmetry;
mod translation;
mod position;
mod entity;
mod template;

pub use self::symmetry::{Transformable, CubeSymmetry, CubeSymmetryIterator};
pub use self::translation::{Translatable, Translation};
pub use self::position::{Position, Positionable, Normalizable, MinimumPosition};
pub use self::entity::{Piece};
pub use self::template::Template;
