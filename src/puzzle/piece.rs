//! Describes objects to be packed.
//!
//! At the moment only objects that are aligned with a ordinary grid can be defined.

use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Error};

/// A `Template` is a container to hold a representation of a `Piece`. By
/// Iterating over a one gets a piece in all the possible orientations.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Template {
    positions: Vec<Position>,
}

impl Template {
    /// Create a `Template` from a vector of `Position`s.
    pub fn new(positions: Vec<Position>) -> Template {
        Template { positions }
    }
}

impl IntoIterator for Template {
    type Item = Piece;
    type IntoIter = PieceIterator;

    fn into_iter(self) -> Self::IntoIter {
        PieceIterator::new(self)
    }
}


/// The `PieceIterator` will return `Piece`s  in all the orientations possible
/// from a `Template`
pub struct PieceIterator {
    symmetry_iterator: CubeSymmetryIterator,
    template: Template,
}

impl PieceIterator {
    /// Creates a `PieceIterator` for the `Template` that is passed as an argument
    pub fn new(template: Template) -> PieceIterator {
        PieceIterator { symmetry_iterator: CubeSymmetryIterator::new(), template: template }
    }
}

impl Iterator for PieceIterator {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        let symmetry_option = self.symmetry_iterator.next();
        symmetry_option.map(|symmetry|{
            let mut piece = Piece::from(self.template.clone());

            piece.transform(&symmetry);

            piece
        })
    }
}

/// Symmetries of the Cube.
///
/// The group of symmetries of the cube is isomorphic to S<sub>4</sub>. Here we
/// use that fact as a naming convention for our elements.
#[derive(Clone)]
pub enum CubeSymmetry {
    #[allow(missing_docs)]
    E0123,
    #[allow(missing_docs)]
    E0132,
    #[allow(missing_docs)]
    E0213,
    #[allow(missing_docs)]
    E0231,
    #[allow(missing_docs)]
    E0312,
    #[allow(missing_docs)]
    E0321,

    #[allow(missing_docs)]
    E1023,
    #[allow(missing_docs)]
    E1032,
    #[allow(missing_docs)]
    E1203,
    #[allow(missing_docs)]
    E1230,
    #[allow(missing_docs)]
    E1302,
    #[allow(missing_docs)]
    E1320,

    #[allow(missing_docs)]
    E2013,
    #[allow(missing_docs)]
    E2031,
    #[allow(missing_docs)]
    E2103,
    #[allow(missing_docs)]
    E2130,
    #[allow(missing_docs)]
    E2301,
    #[allow(missing_docs)]
    E2310,

    #[allow(missing_docs)]
    E3012,
    #[allow(missing_docs)]
    E3021,
    #[allow(missing_docs)]
    E3102,
    #[allow(missing_docs)]
    E3120,
    #[allow(missing_docs)]
    E3201,
    #[allow(missing_docs)]
    E3210,
}

struct CubeSymmetryIterator {
    item: Option<CubeSymmetry>,
}

impl CubeSymmetryIterator {
    fn new() -> CubeSymmetryIterator {
        CubeSymmetryIterator { item: Some(CubeSymmetry::E0123) }
    }
}

