use std::str::FromStr;

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Value of the spelled-out digit starting at the front of `s`.
fn digit_as_word_at(s: &str) -> Option<u32> {
    DIGIT_WORDS
        .iter()
        .zip(1..)
        .find_map(|(word, value)| s.starts_with(word).then_some(value))
}

/// Value of the numeric digit starting at the front of `s`.
fn digit_as_char_at(s: &str) -> Option<u32> {
    s.chars().next()?.to_digit(10)
}

/// Value of any digit starting at the front of `s`, numeric or spelled.
fn digit_at(s: &str) -> Option<u32> {
    digit_as_char_at(s).or_else(|| digit_as_word_at(s))
}

/// First and last digit joined; a line with one digit uses it as both.
/// `digit` decides what counts as one, since that is the only difference
/// between the two parts.
fn calibration(line: &str, digit: impl Fn(&str) -> Option<u32>) -> Result<u32, String> {
    let mut digits = line.char_indices().filter_map(|(i, _)| digit(&line[i..]));
    let first = digits
        .next()
        .ok_or_else(|| format!("no digits in {line:?}"))?;
    Ok(first * 10 + digits.next_back().unwrap_or(first))
}

/// Numeric digits only.
#[derive(Debug)]
struct Numeric(u32);

/// Numeric digits and spelled-out ones.
#[derive(Debug)]
struct Spelled(u32);

impl FromStr for Numeric {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        calibration(line, digit_as_char_at).map(Numeric)
    }
}

impl FromStr for Spelled {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        calibration(line, digit_at).map(Spelled)
    }
}

struct Day01;

impl aoc::Day for Day01 {
    const DAY: u32 = 1;
    type Output = usize;

    fn part1(input: &str) -> usize {
        input
            .lines()
            .map(|line| line.parse::<Numeric>().expect("calibration value").0 as usize)
            .sum()
    }

    fn part2(input: &str) -> usize {
        input
            .lines()
            .map(|line| line.parse::<Spelled>().expect("calibration value").0 as usize)
            .sum()
    }
}

fn main() {
    aoc::run!(Day01);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
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
        assert_eq!(Day01::part1(EXAMPLE), 142);
    }

    #[test]
    fn p2() {
        assert_eq!(Day01::part2(EXAMPLE2), 281);
    }

    #[test]
    fn parses() {
        assert_eq!("1abc2".parse::<Numeric>().unwrap().0, 12);
        assert_eq!("treb7uchet".parse::<Numeric>().unwrap().0, 77);
        // Same line, different rule.
        assert_eq!("two1nine".parse::<Numeric>().unwrap().0, 11);
        assert_eq!("two1nine".parse::<Spelled>().unwrap().0, 29);
    }
}
