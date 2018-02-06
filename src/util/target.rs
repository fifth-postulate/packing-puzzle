//! Utilities for creating `Target`s.

use super::super::puzzle::solver::Target;
use super::super::puzzle::piece::Position;

/// Create a brick `Target`, defined by the dimensions.
pub fn brick(width: u8, height: u8, depth: u8) -> Target {
    let mut positions: Vec<Position> = vec!();
    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                positions.push(Position::new(x as i8, y as i8, z as i8));
            }
        }
    }
    Target::new(positions)
}

#[cfg(test)]
mod tests {
	use super::super::super::puzzle::piece::{Position, Piece};
	use super::*;

	#[test]
	fn should_create_a_brick() {
		let target = brick(1,2,3);

		assert!(target.fits(&Piece::new(vec!(Position::new(0,0,0)))));
		assert!(target.fits(&Piece::new(vec!(Position::new(0,0,1)))));
		assert!(target.fits(&Piece::new(vec!(Position::new(0,0,2)))));
		assert!(target.fits(&Piece::new(vec!(Position::new(0,1,0)))));
		assert!(target.fits(&Piece::new(vec!(Position::new(0,1,1)))));
		assert!(target.fits(&Piece::new(vec!(Position::new(0,1,2)))));
	}
}
