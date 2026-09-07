use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct Day17;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

struct Grid {
    cells: Vec<u64>,
    rows: usize,
    cols: usize,
}

/// One search node: tile + heading + straight-run so far.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct State {
    r: usize,
    c: usize,
    dir: Dir,
    run: u8,
}

impl Dir {
    const fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    const fn left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
}

impl Grid {
    fn parse(input: &str) -> Option<Self> {
        let text = input.trim();
        let cols = text.lines().next()?.len();
        // every line assumed `cols` wide; a non-digit fails the whole collect
        let cells = text
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).map(u64::from)))
            .collect::<Option<Vec<_>>>()?;
        let rows = cells.len().checked_div(cols)?;
        Some(Self { cells, rows, cols })
    }

    fn get(&self, r: usize, c: usize) -> Option<u64> {
        // guard `c`: otherwise `r * cols + c` folds an over-wide column into the
        // next row instead of missing.
        if c >= self.cols {
            return None;
        }
        self.cells.get(r * self.cols + c).copied()
    }
}

/// Fallible step; guards only the top/left edges — bottom/right are caught
/// later by `Grid::get` returning `None`.
const fn step((r, c): (usize, usize), d: Dir) -> Option<(usize, usize)> {
    match d {
        Dir::North if r > 0 => Some((r - 1, c)),
        Dir::East => Some((r, c + 1)),
        Dir::South => Some((r + 1, c)),
        Dir::West if c > 0 => Some((r, c - 1)),
        _ => None,
    }
}

/// Dijkstra relaxation: queue `state` when `cost + tile` improves on the best
/// known. Bottom/right bounds are enforced here via `get` — top/left in `step`.
fn relax(
    grid: &Grid,
    dist: &mut HashMap<State, u64>,
    heap: &mut BinaryHeap<Reverse<(u64, State)>>,
    cost: u64,
    state: State,
) {
    let Some(tile) = grid.get(state.r, state.c) else {
        return;
    };
    let new_cost = cost + tile;
    if new_cost < dist.get(&state).copied().unwrap_or(u64::MAX) {
        dist.insert(state, new_cost);
        heap.push(Reverse((new_cost, state)));
    }
}

// Dijkstra over states; the run window is the only part knob.
fn shortest(grid: &Grid, min_run: u8, max_run: u8) -> Option<u64> {
    let mut dist = HashMap::<State, u64>::new();
    let mut heap = BinaryHeap::<Reverse<(u64, State)>>::new();
    let goal = (grid.rows - 1, grid.cols - 1);

    // Seed: East or South out of the start, run = 1. Cost 0 so relax adds the
    // entered tile's heat — the start tile itself is never paid.
    for dir in [Dir::East, Dir::South] {
        if let Some((r, c)) = step((0, 0), dir) {
            relax(grid, &mut dist, &mut heap, 0, State { r, c, dir, run: 1 });
        }
    }

    while let Some(Reverse((cost, state))) = heap.pop() {
        // Lazy deletion: skip stale entries whose state was reached cheaper
        // after they were pushed.
        if dist.get(&state).copied() != Some(cost) {
            continue;
        }

        // Stopping at the goal needs the same run window as turning.
        if (state.r, state.c) == goal && state.run >= min_run {
            return Some(cost);
        }

        // Straight: keep the run going while it still fits the window.
        if state.run < max_run
            && let Some((r, c)) = step((state.r, state.c), state.dir)
        {
            relax(
                grid,
                &mut dist,
                &mut heap,
                cost,
                State {
                    r,
                    c,
                    dir: state.dir,
                    run: state.run + 1,
                },
            );
        }

        // Turn: only after the run meets the minimum, always restarting at 1.
        if state.run >= min_run {
            for dir in [state.dir.left(), state.dir.right()] {
                if let Some((r, c)) = step((state.r, state.c), dir) {
                    relax(
                        grid,
                        &mut dist,
                        &mut heap,
                        cost,
                        State { r, c, dir, run: 1 },
                    );
                }
            }
        }
    }

    None
}

impl aoc::Day for Day17 {
    const DAY: u32 = 17;
    type Output = u64;

    fn part1(input: &str) -> u64 {
        Grid::parse(input)
            .and_then(|grid| shortest(&grid, 1, 3))
            .unwrap_or(0)
    }

    fn part2(input: &str) -> u64 {
        Grid::parse(input)
            .and_then(|grid| shortest(&grid, 4, 10))
            .unwrap_or(0)
    }
}

fn main() {
    aoc::run!(Day17);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    // small grid: only the min 4-run rule makes it work (goal reached on run 4)
    const EXAMPLE2: &str = indoc! {"
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991
    "};

    #[test]
    fn p1() {
        assert_eq!(Day17::part1(EXAMPLE), 102);
    }

    #[test]
    fn p2() {
        assert_eq!(Day17::part2(EXAMPLE), 94);
    }

    #[test]
    fn p2_min_run() {
        assert_eq!(Day17::part2(EXAMPLE2), 71);
    }
}
