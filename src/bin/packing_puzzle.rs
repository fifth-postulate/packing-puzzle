extern crate pack;

use pack::puzzle::piece::{Position, Template};
use pack::puzzle::pieces::Bag;
use pack::puzzle::solver::solve;
use pack::util::target::brick;

fn main() {
    let target = brick(4, 4, 4);
    let bag = packing_puzzle();

    solve(target, bag, &mut |solution|{
        println!("{}", solution);
    });
}

pub fn packing_puzzle() -> Bag {
    Bag::new(vec!(
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
            Position::new(2, 0, 0),
            Position::new(2, 1, 0),
            Position::new(1, 0, 1),
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
            Position::new(2, 1, 0),
            Position::new(1, 1, 1),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(2, 0, 0),
            Position::new(0, 1, 0),
            Position::new(0, 0, 1),
        )),
    ))
}
