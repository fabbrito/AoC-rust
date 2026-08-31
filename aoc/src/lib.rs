use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

/// Parse whitespace-separated numbers.
///
/// Both the number type and the collection come from the call site, so one
/// parser fills any container with any numeric type:
/// `let seeds: Vec<u64> = aoc::parse_numbers(line)?;`
///
/// # Errors
/// The first word that does not parse as `N`, reported with the word and the
/// parse error.
pub fn parse_numbers<N, T>(s: &str) -> Result<T, String>
where
    N: FromStr,
    N::Err: Display,
    T: FromIterator<N>,
{
    s.split_whitespace()
        .map(|word| word.parse().map_err(|err| format!("{word:?}: {err}")))
        .collect()
}

/// The whole of `input` parsed as `T`.
///
/// Same bargain as [`parse_lines`]: the panic is deliberate and lives here so
/// no day needs its own `expect`.
///
/// # Panics
/// If `input` does not parse, naming the error.
#[must_use]
pub fn parse<T>(input: &str) -> T
where
    T: FromStr,
    T::Err: Display,
{
    input
        .parse()
        .unwrap_or_else(|err| panic!("cannot parse input: {err}"))
}

/// Every line of `input` parsed as `T`.
///
/// The panic is deliberate and lives here so no day needs its own `expect`:
/// a puzzle input that will not parse means the parser is wrong, and half an
/// answer is worse than none.
///
/// # Panics
/// On the first line that fails to parse, naming the line number and the error.
pub fn parse_lines<T>(input: &str) -> impl Iterator<Item = T>
where
    T: FromStr,
    T::Err: Display,
{
    input.lines().enumerate().map(|(i, line)| {
        line.parse()
            .unwrap_or_else(|err| panic!("line {}: {err}", i.saturating_add(1)))
    })
}

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
///
/// # Panics
/// If the file is missing or unreadable — a day cannot run without its input.
#[must_use]
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
