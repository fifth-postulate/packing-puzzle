extern crate pack;

use pack::puzzle::piece::{Position, Template};
use pack::puzzle::pieces::Bag;
use pack::puzzle::solver::solve;
use pack::util::target::rectangle;

fn main() {
    let target = rectangle(3, 20);
    let bag = pentominoes();

    solve(&target, bag, &mut |solution|{
        println!("{}", solution);
    });
}

pub fn pentominoes() -> Bag<(i8, i8)> {
    Bag::new(vec!(
        Template::new(vec!(
            Position::d2(0, 0),
            Position::d2(1, 0),
            Position::d2(2, 0),
            Position::d2(2, 1),
            Position::d2(2, 2),
        )),
        Template::new(vec!(
            Position::d2(0, 0),
            Position::d2(1, 0),
            Position::d2(1, 1),
            Position::d2(1, 2),
            Position::d2(2, 2),
        )),
        Template::new(vec!(
            Position::d2(0, 0),
            Position::d2(1, 0),
            Position::d2(2, 0),
            Position::d2(3, 0),
            Position::d2(2, 1),
        )),
        Template::new(vec!(
            Position::d2(0, 0),
            Position::d2(1, 0),
            Position::d2(1, 1),
            Position::d2(2, 1),
            Position::d2(2, 2),
        )),
        Template::new(vec!(
            Position::d2(0, 0),
            Position::d2(1, 0),
            Position::d2(2, 0),
            Position::d2(1, 1),
            Position::d2(1, 2),
        )),
        Template::new(vec!(
            Position::d2(0, 0),
            Position::d2(1, 0),
            Position::d2(1, 1),
            Position::d2(1, 2),
            Position::d2(2, 1),
        )),
        Template::new(vec!(
            Position::d2(0, 0),
            Position::d2(1, 0),
            Position::d2(2, 0),
            Position::d2(2, 1),
            Position::d2(3, 1),
        )),
        Template::new(vec!(
            Position::d2(0, 0),
            Position::d2(1, 0),
            Position::d2(2, 0),
            Position::d2(3, 0),
            Position::d2(3, 1),
        )),
        Template::new(vec!(
            Position::d2(0, 0),
            Position::d2(1, 0),
            Position::d2(2, 0),
            Position::d2(3, 0),
            Position::d2(4, 0),
        )),
        Template::new(vec!(
            Position::d2(0, 0),
            Position::d2(1, 1),
            Position::d2(2, 1),
            Position::d2(1, 1),
            Position::d2(2, 1),
        )),
        Template::new(vec!(
            Position::d2(0, 1),
            Position::d2(1, 0),
            Position::d2(1, 1),
            Position::d2(1, 2),
            Position::d2(2, 1),
        )),
        Template::new(vec!(
            Position::d2(0, 0),
            Position::d2(1, 0),
            Position::d2(2, 0),
            Position::d2(0, 1),
            Position::d2(2, 1),
        )),
    ))
}
