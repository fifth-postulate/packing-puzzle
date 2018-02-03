extern crate pack;

use pack::puzzle::piece::Position;
use pack::puzzle::pieces::Bag;
use pack::puzzle::solver::{Target,Solution,solve};

fn main() {
    let target = cube4x4x4();
    let bag = packing_puzzle();
    let partial_solution = Solution::empty();

    solve(target, bag, partial_solution, &mut |solution|{
        println!("{}", solution);
    });
}

fn cube4x4x4() -> Target {
    Target::new(vec!(
        Position::new(0, 0, 0),
        Position::new(1, 0, 0),
        Position::new(2, 0, 0),
        Position::new(3, 0, 0),
        Position::new(0, 1, 0),
        Position::new(1, 1, 0),
        Position::new(2, 1, 0),
        Position::new(3, 1, 0),
        Position::new(0, 2, 0),
        Position::new(1, 2, 0),
        Position::new(2, 2, 0),
        Position::new(3, 2, 0),
        Position::new(0, 3, 0),
        Position::new(1, 3, 0),
        Position::new(2, 3, 0),
        Position::new(3, 3, 0),
        Position::new(0, 0, 1),
        Position::new(1, 0, 1),
        Position::new(2, 0, 1),
        Position::new(3, 0, 1),
        Position::new(0, 1, 1),
        Position::new(1, 1, 1),
        Position::new(2, 1, 1),
        Position::new(3, 1, 1),
        Position::new(0, 2, 1),
        Position::new(1, 2, 1),
        Position::new(2, 2, 1),
        Position::new(3, 2, 1),
        Position::new(0, 3, 1),
        Position::new(1, 3, 1),
        Position::new(2, 3, 1),
        Position::new(3, 3, 1),
        Position::new(0, 0, 2),
        Position::new(1, 0, 2),
        Position::new(2, 0, 2),
        Position::new(3, 0, 2),
        Position::new(0, 1, 2),
        Position::new(1, 1, 2),
        Position::new(2, 1, 2),
        Position::new(3, 1, 2),
        Position::new(0, 2, 2),
        Position::new(1, 2, 2),
        Position::new(2, 2, 2),
        Position::new(3, 2, 2),
        Position::new(0, 3, 2),
        Position::new(1, 3, 2),
        Position::new(2, 3, 2),
        Position::new(3, 3, 2),
        Position::new(0, 0, 3),
        Position::new(1, 0, 3),
        Position::new(2, 0, 3),
        Position::new(3, 0, 3),
        Position::new(0, 1, 3),
        Position::new(1, 1, 3),
        Position::new(2, 1, 3),
        Position::new(3, 1, 3),
        Position::new(0, 2, 3),
        Position::new(1, 2, 3),
        Position::new(2, 2, 3),
        Position::new(3, 2, 3),
        Position::new(0, 3, 3),
        Position::new(1, 3, 3),
        Position::new(2, 3, 3),
        Position::new(3, 3, 3),
    ))
}

pub fn packing_puzzle() -> Bag {
    Bag { collection: vec!(
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(1, 1, 1),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(2, 0, 0),
            Position::new(1, 1, 0),
            Position::new(1, 0, 1),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(2, 0, 0),
            Position::new(2, 1, 0),
            Position::new(0, 0, 1),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(0, 0, 1),
            Position::new(0, 0, 2),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(2, 1, 0),
            Position::new(2, 2, 0),
        )),

        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(1, 2, 0),
            Position::new(2, 1, 0),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(2, 0, 0),
            Position::new(1, 1, 0),
            Position::new(1, 1, 1),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(1, 1, 1),
            Position::new(2, 1, 1),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(2, 0, 0),
            Position::new(0, 0, 1),
        )),

        Template::new(vec!(
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(1, 1, 0),
            Position::new(2, 1, 0),
            Position::new(1, 2, 0),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(2, 1, 0),
            Position::new(0, 0, 1),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(1, 2, 0),
            Position::new(1, 0, 1),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(1, 1, 0),
            Position::new(2, 1, 0),
            Position::new(1, 2, 0),
        )),
    ) }
}
