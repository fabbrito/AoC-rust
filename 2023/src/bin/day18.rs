use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

/// One dig instruction.
struct Step {
    dir: Dir,
    dist: u64,
}

impl FromStr for Step {
    type Err = String;

    /// "R 6 (#70c710)" -> the literal instruction.
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut words = line.split_whitespace();
        let dir = match words.next() {
            Some("U") => Dir::Up,
            Some("D") => Dir::Down,
            Some("L") => Dir::Left,
            Some("R") => Dir::Right,
            other => return Err(format!("bad dir {other:?}")),
        };
        let dist = words
            .next()
            .ok_or("missing dist")?
            .parse()
            .map_err(|e| format!("bad dist: {e}"))?;
        Ok(Self { dir, dist })
    }
}

/// The hex is the real instruction: first 5 digits = dist,
/// last digit = dir (0=R, 1=D, 2=L, 3=U).
struct HexStep(Step);

impl FromStr for HexStep {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let hex = line
            .split_once("(#")
            .and_then(|(_, rest)| rest.strip_suffix(')'))
            .ok_or("no hex color")?;
        let (dist_hex, dir_hex) = hex.split_at_checked(5).ok_or("short hex color")?;
        let dist = u64::from_str_radix(dist_hex, 16).map_err(|e| format!("bad hex dist: {e}"))?;
        let dir = match dir_hex {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
            other => return Err(format!("bad hex dir {other:?}")),
        };
        Ok(Self(Step { dir, dist }))
    }
}

impl From<HexStep> for Step {
    fn from(HexStep(step): HexStep) -> Self {
        step
    }
}

/// Walk the plan from (0, 0): the corner points in order — start included and
/// closed, since the last step lands back on it — plus the total perimeter.
fn walk(steps: &[Step]) -> (Vec<(i64, i64)>, u64) {
    let (mut x, mut y) = (0i64, 0i64);
    let mut points = vec![(x, y)];
    let mut perimeter = 0;

    for step in steps {
        let (dx, dy) = match step.dir {
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };
        x += dx * step.dist.cast_signed();
        y += dy * step.dist.cast_signed();
        perimeter += step.dist;
        points.push((x, y));
    }

    (points, perimeter)
}

/// Shoelace + Pick: interior + boundary = shoelace + perimeter/2 + 1.
fn area(points: &[(i64, i64)], perimeter: u64) -> u64 {
    let mut sum: i128 = 0;
    let wrapped = points.iter().skip(1).chain(points.iter().take(1));
    for (&(x0, y0), &(x1, y1)) in points.iter().zip(wrapped) {
        sum += i128::from(x0) * i128::from(y1) - i128::from(x1) * i128::from(y0);
    }
    let shoelace = u64::try_from(sum.unsigned_abs() / 2).unwrap_or_default();
    shoelace + perimeter / 2 + 1
}

/// The lava the lagoon holds: walk the plan, then shoelace + Pick.
fn enclosed(steps: &[Step]) -> u64 {
    let (points, perimeter) = walk(steps);
    area(&points, perimeter)
}

struct Day18;

impl aoc::Day for Day18 {
    const DAY: u32 = 18;
    type Output = usize;

    fn part1(input: &str) -> usize {
        let steps: Vec<Step> = aoc::parse_lines(input.trim()).collect();
        usize::try_from(enclosed(&steps)).unwrap_or_default()
    }

    fn part2(input: &str) -> usize {
        let steps: Vec<Step> = aoc::parse_lines::<HexStep>(input.trim())
            .map(Step::from)
            .collect();
        usize::try_from(enclosed(&steps)).unwrap_or_default()
    }
}

fn main() {
    aoc::run!(Day18);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn parses_literal() {
        let step = "R 6 (#70c710)".parse::<Step>().unwrap();
        assert!(matches!(
            step,
            Step {
                dir: Dir::Right,
                dist: 6
            }
        ));

        let step = "U 2 (#7a21e3)".parse::<Step>().unwrap();
        assert!(matches!(
            step,
            Step {
                dir: Dir::Up,
                dist: 2
            }
        ));
    }

    #[test]
    fn parses_hex() {
        let step = "R 6 (#70c710)".parse::<HexStep>().unwrap();
        assert!(matches!(
            step,
            HexStep(Step {
                dir: Dir::Right,
                dist: 461_937
            })
        ));

        let step = "D 2 (#0dc571)".parse::<HexStep>().unwrap();
        assert!(matches!(
            step,
            HexStep(Step {
                dir: Dir::Down,
                dist: 56_407
            })
        ));

        let step = "L 5 (#8ceee2)".parse::<HexStep>().unwrap();
        assert!(matches!(
            step,
            HexStep(Step {
                dir: Dir::Left,
                dist: 577_262
            })
        ));

        let step = "U 2 (#caa173)".parse::<HexStep>().unwrap();
        assert!(matches!(
            step,
            HexStep(Step {
                dir: Dir::Up,
                dist: 829_975
            })
        ));
    }

    #[test]
    fn rejects_short_hex() {
        assert!("R 6 (#ff)".parse::<HexStep>().is_err());
    }

    // the real input ends with a blank line; lines() yields it as an empty str
    #[test]
    fn p1_tolerates_blank_lines() {
        let input = format!("{EXAMPLE}\n\n");
        assert_eq!(Day18::part1(&input), 62);
    }

    #[test]
    fn p1() {
        assert_eq!(Day18::part1(EXAMPLE), 62);
    }

    #[test]
    fn p2() {
        assert_eq!(Day18::part2(EXAMPLE), 952_408_144_115);
    }
}
