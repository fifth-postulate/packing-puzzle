extern crate pack;

use pack::puzzle::piece::{Position, Template};
use pack::puzzle::pieces::Bag;
use pack::puzzle::solver::{Target,Solution,solve};

fn main() {
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
    solve(target, bag, partial_solution, &mut |solution|{
        println!("{:?}", solution);
        solutions.push(solution);
    });
}
