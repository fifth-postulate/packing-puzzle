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
    symmetry_index: u8,
    template: Template,
}

impl PieceIterator {
    pub fn new(template: Template) -> PieceIterator {
        PieceIterator { symmetry_index: 0, template: template }
    }
}

impl Iterator for PieceIterator {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        if self.symmetry_index < 24 {
            self.symmetry_index += 1;
            let piece = Piece::from(self.template.clone());

            Some(piece)
        } else {
            None
        }
    }
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

pub struct Piece {
    positions: Vec<Position>,
}

impl From<Template> for Piece {
    fn from(template: Template) -> Self {
        Piece { positions : template.positions }
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
