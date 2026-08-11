#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    const ALL: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];
}

/// (row, col).
type Pos = (usize, usize);

struct Grid<'a> {
    rows: Vec<&'a [u8]>,
}

impl<'a> Grid<'a> {
    fn parse(input: &'a str) -> Grid<'a> {
        let rows = input.lines().map(str::as_bytes).collect();
        Grid { rows }
    }

    fn start(&self) -> Option<Pos> {
        self.rows
            .iter()
            .enumerate()
            .find_map(|(r, row)| Some((r, row.iter().position(|&b| b == b'S')?)))
    }

    /// One step `dir` from `pos`: where you land and what's there. `None` off
    /// the edge.
    fn step(&self, pos: Pos, dir: Dir) -> Option<(Pos, u8)> {
        let (r, c) = match dir {
            Dir::North => (pos.0.checked_sub(1)?, pos.1),
            Dir::East => (pos.0, pos.1 + 1),
            Dir::South => (pos.0 + 1, pos.1),
            Dir::West => (pos.0, pos.1.checked_sub(1)?),
        };
        let tile = *self.rows.get(r)?.get(c)?;
        Some(((r, c), tile))
    }
}

/// Enter `pipe` heading `heading`, leave heading this way. `None` when no end
/// faces back the way you came — ground, `S`, or a pipe that misses.
fn exit(pipe: u8, heading: Dir) -> Option<Dir> {
    match (pipe, heading) {
        (b'F', Dir::North) | (b'L', Dir::South) => Some(Dir::East),
        (b'F', Dir::West) | (b'7', Dir::East) => Some(Dir::South),
        (b'7', Dir::North) | (b'J', Dir::South) => Some(Dir::West),
        (b'J', Dir::East) | (b'L', Dir::West) => Some(Dir::North),
        (b'-', Dir::East | Dir::West) | (b'|', Dir::North | Dir::South) => Some(heading),
        _ => None,
    }
}

/// Every tile of the main loop, in walking order, starting at `S`. Its length
/// is both the loop's perimeter and the count of boundary lattice points.
fn compute_edge(grid: &Grid) -> Vec<Pos> {
    let start = grid.start().expect("grid to have a start");
    let heading = Dir::ALL
        .into_iter()
        .find_map(|d| {
            let (_, tile) = grid.step(start, d)?;
            exit(tile, d)?;
            Some(d)
        })
        .expect("S to connect to two pipes");

    // The seed puts `start` at index 0; `exit` is `None` on `S`, which is what
    // ends the walk when we arrive back there.
    std::iter::successors(Some((start, heading)), |&(pos, heading)| {
        let (next, tile) = grid.step(pos, heading).expect("loop stays on the grid");
        exit(tile, heading).map(|next_heading| (next, next_heading))
    })
    .map(|(pos, _)| pos)
    .collect()
}

/// Area enclosed by a polygon given as its boundary points in order. The cross
/// terms go negative, hence the signed sum over unsigned `Pos`.
fn shoelace(polygon: &[Pos]) -> usize {
    let twice_area: i64 = (0..polygon.len())
        .map(|i| {
            let (r0, c0) = polygon[i];
            let (r1, c1) = polygon[(i + 1) % polygon.len()];
            r0 as i64 * c1 as i64 - r1 as i64 * c0 as i64
        })
        .sum();
    (twice_area.abs() / 2) as usize
}

struct Day10;

impl aoc::Day for Day10 {
    const DAY: u32 = 10;
    type Output = usize;

    /// The far point is half a lap away, and the lap is always even.
    fn part1(input: &str) -> usize {
        let grid = Grid::parse(input);
        let edge = compute_edge(&grid);
        edge.len() / 2
    }

    /// Pick's theorem rearranged: `interior = area - boundary / 2 + 1`.
    fn part2(input: &str) -> usize {
        let grid = Grid::parse(input);
        let edge = compute_edge(&grid);
        shoelace(&edge) - edge.len() / 2 + 1
    }
}

fn main() {
    aoc::run!(Day10);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    /// The bare square loop.
    const SQUARE: &str = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};

    /// The same square loop, buried in pipe that isn't connected to it.
    const SQUARE_JUNK: &str = indoc! {"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
    "};

    /// A loop that doubles back on itself.
    const COMPLEX: &str = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    /// The same doubling loop, with the junk pipe shown.
    const COMPLEX_JUNK: &str = indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "};

    /// Wide open middle, four enclosed tiles in the two lower pockets.
    const OPEN: &str = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    /// Same four tiles, but the outside now reaches the middle only by
    /// squeezing between two touching pipes.
    const SQUEEZE: &str = indoc! {"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
    "};

    /// Eight enclosed, with ground both inside and outside the loop.
    const LARGE: &str = indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    "};

    /// Ten enclosed, and every spare cell is junk pipe rather than ground.
    const LARGE_JUNK: &str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn p1_square() {
        assert_eq!(Day10::part1(SQUARE), 4);
    }

    #[test]
    fn p1_square_junk() {
        assert_eq!(Day10::part1(SQUARE_JUNK), 4);
    }

    #[test]
    fn p1_complex() {
        assert_eq!(Day10::part1(COMPLEX), 8);
    }

    #[test]
    fn p1_complex_junk() {
        assert_eq!(Day10::part1(COMPLEX_JUNK), 8);
    }

    #[test]
    fn p2_open() {
        assert_eq!(Day10::part2(OPEN), 4);
    }

    #[test]
    fn p2_squeeze() {
        assert_eq!(Day10::part2(SQUEEZE), 4);
    }

    #[test]
    fn p2_large() {
        assert_eq!(Day10::part2(LARGE), 8);
    }

    #[test]
    fn p2_large_junk() {
        assert_eq!(Day10::part2(LARGE_JUNK), 10);
    }
}
