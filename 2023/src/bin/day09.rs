use std::str::FromStr;

/// The differences between neighbours — one shorter than `row`.
fn diff(row: &[i64]) -> Vec<i64> {
    // The row zipped against itself one step along: each pair is a neighbour
    // pair, and the shorter side ends the zip - no indexing, no length maths.
    row.iter()
        .zip(row.iter().skip(1))
        .map(|(&left, &right)| right.saturating_sub(left))
        .collect()
}

/// `history` and every difference row below it, ending with the first all-zero
/// row. Lazy: nothing past that row is ever built.
fn rows(history: Vec<i64>) -> impl Iterator<Item = Vec<i64>> {
    std::iter::successors(Some(history), |row| {
        row.iter().any(|&n| n != 0).then(|| diff(row))
    })
}

/// The value that follows the last of `history`. Feed it a reversed history and
/// it gives you the value that precedes the first.
fn next_value(history: Vec<i64>) -> i64 {
    rows(history)
        .map(|row| row.last().copied().unwrap_or(0))
        .sum()
}

/// One line of the report.
struct History(Vec<i64>);

impl FromStr for History {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        aoc::parse_numbers(line).map(History)
    }
}

/// One parsed history per line of the report.
fn histories(input: &str) -> impl Iterator<Item = Vec<i64>> {
    aoc::parse_lines(input).map(|History(row)| row)
}

struct Day09;

impl aoc::Day for Day09 {
    const DAY: u32 = 9;
    type Output = i64;

    fn part1(input: &str) -> i64 {
        histories(input).map(next_value).sum()
    }

    fn part2(input: &str) -> i64 {
        histories(input)
            .map(|mut history| {
                history.reverse();
                next_value(history)
            })
            .sum()
    }
}

fn main() {
    aoc::run!(Day09);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    // Each history alone — a one-line report sums to just that prediction, so
    // these bisect which one is wrong when EXAMPLE is off.
    const HISTORY_1: &str = "0 3 6 9 12 15\n";
    const HISTORY_2: &str = "1 3 6 10 15 21\n";
    const HISTORY_3: &str = "10 13 16 21 30 45\n";

    #[test]
    fn p1_history_1() {
        assert_eq!(Day09::part1(HISTORY_1), 18);
    }

    #[test]
    fn p1_history_2() {
        assert_eq!(Day09::part1(HISTORY_2), 28);
    }

    #[test]
    fn p1_history_3() {
        assert_eq!(Day09::part1(HISTORY_3), 68);
    }

    #[test]
    fn p1() {
        assert_eq!(Day09::part1(EXAMPLE), 114);
    }

    #[test]
    fn p2_history_1() {
        assert_eq!(Day09::part2(HISTORY_1), -3);
    }

    #[test]
    fn p2_history_2() {
        assert_eq!(Day09::part2(HISTORY_2), 0);
    }

    #[test]
    fn p2_history_3() {
        assert_eq!(Day09::part2(HISTORY_3), 5);
    }

    #[test]
    fn p2() {
        assert_eq!(Day09::part2(EXAMPLE), 2);
    }
}
