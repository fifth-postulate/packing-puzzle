use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Template {
    positions: Vec<Position>,
}

impl Template {
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

pub struct PieceIterator {
    symmetry_iterator: CubeSymmetryIterator,
    template: Template,
}

impl PieceIterator {
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

#[derive(Clone)]
pub enum CubeSymmetry {
    E0123,
    E0132,
    E0213,
    E0231,
    E0312,
    E0321,

    E1023,
    E1032,
    E1203,
    E1230,
    E1302,
    E1320,

    E2013,
    E2031,
    E2103,
    E2130,
    E2301,
    E2310,

    E3012,
    E3021,
    E3102,
    E3120,
    E3201,
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

pub trait Transformable {
    fn transform(&mut self, symmetry: &CubeSymmetry);
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Translation {
    x: i8,
    y: i8,
    z: i8,
}

impl Translation {
    pub fn new(x: i8, y: i8, z: i8) -> Translation {
        Translation { x, y, z }
    }
}

pub trait Translatable {
    fn translate(&mut self, translation: &Translation);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    x: i8,
    y: i8,
    z: i8,
}

impl Position {
    pub fn new(x: i8, y: i8, z: i8) -> Position {
        Position { x, y, z }
    }

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
        match self.x.cmp(&other.x) {
            Ordering::Equal => {
                match self.y.cmp(&other.y) {
                    Ordering::Equal => {
                        self.z.cmp(&other.z)
                    },
                    y_ordering @ _ => y_ordering,
                }
            },
            x_ordering @ _ => x_ordering,
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

pub trait MinimumPosition {
    fn minimum_position(&self) -> Option<Position>;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Piece {
    pub positions: Vec<Position>,
}

impl Piece {
    pub fn new(positions: Vec<Position>) -> Piece {
        Piece { positions }
    }

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