impl Iterator for CubeSymmetryIterator {
    type Item = CubeSymmetry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.item.clone() {
            s @ Some(CubeSymmetry::E0123) => { self.item = Some(CubeSymmetry::E0132); s },
            s @ Some(CubeSymmetry::E0132) => { self.item = Some(CubeSymmetry::E0213); s },
            s @ Some(CubeSymmetry::E0213) => { self.item = Some(CubeSymmetry::E0231); s },
            s @ Some(CubeSymmetry::E0231) => { self.item = Some(CubeSymmetry::E0312); s },
            s @ Some(CubeSymmetry::E0312) => { self.item = Some(CubeSymmetry::E0321); s },
            s @ Some(CubeSymmetry::E0321) => { self.item = Some(CubeSymmetry::E1023); s },

            s @ Some(CubeSymmetry::E1023) => { self.item = Some(CubeSymmetry::E1032); s },
            s @ Some(CubeSymmetry::E1032) => { self.item = Some(CubeSymmetry::E1203); s },
            s @ Some(CubeSymmetry::E1203) => { self.item = Some(CubeSymmetry::E1230); s },
            s @ Some(CubeSymmetry::E1230) => { self.item = Some(CubeSymmetry::E1302); s },
            s @ Some(CubeSymmetry::E1302) => { self.item = Some(CubeSymmetry::E1320); s },
            s @ Some(CubeSymmetry::E1320) => { self.item = Some(CubeSymmetry::E2013); s },

            s @ Some(CubeSymmetry::E2013) => { self.item = Some(CubeSymmetry::E2031); s },
            s @ Some(CubeSymmetry::E2031) => { self.item = Some(CubeSymmetry::E2103); s },
            s @ Some(CubeSymmetry::E2103) => { self.item = Some(CubeSymmetry::E2130); s },
            s @ Some(CubeSymmetry::E2130) => { self.item = Some(CubeSymmetry::E2301); s },
            s @ Some(CubeSymmetry::E2301) => { self.item = Some(CubeSymmetry::E2310); s },
            s @ Some(CubeSymmetry::E2310) => { self.item = Some(CubeSymmetry::E3012); s },

            s @ Some(CubeSymmetry::E3012) => { self.item = Some(CubeSymmetry::E3021); s },
            s @ Some(CubeSymmetry::E3021) => { self.item = Some(CubeSymmetry::E3102); s },
            s @ Some(CubeSymmetry::E3102) => { self.item = Some(CubeSymmetry::E3120); s },
            s @ Some(CubeSymmetry::E3120) => { self.item = Some(CubeSymmetry::E3201); s },
            s @ Some(CubeSymmetry::E3201) => { self.item = Some(CubeSymmetry::E3210); s },
            s @ Some(CubeSymmetry::E3210) => { self.item = None; s },
            s @ None                      => { self.item = None; s },
        }
    }
}

/// Contract how various entities transform under the symmetries of the cube.
pub trait Transformable {
    /// Apply a symmetry and transform the entity.
    fn transform(&mut self, symmetry: &CubeSymmetry);
}

/// Entities can be translated through space. This struct determines how.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Translation {
    x: i8,
    y: i8,
    z: i8,
}

impl Translation {
    /// Create a Translation by stating how to move along each coordinate.
    pub fn new(x: i8, y: i8, z: i8) -> Translation {
        Translation { x, y, z }
    }
}

/// Contract how to translate entities.
pub trait Translatable {
    /// move entity by the `Translation`.
    fn translate(&mut self, translation: &Translation);
}

/// Position of a cubelet.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    x: i8,
    y: i8,
    z: i8,
}

impl Position {
    /// Create  position at the given coordinates.
    pub fn new(x: i8, y: i8, z: i8) -> Position {
        Position { x, y, z }
    }

