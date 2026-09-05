use std::collections::HashMap;

struct Day14;

/// Row-major bytes, row 0 = north. `O` rounded, `#` cube, `.` empty.
type Grid = Vec<Vec<u8>>;

#[derive(Clone, Copy)]
enum Dir {
    North,
    West,
    South,
    East,
}

/// `O` rocks slide to the low-index end of each `#`-delimited run.
fn roll_front(lane: impl Iterator<Item = u8>) -> Vec<u8> {
    let lane: Vec<u8> = lane.collect();
    lane.split(|&b| b == b'#')
        .map(|run| {
            let (mut rolled, gaps): (Vec<u8>, Vec<u8>) =
                run.iter().copied().partition(|&b| b == b'O');
            rolled.extend(gaps);
            rolled
        })
        // `split` dropped the `#`s; `join` puts one back between every run,
        // and the empty runs from edge/adjacent `#` keep them in place.
        .collect::<Vec<_>>()
        .join(&b'#')
}

/// Swap rows and columns.
fn transpose(grid: &Grid) -> Grid {
    // One cursor per source row; each output row takes the next cell from every
    // cursor, i.e. one column of the original.
    let mut cols: Vec<_> = grid.iter().map(|row| row.iter()).collect();
    (0..grid.first().map_or(0, Vec::len))
        .map(|_| {
            cols.iter_mut()
                .filter_map(Iterator::next)
                .copied()
                .collect()
        })
        .collect()
}

/// Roll every `O` as far as it goes in `dir`.
fn tilt(grid: &Grid, dir: Dir) -> Grid {
    match dir {
        Dir::West => grid
            .iter()
            .map(|row| roll_front(row.iter().copied()))
            .collect(),
        Dir::East => grid
            .iter()
            .map(|row| {
                let mut rolled = roll_front(row.iter().rev().copied());
                rolled.reverse();
                rolled
            })
            .collect(),
        // North/south = west/east along columns: transpose, roll, transpose back.
        Dir::North => transpose(&tilt(&transpose(grid), Dir::West)),
        Dir::South => transpose(&tilt(&transpose(grid), Dir::East)),
    }
}

fn parse(input: &str) -> Grid {
    input.trim().lines().map(|l| l.bytes().collect()).collect()
}

/// One cycle: tilt N, W, S, E.
fn spin(grid: &Grid) -> Grid {
    [Dir::North, Dir::West, Dir::South, Dir::East]
        .into_iter()
        .fold(grid.clone(), |g, dir| tilt(&g, dir))
}

/// Sum of `height - row` over `O` — north-beam load.
fn load(grid: &Grid) -> usize {
    let h = grid.len();
    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().map(move |&b| (r, b)))
        .filter(|&(_, b)| b == b'O')
        .map(|(r, _)| h - r)
        .sum()
}

impl aoc::Day for Day14 {
    const DAY: u32 = 14;
    type Output = usize;

    fn part1(input: &str) -> usize {
        load(&tilt(&parse(input), Dir::North))
    }

    fn part2(input: &str) -> usize {
        const TOTAL: usize = 1_000_000_000;
        let mut grid = parse(input);
        let mut seen: HashMap<Grid, usize> = HashMap::new();
        let mut i = 0usize;
        // Spin until a state recurs; everything past that is periodic.
        let cycle = loop {
            if let Some(&first) = seen.get(&grid) {
                break i - first;
            }
            seen.insert(grid.clone(), i);
            grid = spin(&grid);
            i += 1;
        };
        // Only the remainder within the last period still matters.
        for _ in 0..(TOTAL - i) % cycle {
            grid = spin(&grid);
        }
        load(&grid)
    }
}

fn main() {
    aoc::run!(Day14);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn parses_grid() {
        let g = parse(EXAMPLE);
        assert_eq!(g.len(), 10);
        assert!(g.iter().all(|row| row.len() == 10));
        assert_eq!(g[0], b"O....#....");
        assert_eq!(g[9], b"#OO..#....");
    }

    #[test]
    fn p1() {
        assert_eq!(Day14::part1(EXAMPLE), 136);
    }

    #[test]
    fn p2() {
        assert_eq!(Day14::part2(EXAMPLE), 64);
    }
}
