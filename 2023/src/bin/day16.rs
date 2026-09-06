use std::collections::HashSet;

struct Day16;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

type Beam = (usize, usize, Dir);

struct Grid {
    cells: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl Dir {
    const fn rotate_cw(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    const fn rotate_ccw(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
}

/// Fallible step: `Some` = landed on a tile, `None` = walked off the top/left
/// edge. The bottom/right edges are caught later by `Grid::get` returning `None`,
/// so only the guards for row/col 0 live here.
const fn step((r, c): (usize, usize), d: Dir) -> Option<(usize, usize)> {
    match d {
        Dir::North if r > 0 => Some((r - 1, c)),
        Dir::East => Some((r, c + 1)),
        Dir::South => Some((r + 1, c)),
        Dir::West if c > 0 => Some((r, c - 1)),
        _ => None,
    }
}

/// Tile + heading -> outgoing heading(s): `[_, None]` continues as one beam,
/// `[_, Some(_)]` splits. The `_` arm is empty space *and* a splitter hit
/// pointy-end — both pass straight through.
const fn outgoing(tile: u8, d: Dir) -> [Option<Dir>; 2] {
    use Dir::{East, North, South, West};
    // `/`: N<->E, S<->W.  `\`: N<->W, S<->E.  `|`/`-`: split when hit side-on.
    match (tile, d) {
        (b'/', North | South) | (b'\\', East | West) => [Some(d.rotate_cw()), None],
        (b'/', East | West) | (b'\\', North | South) => [Some(d.rotate_ccw()), None],
        (b'|', East | West) | (b'-', North | South) => [Some(d.rotate_ccw()), Some(d.rotate_cw())],
        _ => [Some(d), None],
    }
}

impl Grid {
    fn parse(input: &str) -> Option<Self> {
        let text = input.trim();
        let cols = text.lines().next()?.len();
        // one flat byte buffer; assumes every line is `cols` wide.
        let cells = text.lines().flat_map(str::bytes).collect::<Vec<_>>();
        let rows = cells.len().checked_div(cols)?;
        Some(Self { cells, rows, cols })
    }

    fn get(&self, r: usize, c: usize) -> Option<u8> {
        // guard `c`: otherwise `r * cols + c` folds an over-wide column into the
        // next row instead of missing.
        if c >= self.cols {
            return None;
        }
        self.cells.get(r * self.cols + c).copied()
    }

    fn energized(&self, start: Beam) -> usize {
        let mut visited: HashSet<Beam> = HashSet::new();
        let mut worklist = vec![start];

        while let Some(beam @ (r, c, dir)) = worklist.pop() {
            // bounds-check before `visited`: `step` lets beams run off the bottom
            // and right edges, and those must not count as energized.
            let Some(tile) = self.get(r, c) else {
                continue;
            };
            // this exact (tile, heading) already marched -> the rest is a loop.
            if !visited.insert(beam) {
                continue;
            }
            for next_dir in outgoing(tile, dir).into_iter().flatten() {
                if let Some((nr, nc)) = step((r, c), next_dir) {
                    worklist.push((nr, nc, next_dir));
                }
            }
        }

        // energized = distinct positions, heading discarded.
        visited
            .iter()
            .map(|&(r, c, _)| (r, c))
            .collect::<HashSet<_>>()
            .len()
    }
}

impl aoc::Day for Day16 {
    const DAY: u32 = 16;
    type Output = usize;

    fn part1(input: &str) -> usize {
        Grid::parse(input).map_or(0, |grid| grid.energized((0, 0, Dir::East)))
    }

    fn part2(input: &str) -> usize {
        let Some(grid) = Grid::parse(input) else {
            return 0;
        };
        let (rows, cols) = (grid.rows, grid.cols);
        // every perimeter cell, aimed inward; `parse` guarantees rows, cols >= 1.
        let edges = (0..cols)
            .flat_map(|c| [(0, c, Dir::South), (rows - 1, c, Dir::North)])
            .chain((0..rows).flat_map(|r| [(r, 0, Dir::East), (r, cols - 1, Dir::West)]));
        edges.map(|s| grid.energized(s)).max().unwrap_or(0)
    }
}

fn main() {
    aoc::run!(Day16);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "};

    #[test]
    fn parses_grid() {
        let grid = Grid::parse(EXAMPLE).unwrap();
        assert_eq!((grid.rows, grid.cols), (10, 10));
        assert_eq!(grid.get(0, 1), Some(b'|'));
        assert_eq!(grid.get(0, 5), Some(b'\\'));
        assert_eq!(grid.get(9, 3), Some(b'/'));
        assert_eq!(grid.get(0, 10), None); // past the right edge
        assert_eq!(grid.get(10, 0), None); // past the bottom
    }

    #[test]
    fn p1() {
        assert_eq!(Day16::part1(EXAMPLE), 46);
    }

    #[test]
    fn p2() {
        assert_eq!(Day16::part2(EXAMPLE), 51);
    }
}
