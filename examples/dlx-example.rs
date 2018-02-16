extern crate dlx;

use dlx::{Row, Solutions, Solution, Solver};

/// We take the dlx crate for a spin by packing dominoes in a 2x3 rectangle.
///
/// We first need to cast the problem in a exact cover format.
/// For this we will use 9 columns with the following meaning
/// * column 0-2: dominoes.
/// * column (3*j + i + 3): cell (i, j) is occupied.
///
/// With this interpretation we have the following matrix.
fn dominoes2x3() -> Vec<Row> {
    vec!(
        vec!(0, 3, 4),
        vec!(0, 4, 5),
        vec!(0, 6, 7),
        vec!(0, 7, 8),
        vec!(0, 3, 6),
        vec!(0, 4, 7),
        vec!(0, 5, 8),

        vec!(1, 3, 4),
        vec!(1, 4, 5),
        vec!(1, 6, 7),
        vec!(1, 7, 8),
        vec!(1, 3, 6),
        vec!(1, 4, 7),
        vec!(1, 5, 8),

        vec!(2, 3, 4),
        vec!(2, 4, 5),
        vec!(2, 6, 7),
        vec!(2, 7, 8),
        vec!(2, 3, 6),
        vec!(2, 4, 7),
        vec!(2, 5, 8),
    )
}

/// We want to store the number of solutions in a struct to inspect them later.
#[derive(Debug)]
struct DominoesSolutions {
    count: usize
}

impl DominoesSolutions {
    fn new() -> Self {
        DominoesSolutions { count: 0 }
    }
}

/// When a solution is found, we want to be notified.
impl Solutions for DominoesSolutions {
    fn push(&mut self, _: Solution) -> bool {
        self.count += 1;
        true
    }
}

fn main() {
    let mut solver = Solver::new(9, dominoes2x3().into_iter());
    let mut solutions = DominoesSolutions::new();
    let partial: Vec<Row> = vec!();

    solver.solve(partial, &mut solutions);

    println!("Number of solutions {:?}", solutions);
}
