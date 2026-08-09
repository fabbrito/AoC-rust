/// The differences between neighbours — one shorter than `row`.
fn diff(row: &[i64]) -> Vec<i64> {
    row.windows(2).map(|w| w[1] - w[0]).collect()
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

/// One parsed history per line of the report.
fn histories(input: &str) -> impl Iterator<Item = Vec<i64>> {
    input
        .lines()
        .map(|line| aoc::parse_numbers(line).expect("numbers"))
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
