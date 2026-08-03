use std::{collections::HashMap, str::FromStr};

/// Offsets where a digit run begins; mid-run digits are skipped.
fn number_starts(line: &str) -> impl Iterator<Item = usize> {
    let bytes = line.as_bytes();
    (0..bytes.len())
        .filter(move |&i| bytes[i].is_ascii_digit() && (i == 0 || !bytes[i - 1].is_ascii_digit()))
}

fn symbols_in(line: &str) -> impl Iterator<Item = (usize, u8)> {
    line.bytes()
        .enumerate()
        .filter(|&(_, byte)| byte != b'.' && !byte.is_ascii_digit())
}

/// Value and width of the digit run at the front of `s`.
fn number_at(s: &str) -> Option<(u32, usize)> {
    let len = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    if len == 0 {
        return None;
    }
    Some((s[..len].parse::<u32>().expect("digits fit u32"), len))
}

type Position = (usize, usize);

/// `(row, start)` identifies a number uniquely; its value does not.
#[derive(Debug, Clone, Copy)]
struct Number {
    start: usize,
    value: u32,
}

#[derive(Debug)]
struct Schematic {
    /// Keyed by every cell a number covers, not just its first.
    cells: HashMap<Position, Number>,
    symbols: HashMap<Position, u8>,
}

impl FromStr for Schematic {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut cells = HashMap::new();
        let mut symbols = HashMap::new();
        for (row, line) in input.lines().enumerate() {
            for start in number_starts(line) {
                let (value, len) = number_at(&line[start..]).expect("run starts with a digit");
                let number = Number { start, value };
                cells.extend((start..start + len).map(|col| ((row, col), number)));
            }
            symbols.extend(symbols_in(line).map(|(col, symbol)| ((row, col), symbol)));
        }
        Ok(Schematic { cells, symbols })
    }
}

impl Schematic {
    /// Keyed by identity: a number touched at several cells appears once.
    fn adjacent_numbers(&self, (row, col): Position) -> HashMap<Position, u32> {
        let mut numbers = HashMap::new();
        // Clamped at the low edge only; an off-grid key just misses.
        for r in row.saturating_sub(1)..=row + 1 {
            for c in col.saturating_sub(1)..=col + 1 {
                if let Some(number) = self.cells.get(&(r, c)) {
                    numbers.insert((r, number.start), number.value);
                }
            }
        }
        numbers
    }

    fn part_numbers(&self) -> HashMap<Position, u32> {
        self.symbols
            .keys()
            .flat_map(|&pos| self.adjacent_numbers(pos))
            .collect()
    }

    /// Touching a `*` already makes a number a part, so no cross-check against
    /// [`Self::part_numbers`] is needed.
    fn gear_ratios(&self) -> impl Iterator<Item = usize> {
        self.symbols
            .iter()
            .filter(|&(_, &symbol)| symbol == b'*')
            .map(|(&pos, _)| self.adjacent_numbers(pos))
            .filter(|numbers| numbers.len() == 2)
            .map(|numbers| numbers.values().map(|&v| v as usize).product())
    }
}

struct Day03;

impl aoc::Day for Day03 {
    const DAY: u32 = 3;
    type Output = usize;

    fn part1(input: &str) -> usize {
        input
            .parse::<Schematic>()
            .expect("not parsed correctly")
            .part_numbers()
            .values()
            .map(|&v| v as usize)
            .sum()
    }

    fn part2(input: &str) -> usize {
        input
            .parse::<Schematic>()
            .expect("not parsed correctly")
            .gear_ratios()
            .sum()
    }
}

fn main() {
    aoc::run!(Day03);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
      467..114..
      ...*......
      ..35..633.
      ......#...
      617*......
      .....+.58.
      ..592.....
      ......755.
      ...$.*....
      .664.598..
    "};

    #[test]
    fn p1() {
        assert_eq!(Day03::part1(EXAMPLE), 4361);
    }

    #[test]
    fn p2() {
        assert_eq!(Day03::part2(EXAMPLE), 467835);
    }

    #[test]
    fn parses() {
        let s: Schematic = EXAMPLE.parse().unwrap();
        assert_eq!(s.cells.len(), 28);
        assert_eq!(s.symbols.len(), 6);
    }
}
