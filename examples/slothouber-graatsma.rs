extern crate pack;

use pack::puzzle::solver::solve;
use pack::puzzle::piece::{Position, Template};
use pack::puzzle::pieces::Bag;
use pack::util::target::brick;

fn main(){
    let target = brick(3, 3, 3);
    let bag = slothouber_graatsma_bag();

    solve(&target, bag, &mut |solution|{
        println!("{}", solution)
    });
}

fn slothouber_graatsma_bag() -> Bag<(i8, i8, i8)> {
    Bag::new(vec!(
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(1, 1, 0),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(1, 1, 0),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(1, 1, 0),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(1, 1, 0),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(1, 1, 0),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(1, 1, 0),
        )),

        Template::new(vec!(
            Position::new(0, 0, 0),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
        )),
        Template::new(vec!(
            Position::new(0, 0, 0),
        )),
    ))
}