    /// Apply translation to move a point to an other.
    pub fn to(&self, other: &Self) -> Translation {
        Translation::new(
            other.x - self.x,
            other.y - self.y,
            other.z - self.z
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.z.cmp(&other.z) {
            Ordering::Equal => {
                match self.y.cmp(&other.y) {
                    Ordering::Equal => {
                        self.x.cmp(&other.x)
                    },
                    y_ordering @ _ => y_ordering,
                }
            },
            z_ordering @ _ => z_ordering,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Transformable for Position {
    fn transform(&mut self, symmetry: &CubeSymmetry) {
        let (x, y, z) = (self.x, self.y, self.z);
        match *symmetry {
            CubeSymmetry::E0123 => { self.x =  x; self.y =  y; self.z =  z; },
            CubeSymmetry::E0132 => { self.x = -z; self.y = -y; self.z = -x; },
            CubeSymmetry::E0213 => { self.x = -x; self.y = -z; self.z = -y; },
            CubeSymmetry::E0231 => { self.x =  y; self.y =  z; self.z =  x; },
            CubeSymmetry::E0312 => { self.x =  z; self.y =  x; self.z =  y; },
            CubeSymmetry::E0321 => { self.x = -y; self.y = -x; self.z = -z; },
            CubeSymmetry::E1023 => { self.x =  z; self.y = -y; self.z =  x; },
            CubeSymmetry::E1032 => { self.x = -x; self.y =  y; self.z = -z; },
            CubeSymmetry::E1203 => { self.x = -z; self.y = -x; self.z =  y; },
            CubeSymmetry::E1230 => { self.x = -y; self.y =  x; self.z =  z; },
            CubeSymmetry::E1302 => { self.x =  x; self.y =  z; self.z = -y; },
            CubeSymmetry::E1320 => { self.x =  y; self.y = -z; self.z = -x; },
            CubeSymmetry::E2013 => { self.x = -y; self.y =  z; self.z = -x; },
            CubeSymmetry::E2031 => { self.x =  x; self.y = -z; self.z =  y; },
            CubeSymmetry::E2103 => { self.x =  y; self.y =  x; self.z = -z; },
            CubeSymmetry::E2130 => { self.x =  z; self.y = -x; self.z = -y; },
            CubeSymmetry::E2301 => { self.x = -x; self.y = -y; self.z =  z; },
            CubeSymmetry::E2310 => { self.x = -z; self.y =  y; self.z =  x; },
            CubeSymmetry::E3012 => { self.x =  y; self.y = -x; self.z =  z; },
            CubeSymmetry::E3021 => { self.x = -z; self.y =  x; self.z = -y; },
            CubeSymmetry::E3102 => { self.x = -y; self.y = -z; self.z =  x; },
            CubeSymmetry::E3120 => { self.x = -x; self.y =  z; self.z =  y; },
            CubeSymmetry::E3201 => { self.x =  z; self.y =  y; self.z = -x; },
            CubeSymmetry::E3210 => { self.x =  x; self.y = -y; self.z = -z; },
         }
    }
}

impl Translatable for Position {
    fn translate(&mut self, translation: &Translation) {
        self.x += translation.x;
        self.y += translation.y;
        self.z += translation.z;
    }
}

/// Contract to find the minimal `Position`
pub trait MinimumPosition {
    /// Return the minimal `Position` for the entity.
    fn minimum_position(&self) -> Option<Position>;
}

/// Entities that get packed.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Piece {
    /// TODO remove public modifier
    pub positions: Vec<Position>,
}

impl Piece {
    /// Create a new `Piece` from a collection of `Position`s.
    pub fn new(positions: Vec<Position>) -> Piece {
        Piece { positions }
    }

    /// Determine if a `Position` is contained in this `Piece`.
    pub fn contains(&self, position: &Position) -> bool {
        self.positions.contains(position)
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[")?;
        for position in &self.positions {
            write!(f, "{}", position)?
        }
        write!(f, "]")
    }
}

impl From<Template> for Piece {
    fn from(template: Template) -> Self {
        Piece { positions : template.positions }
    }
}

impl Transformable for Piece {
    fn transform(&mut self, symmetry: &CubeSymmetry) {
        for position in self.positions.iter_mut() {
            position.transform(symmetry);
        }
    }
}

impl Translatable for Piece {
    fn translate(&mut self, translation: &Translation) {
        for position in self.positions.iter_mut() {
            position.translate(translation);
        }
    }
}

impl MinimumPosition for Piece {
    fn minimum_position(&self) -> Option<Position> {
        self.positions.iter().min().map(|position| position.clone())
    }
}


#[cfg(test)]
mod tests {
    use std::iter::Iterator;
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
    fn templates_should_return_24_pieces() {
        let template = Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(1, 1, 1),
        ));

        let iterator: PieceIterator = template.into_iter();

        assert_eq!(iterator.count(), 24);
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
