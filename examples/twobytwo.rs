extern crate pack;

use pack::puzzle::piece::{Position, Template};
use pack::puzzle::pieces::Bag;
use pack::puzzle::solver::{Solution, solve};
use pack::util::target::brick;

fn main() {
    let target = brick(2, 2, 2);

    let bag = Bag::new(vec!(
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(0, 0, 1),
        )).with_name("A"),
        Template::new(vec!(
            Position::new(0, 0, 0),
            Position::new(1, 0, 0),
            Position::new(0, 1, 0),
            Position::new(0, 0, 1),
        )).with_name("B"),
    ));

    let mut solutions: Vec<Solution<(i8, i8, i8)>> = vec!();
    solve(&target, bag, &mut |solution|{
        println!("{}", solution);
        solutions.push(solution);
    });
}
