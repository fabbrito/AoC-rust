/// (row, col).
type Pos = (usize, usize);

/// Galaxy coords with every empty row/column expanded to `factor` of them.
fn galaxies(input: &str, factor: usize) -> Vec<Pos> {
    let width = input.lines().next().map_or(0, str::len);
    let mut unexpanded: Vec<Pos> = Vec::new();
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut occupied_cols = vec![false; width];

    for (r, row) in input.lines().enumerate() {
        if !row.contains('#') {
            empty_rows.push(r);
            continue;
        }
        for (c, _) in row.match_indices('#') {
            if let Some(occupied) = occupied_cols.get_mut(c) {
                *occupied = true;
            }
            unexpanded.push((r, c));
        }
    }

    let empty_cols: Vec<usize> = occupied_cols
        .iter()
        .enumerate()
        .filter_map(|(c, occ)| (!occ).then_some(c))
        .collect();

    let gap = factor.saturating_sub(1);
    let expand = |coord: usize, empties: &[usize]| {
        let crossed = empties.partition_point(|&e| e < coord);
        coord.saturating_add(gap.saturating_mul(crossed))
    };
    unexpanded
        .into_iter()
        .map(|(r, c)| (expand(r, &empty_rows), expand(c, &empty_cols)))
        .collect()
}

/// Sum of Manhattan distances over every unordered pair.
fn sum_distances(galaxies: &[Pos]) -> usize {
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            // `skip` past self is the same pair set as slicing from i + 1,
            // without a range that could outrun the slice.
            galaxies
                .iter()
                .skip(i.saturating_add(1))
                .map(move |b| a.0.abs_diff(b.0).saturating_add(a.1.abs_diff(b.1)))
        })
        .sum()
}

struct Day11;

impl aoc::Day for Day11 {
    const DAY: u32 = 11;
    type Output = usize;

    fn part1(input: &str) -> usize {
        sum_distances(&galaxies(input, 2))
    }
    fn part2(input: &str) -> usize {
        sum_distances(&galaxies(input, 1_000_000))
    }
}

fn main() {
    aoc::run!(Day11);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn p1() {
        assert_eq!(Day11::part1(EXAMPLE), 374);
    }

    #[test]
    fn p2_x10() {
        assert_eq!(sum_distances(&galaxies(EXAMPLE, 10)), 1030);
    }

    #[test]
    fn p2_x100() {
        assert_eq!(sum_distances(&galaxies(EXAMPLE, 100)), 8410);
    }
}
