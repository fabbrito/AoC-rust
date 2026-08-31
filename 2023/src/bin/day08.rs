use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Right,
}

impl TryFrom<char> for Turn {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Turn::Left),
            'R' => Ok(Turn::Right),
            _ => Err(format!("bad turn {c:?}")),
        }
    }
}

/// Euclid: the remainder chain hits zero, and the last non-zero value is the
/// divisor both numbers share.
fn gcd(mut a: usize, mut b: usize) -> usize {
    // `checked_rem` is the b != 0 test and the remainder in one step: when the
    // chain hits zero there is nothing left to divide by, and `a` is the gcd.
    while let Some(rem) = a.checked_rem(b) {
        (a, b) = (b, rem);
    }
    a
}

/// Divide before multiplying — `a * b` overflows long before the real answer.
fn lcm(a: usize, b: usize) -> usize {
    a.checked_div(gcd(a, b)).map_or(0, |q| q.saturating_mul(b))
}

/// One `AAA = (BBB, CCC)` line, as the entry it becomes.
fn parse_node(line: &str) -> Result<(String, (String, String)), String> {
    let (label, pair) = line.split_once(" = ").ok_or(format!("bad node {line:?}"))?;
    let (left, right) = pair
        .strip_prefix('(')
        .and_then(|pair| pair.strip_suffix(')'))
        .and_then(|pair| pair.split_once(", "))
        .ok_or(format!("bad pair {pair:?}"))?;

    Ok((label.to_string(), (left.to_string(), right.to_string())))
}

/// The document: the turn sequence, and the network it indexes into.
#[derive(Debug)]
struct Map {
    turns: Vec<Turn>,
    network: HashMap<String, (String, String)>,
}

impl Map {
    /// Steps from `start` to the first node satisfying `done`, following
    /// `turns` and repeating them as needed.
    fn steps(&self, start: &str, done: impl Fn(&str) -> bool) -> usize {
        let mut node = start;
        let mut steps = 0_usize;

        // `cycle()` restarts the slice forever, so running out of turns is not
        // a case the walk has to handle.
        for turn in self.turns.iter().cycle() {
            // A label with no entry is a dead end, not a reason to panic: stop
            // and report the steps taken so far.
            let Some((left, right)) = self.network.get(node) else {
                break;
            };
            node = match turn {
                Turn::Left => left,
                Turn::Right => right,
            };
            steps = steps.saturating_add(1);

            if done(node) {
                break;
            }
        }

        steps
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, rest) = s.split_once("\n\n").ok_or("missing instruction list")?;

        Ok(Map {
            turns: header
                .chars()
                .map(Turn::try_from)
                .collect::<Result<_, _>>()?,
            network: rest.lines().map(parse_node).collect::<Result<_, _>>()?,
        })
    }
}

struct Day08;

impl aoc::Day for Day08 {
    const DAY: u32 = 8;
    type Output = usize;

    fn part1(input: &str) -> usize {
        aoc::parse::<Map>(input).steps("AAA", |node| node == "ZZZ")
    }

    fn part2(input: &str) -> usize {
        let map = aoc::parse::<Map>(input);
        map.network
            .keys()
            .filter(|node| node.ends_with('A'))
            .map(|start| map.steps(start, |node| node.ends_with('Z')))
            .fold(1, lcm)
    }
}

fn main() {
    aoc::run!(Day08);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    const REPEATS: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    const GHOSTS: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn p1() {
        assert_eq!(Day08::part1(EXAMPLE), 2);
    }

    #[test]
    fn p1_repeats_turns() {
        assert_eq!(Day08::part1(REPEATS), 6);
    }

    #[test]
    fn p2() {
        assert_eq!(Day08::part2(GHOSTS), 6);
    }

    #[test]
    fn parses() {
        let map: Map = EXAMPLE.parse().unwrap();
        assert_eq!(map.turns.len(), 2);
        assert_eq!(map.network.len(), 7);
        assert_eq!(map.network["CCC"], ("ZZZ".to_string(), "GGG".to_string()));
    }
}
