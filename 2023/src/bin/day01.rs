const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Value of the spelled-out digit starting at the front of `s`.
fn digit_as_word_at(s: &str) -> Option<u32> {
    DIGIT_WORDS
        .iter()
        .position(|w| s.starts_with(w))
        .map(|i| i as u32 + 1)
}

/// Value of the numeric digit starting at the front of `s`.
fn digit_as_char_at(s: &str) -> Option<u32> {
    s.chars().next()?.to_digit(10)
}

/// Value of any digit starting at the front of `s`, numeric or spelled.
fn digit_at(s: &str) -> Option<u32> {
    digit_as_char_at(s).or_else(|| digit_as_word_at(s))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut ranks = line
                .char_indices()
                .filter_map(|(i, _)| digit_as_char_at(&line[i..]));
            let first = ranks.next().expect("line has no digits");
            let last = ranks.next_back().unwrap_or(first);
            (first * 10 + last) as usize
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut ranks = line
                .char_indices()
                .filter_map(|(i, _)| digit_at(&line[i..]));
            let first = ranks.next().expect("line has no digits");
            let last = ranks.next_back().unwrap_or(first);
            (first * 10 + last) as usize
        })
        .sum()
}

fn main() {
    let input = aoc::input!(1);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const EXAMPLE2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn p1() {
        assert_eq!(part1(EXAMPLE), 142);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(EXAMPLE2), 281);
    }
}
