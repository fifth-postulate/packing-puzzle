extern crate pack;

use pack::puzzle::solver::{Target, Solution, solve};
use pack::puzzle::piece::{Position, Template};
use pack::puzzle::pieces::Bag;

fn main(){
    let target = brick3x3x3();
    let bag = slothouber_graatsma_bag();
    let partial_solution = Solution::empty();

    solve(target, bag, partial_solution, &mut |solution|{
        println!("{}", solution)
    });
}

fn brick3x3x3() -> Target {
    Target::new(vec!(
        Position::new(0, 0, 0),
        Position::new(1, 0, 0),
        Position::new(2, 0, 0),
        Position::new(0, 1, 0),
        Position::new(1, 1, 0),
        Position::new(2, 1, 0),
        Position::new(0, 2, 0),
        Position::new(1, 2, 0),
        Position::new(2, 2, 0),

        Position::new(0, 0, 1),
        Position::new(1, 0, 1),
        Position::new(2, 0, 1),
        Position::new(0, 1, 1),
        Position::new(1, 1, 1),
        Position::new(2, 1, 1),
        Position::new(0, 2, 1),
        Position::new(1, 2, 1),
        Position::new(2, 2, 1),

        Position::new(0, 0, 2),
        Position::new(1, 0, 2),
        Position::new(2, 0, 2),
        Position::new(0, 1, 2),
        Position::new(1, 1, 2),
        Position::new(2, 1, 2),
        Position::new(0, 2, 2),
        Position::new(1, 2, 2),
        Position::new(2, 2, 2),
    ))
}

fn slothouber_graatsma_bag() -> Bag {
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
