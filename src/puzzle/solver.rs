use std::fmt::{Display, Formatter, Error};

use super::piece::{MinimumPosition, Position, Translatable, Piece};
use super::pieces::Bag;

pub struct Target {
    collection: Vec<Position>,
}

impl Target {
    pub fn new(collection: Vec<Position>) -> Target {
        Target { collection }
    }

    pub fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }

    pub fn fits(&self, piece: &Piece) -> bool {
        /* TODO Create a position iterator */
        piece.positions.iter().all(|position| self.collection.contains(position))
    }

    pub fn place(&self, piece: &Piece) -> Target {
        let collection: Vec<Position> = self.collection
            .iter()
            .filter(|position| !piece.contains(position))
            .cloned()
            .collect();

        Target::new(collection)
    }
}

impl MinimumPosition for Target {
    fn minimum_position(&self) -> Option<Position> {
        self.collection.iter().min().map(|position| position.clone())
    }
}

#[derive(Debug)]
pub struct Solution {
    pieces: Vec<Piece>
}

impl Solution {
    pub fn empty() -> Solution {
        Solution { pieces: vec!() }
    }

    pub fn record(&self, piece: &Piece) -> Solution {
        let mut pieces: Vec<Piece> = self.pieces.iter().cloned().collect();
        pieces.push(piece.clone());

        Solution { pieces }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<")?;
        for piece in &self.pieces {
            write!(f, "{}", piece)?;
        }
        write!(f, ">")
    }
}

pub fn solve<F>(target: Target, bag: Bag, partial_solution: Solution, when_solved: &mut F) where F: (FnMut(Solution)) + Sized {
    if target.is_empty() {
        when_solved(partial_solution)
    } else {
        let open_position = target.minimum_position().unwrap();
        for (template, rest_of_bag) in bag {
            for mut piece in template {
                let block = piece.minimum_position().unwrap();
                let translation = block.to(&open_position);
                piece.translate(&translation);
                if target.fits(&piece) {
                    let remaining_target = target.place(&piece);
                    let candidate_solution = partial_solution.record(&piece);
                    solve(remaining_target, rest_of_bag.clone(), candidate_solution, when_solved)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::piece::{Position, Piece, Template};
    use super::super::pieces::Bag;
    use super::*;

    #[test]
    fn piece_should_fit_in_target() {
        let target = Target::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(1, 1, 0),
            Position::new(0, 0, 1),
            Position::new(1, 0, 1),
            Position::new(0, 1, 1),
            Position::new(1, 1, 1),
        ));

        let piece = Piece::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(0, 0, 1),
        ));

        assert!(target.fits(&piece));
    }

    #[test]
    fn solve_should_pack_pieces() {
        let target = Target::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(1, 1, 0),
            Position::new(0, 0, 1),
            Position::new(1, 0, 1),
            Position::new(0, 1, 1),
            Position::new(1, 1, 1),
        ));

        let bag = Bag::new(vec!(
            Template::new(vec!(
                Position::new(0, 0, 0),
                Position::new(1, 0, 0),
                Position::new(0, 1, 0),
                Position::new(0, 0, 1),
            )),
            Template::new(vec!(
                Position::new(0, 0, 0),
                Position::new(1, 0, 0),
                Position::new(0, 1, 0),
                Position::new(0, 0, 1),
            )),
        ));

        let partial_solution = Solution::empty();

        let mut solutions: Vec<Solution> = vec!();
        solve(target, bag, partial_solution, &mut |solution|{ solutions.push(solution)});
        assert_eq!(solutions.len(), 72);
    }
}
