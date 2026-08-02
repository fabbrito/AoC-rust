use std::fmt::Display;
use std::path::Path;

/// One puzzle day. Implement on a unit struct in `<year>/src/bin/dayNN.rs`.
pub trait Day {
    /// Which day this is — used to locate `inputs/dayNN.txt`.
    const DAY: u32;

    /// What both parts answer with, e.g. `usize` or `String`.
    type Output: Display;

    fn part1(input: &str) -> Self::Output;
    fn part2(input: &str) -> Self::Output;
}

/// Read `<crate>/inputs/dayNN.txt`. Use via the [`input!`] macro.
pub fn read_input(manifest_dir: &str, day: u32) -> String {
    let path = Path::new(manifest_dir).join(format!("inputs/day{day:02}.txt"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Read `D`'s input and print both parts. Use via the [`run!`] macro.
pub fn run<D: Day>(manifest_dir: &str) {
    let input = read_input(manifest_dir, D::DAY);
    println!("part1: {}", D::part1(&input));
    println!("part2: {}", D::part2(&input));
}

/// `aoc::run!(Day01);` -> read `inputs/day01.txt`, print both parts.
#[macro_export]
macro_rules! run {
    ($day:ty) => {
        $crate::run::<$day>(env!("CARGO_MANIFEST_DIR"))
    };
}

/// `let input = aoc::input!(1);` -> contents of `inputs/day01.txt` in the calling crate.
#[macro_export]
macro_rules! input {
    ($day:expr) => {
        $crate::read_input(env!("CARGO_MANIFEST_DIR"), $day)
    };
}
