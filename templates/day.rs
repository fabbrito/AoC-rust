fn part1(input: &str) -> usize {
    let _ = input;
    0
}

fn part2(input: &str) -> usize {
    let _ = input;
    0
}

fn main() {
    let input = aoc::input!(DAY);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
";

    #[test]
    fn p1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
