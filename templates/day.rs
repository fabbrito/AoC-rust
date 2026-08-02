struct DayNN;

impl aoc::Day for DayNN {
    const DAY: u32 = DAYNUM;
    type Output = usize;

    fn part1(input: &str) -> usize {
        let _ = input;
        0
    }

    fn part2(input: &str) -> usize {
        let _ = input;
        0
    }
}

fn main() {
    aoc::run!(DayNN);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        paste the example here
    "};

    #[test]
    fn p1() {
        assert_eq!(DayNN::part1(EXAMPLE), 0);
    }

    #[test]
    fn p2() {
        assert_eq!(DayNN::part2(EXAMPLE), 0);
    }
}
