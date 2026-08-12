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
            occupied_cols[c] = true;
            unexpanded.push((r, c));
        }
    }

    let empty_cols: Vec<usize> = occupied_cols
        .iter()
        .enumerate()
        .filter_map(|(c, occ)| (!occ).then_some(c))
        .collect();

    let gap = factor - 1;
    unexpanded
        .into_iter()
        .map(|(r, c)| {
            (
                r + gap * empty_rows.partition_point(|&row| row < r),
                c + gap * empty_cols.partition_point(|&col| col < c),
            )
        })
        .collect()
}

/// Sum of Manhattan distances over every unordered pair.
fn sum_distances(galaxies: &[Pos]) -> usize {
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            galaxies[i + 1..]
                .iter()
                .map(move |b| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
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
