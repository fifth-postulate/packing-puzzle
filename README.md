# Packing Puzzle[![Build Status](https://travis-ci.org/fifth-postulate/packing-puzzle.svg?branch=master)](https://travis-ci.org/fifth-postulate/packing-puzzle)[![Coverage Status](https://coveralls.io/repos/github/fifth-postulate/packing-puzzle/badge.svg?branch=master)](https://coveralls.io/github/fifth-postulate/packing-puzzle?branch=master)[![pack on crates.io](https://img.shields.io/crates/v/pack.svg)](https://crates.io/crates/pack)
Solver for [packing problems][packing].

## installation
You can use this library by adding a dependency to your `Cargo.toml`

```toml
[dependencies]
pack = "*"
```

If you want to fix a specific version, feel free to enter a version number.

## Slothouber-Graatsma Puzzle
The [Slohouber-Graatsma puzzle][puzzle] asks for

> packing six 1 × 2 × 2 blocks and three 1 × 1 × 1 blocks into a 3 × 3 × 3 box.

We are going to solve it with the `pack` library.

First we announce the use of the external crate.

```rust
extern crate pack;
```

We need to a few things before we can start solving the puzzle. One is a
`pack::puzzle::solver::Target`. A target designates the volume to pack. A target
is created with a vector of `pack::puzzle::piece::Position`s.

```rust
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
```

Because bricks are often used as targets, there is a utility
`pack::util::target::brick` function that does exactly that. The above code
could be replaced with `brick(3, 3, 3)`.


One other thing we need is a `pack::puzzle::pieces::Bag` of
`pack::puzzle::piece:Template`s. A template is a shape that can be oriented in
different ways by iterating over them. A bag is a container to hold templates.

Templates are created by providing the vector of positions they occupy.

```rust
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
```

Finally we need to tell the `solve` function what to do when they find a
solution. This can be done by passing a clojure. For this example we will just
print the solution.

Our `main` function could look like the following code.

```rust
fn main(){
    let target = brick(3, 3, 3);
    let bag = slothouber_graatsma_bag();
    let partial_solution = Solution::empty();

    solve(&target, bag, partial_solution, &mut |solution|{
        println!("{}", solution)
    });
}
```

Running it will *eventually* print a solution to the Slothouber-Graatsma puzzle.
The full source for this example can be found in
[examples/slothouber-graatsma.rs][example]. For a more extensive documentation
see the [wiki][].

## Development
If you are interested in contributing to this library please read
[CONTRIBUTING.md][contributing].


### Running clippy
Follow the installation instruction for [clippy][] and run the following command.
```sh
cargo +nightly clippy
```

[packing]: https://en.wikipedia.org/wiki/Packing_problems
[puzzle]: https://en.wikipedia.org/wiki/Slothouber%E2%80%93Graatsma_puzzle
[example]: examples/slothouber-graatsma.rs
[wiki]: https://github.com/fifth-postulate/packing-puzzle/wiki
[contributing]: CONTRIBUTING.md
[clippy]: https://github.com/rust-lang-nursery/rust-clippy
