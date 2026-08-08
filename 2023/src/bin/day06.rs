use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
struct Race {
    time: u64,
    distance: u64,
}

#[derive(Debug)]
struct Sheet {
    time: Vec<u64>,
    distance: Vec<u64>,
}

impl FromStr for Sheet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = |label: &str| {
            s.lines()
                .find_map(|line| line.strip_prefix(label))
                .ok_or_else(|| format!("{label} row missing"))
        };
        Ok(Sheet {
            time: aoc::parse_numbers(row("Time:")?)?,
            distance: aoc::parse_numbers(row("Distance:")?)?,
        })
    }
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;
    (discriminant >= 0.0).then(|| {
        let sqrt = discriminant.sqrt();
        ((-b - sqrt) / (2.0 * a), (-b + sqrt) / (2.0 * a))
    })
}

impl Race {
    /// `hold * (time - hold) > distance` rearranges to
    /// `hold² - time·hold + distance < 0`, a downward parabola, so the winning
    /// holds are the integers strictly between its roots — a root is the hold
    /// that exactly ties the record, and a tie does not win.
    fn ways(&self) -> usize {
        let equation = solve_quadratic(1.0, -(self.time as f64), self.distance as f64);
        equation.map_or(0, |(lower_root, upper_root)| {
            let shortest = lower_root.floor() as usize + 1;
            let longest = upper_root.ceil() as usize - 1;
            (shortest..=longest).count()
        })
    }
}

impl Sheet {
    fn races(&self) -> Vec<Race> {
        self.time
            .iter()
            .zip(&self.distance)
            .map(|(&time, &distance)| Race { time, distance })
            .collect()
    }

    /// Part 2 reads each row as a single number, so the values are concatenated
    /// digit-wise. `ilog10() + 1` is the digit count — it panics on `0`, which
    /// no race time or distance is.
    fn joined_race(&self) -> Vec<Race> {
        let concat = |joined: u64, &n: &u64| joined * 10u64.pow(n.ilog10() + 1) + n;
        vec![Race {
            time: self.time.iter().fold(0, concat),
            distance: self.distance.iter().fold(0, concat),
        }]
    }
}

fn margin(races: &[Race]) -> usize {
    races.iter().map(Race::ways).product()
}

struct Day06;

impl aoc::Day for Day06 {
    const DAY: u32 = 6;
    type Output = usize;

    fn part1(input: &str) -> usize {
        let sheet = input.parse::<Sheet>().expect("parsed sheet");
        margin(&sheet.races())
    }

    fn part2(input: &str) -> usize {
        let sheet = input.parse::<Sheet>().expect("parsed sheet");
        margin(&sheet.joined_race())
    }
}

fn main() {
    aoc::run!(Day06);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn p1() {
        assert_eq!(Day06::part1(EXAMPLE), 288);
    }

    #[test]
    fn p2() {
        assert_eq!(Day06::part2(EXAMPLE), 71503);
    }

    #[test]
    fn parses() {
        let sheet: Sheet = EXAMPLE.parse().unwrap();
        assert_eq!(sheet.time, [7, 15, 30]);
        assert_eq!(sheet.distance, [9, 40, 200]);
    }
}
