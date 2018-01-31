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
            let piece = Piece::from(self.template.clone());

            piece
        })
    }
}

#[derive(Clone)]
enum CubeSymmetry {
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

trait Transformable {
    fn transform(&mut self, symmetry: &CubeSymmetry);
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
}

impl Transformable for Position {
    fn transform(&mut self, symmetry: &CubeSymmetry) {
        match *symmetry {
            CubeSymmetry::E0123 => { self.x =  self.x; self.y =  self.y; self.z =  self.z; },
            CubeSymmetry::E0132 => { self.x = -self.z; self.y = -self.y; self.z = -self.x; },
            CubeSymmetry::E0213 => { self.x = -self.x; self.y = -self.z; self.z = -self.y; },
            CubeSymmetry::E0231 => { self.x =  self.y; self.y =  self.z; self.z =  self.x; },
            CubeSymmetry::E0312 => { self.x =  self.z; self.y =  self.x; self.z =  self.y; },
            CubeSymmetry::E0321 => { self.x = -self.y; self.y = -self.x; self.z = -self.z; },
            CubeSymmetry::E1023 => { self.x =  self.z; self.y = -self.y; self.z =  self.x; },
            CubeSymmetry::E1032 => { self.x = -self.x; self.y =  self.y; self.z = -self.z; },
            CubeSymmetry::E1203 => { self.x = -self.z; self.y = -self.x; self.z =  self.y; },
            CubeSymmetry::E1230 => { self.x = -self.y; self.y =  self.x; self.z =  self.z; },
            CubeSymmetry::E1302 => { self.x =  self.x; self.y =  self.z; self.z = -self.y; },
            CubeSymmetry::E1320 => { self.x =  self.y; self.y = -self.z; self.z = -self.x; },
            CubeSymmetry::E2013 => { self.x = -self.y; self.y =  self.z; self.z = -self.x; },
            CubeSymmetry::E2031 => { self.x =  self.x; self.y = -self.z; self.z =  self.y; },
            CubeSymmetry::E2103 => { self.x =  self.y; self.y =  self.x; self.z = -self.z; },
            CubeSymmetry::E2130 => { self.x =  self.z; self.y = -self.x; self.z = -self.y; },
            CubeSymmetry::E2301 => { self.x = -self.x; self.y = -self.y; self.z =  self.z; },
            CubeSymmetry::E2310 => { self.x = -self.z; self.y =  self.y; self.z =  self.x; },
            CubeSymmetry::E3012 => { self.x =  self.y; self.y = -self.x; self.z =  self.z; },
            CubeSymmetry::E3021 => { self.x = -self.z; self.y =  self.x; self.z = -self.y; },
            CubeSymmetry::E3102 => { self.x = -self.y; self.y = -self.z; self.z =  self.x; },
            CubeSymmetry::E3120 => { self.x = -self.x; self.y =  self.z; self.z =  self.y; },
            CubeSymmetry::E3201 => { self.x =  self.z; self.y =  self.y; self.z = -self.x; },
            CubeSymmetry::E3210 => { self.x =  self.x; self.y = -self.y; self.z = -self.z; },
         }
    }
}

pub struct Piece {
    positions: Vec<Position>,
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
}
