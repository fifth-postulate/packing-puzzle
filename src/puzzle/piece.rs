//! Describes objects to be packed.
//!
//! At the moment only objects that are aligned with a ordinary grid can be defined.

use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Error};

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
    seen_pieces: Vec<Piece>,
    template: Template,
}

impl PieceIterator {
    /// Creates a `PieceIterator` for the `Template` that is passed as an argument
    pub fn new(template: Template) -> PieceIterator {
        PieceIterator {
            symmetry_iterator: CubeSymmetryIterator::new(),
            seen_pieces: vec!(),
            template: template
        }
    }
}

impl Iterator for PieceIterator {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        let mut symmetry_option = self.symmetry_iterator.next();
        while symmetry_option.is_some() {
            let piece_option = symmetry_option.map(|symmetry|{
                let mut piece = Piece::from(self.template.clone());

                piece.transform(&symmetry);
                let minimum_position = piece.minimum_position();
                let translation = minimum_position.unwrap().to(&Position::new(0, 0, 0));
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
            s @ Some(CubeSymmetry::E3210) | s @ None => { self.item = None; s },
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
pub struct Translation<T> {
    delta : T,
}

impl Translation<(i8, i8, i8)> {
    /// Create a Translation by stating how to move along each coordinate.
    pub fn new(x: i8, y: i8, z: i8) -> Translation<(i8, i8, i8)> {
        Translation { delta: (x, y, z) }
    }
}

/// Contract how to translate entities.
pub trait Translatable<T> {
    /// move entity by the `Translation`.
    fn translate(&mut self, translation: &Translation<T>);
}

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

    /// Return a translation to move a point to an other.
    pub fn to(&self, other: &Self) -> Translation<(i8, i8, i8)> {
        let translation : Translation<(i8, i8, i8)> = Translation::new(
            other.base.0 - self.base.0,
            other.base.1 - self.base.1,
            other.base.2 - self.base.2
        );

        translation
    }
}

impl Display for Position<(i8, i8, i8)> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({}, {}, {})", self.base.0, self.base.1, self.base.2)
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

impl Translatable<(i8, i8, i8)> for Position<(i8, i8, i8)> {
    fn translate(&mut self, translation: &Translation<(i8, i8, i8)>) {
        let x = self.base.0 + translation.delta.0;
        let y = self.base.1 + translation.delta.1;
        let z = self.base.2 + translation.delta.2;
        self.base = (x, y, z);
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

impl From<Template> for Piece {
    fn from(template: Template) -> Self {
        Piece { positions : template.positions, name : template.name }
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
    fn templates_should_return_24_pieces_unsymmetric_templates() {
        let template = Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(1, 1, 1),
            Position::new(1, 1, 2),
        ));

        let iterator: PieceIterator = template.into_iter();

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

        let iterator: PieceIterator = template.into_iter();

        assert_eq!(iterator.count(), 3);
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
