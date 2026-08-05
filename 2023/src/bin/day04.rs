use std::{collections::HashSet, str::FromStr};

/// The header is dropped: a card's number is its position in the input.
#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    have: Vec<u32>,
}

/// `T` is chosen by the call site, so one parser fills any collection.
fn parse_numbers<T: FromIterator<u32>>(s: &str) -> Result<T, String> {
    s.split_whitespace()
        .map(|n| n.parse().map_err(|e| format!("{n:?}: {e}")))
        .collect()
}

impl FromStr for Card {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = line.split_once(':').ok_or("missing ':'")?;
        let (winning, have) = numbers.split_once('|').ok_or("missing '|'")?;
        Ok(Card {
            winning: parse_numbers(winning)?,
            have: parse_numbers(have)?,
        })
    }
}

impl Card {
    fn matches(&self) -> usize {
        self.have
            .iter()
            .filter(|v| self.winning.contains(v))
            .count()
    }

    /// Doubles per match past the first; zero matches scores nothing.
    fn points(&self) -> usize {
        self.matches().checked_sub(1).map_or(0, |exp| 1 << exp)
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(str::parse)
        .collect::<Result<_, String>>()
        .expect("parsed cards")
}

struct Day04;

impl aoc::Day for Day04 {
    const DAY: u32 = 4;
    type Output = usize;

    fn part1(input: &str) -> usize {
        parse_cards(input).iter().map(Card::points).sum()
    }

    fn part2(input: &str) -> usize {
        let cards = parse_cards(input);
        let mut counts = vec![1; cards.len()];
        for (i, card) in cards.iter().enumerate() {
            let copies = counts[i];
            for slot in counts[i + 1..].iter_mut().take(card.matches()) {
                *slot += copies;
            }
        }
        counts.into_iter().sum()
    }
}

fn main() {
    aoc::run!(Day04);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn p1() {
        assert_eq!(Day04::part1(EXAMPLE), 13);
    }

    #[test]
    fn p2() {
        assert_eq!(Day04::part2(EXAMPLE), 30);
    }

    #[test]
    fn parses() {
        for line in EXAMPLE.lines() {
            let card: Card = line.parse().unwrap();
            assert_eq!(card.winning.len(), 5);
            assert_eq!(card.have.len(), 8);
        }
    }
}
