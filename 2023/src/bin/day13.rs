struct Day13;

/// One bit per cell of a line: rock set, ash clear.
type Sig = u64;

struct Pattern {
    rows: Vec<Sig>,
    cols: Vec<Sig>,
}

/// Split the notes into self-contained patterns, each as row + column signatures.
fn parse(input: &str) -> impl Iterator<Item = Pattern> + '_ {
    input.trim().split("\n\n").map(|problem| {
        let width = problem.lines().next().map_or(0, str::len);

        let rows: Vec<Sig> = problem
            .lines()
            .map(|line| {
                line.chars()
                    .enumerate()
                    .fold(0u64, |acc, (i, c)| acc | (u64::from(c == '#') << i))
            })
            .collect();

        let cols: Vec<Sig> = (0..width)
            .map(|col_i| {
                rows.iter()
                    .enumerate()
                    .fold(0u64, |acc, (row_i, &row_sig)| {
                        acc | (((row_sig >> col_i) & 1) << row_i)
                    })
            })
            .collect();
        Pattern { rows, cols }
    })
}

/// Lines before the seam whose reflection has exactly `smudges` mismatching
/// cells, or `None` if no seam does.
fn reflection(sigs: &[Sig], smudges: u32) -> Option<usize> {
    (1..sigs.len()).find(|&i| {
        // Cells before candidate seam `i`, nearest-first, paired with cells
        // after it, nearest-first: walking outward from the seam in lockstep.
        let mismatches: u32 = sigs
            .iter()
            .take(i)
            .rev()
            .zip(sigs.iter().skip(i))
            // XOR leaves the differing bits set; count_ones tallies them.
            .map(|(a, b)| (a ^ b).count_ones())
            .sum();
        mismatches == smudges
    })
}

impl Pattern {
    /// `cols` before the seam, or `100 *` the `rows` above it.
    fn score(&self, smudges: u32) -> usize {
        reflection(&self.cols, smudges)
            .or_else(|| reflection(&self.rows, smudges).map(|r| r * 100))
            .unwrap_or(0)
    }
}

impl aoc::Day for Day13 {
    const DAY: u32 = 13;
    type Output = usize;

    fn part1(input: &str) -> usize {
        parse(input).map(|p| p.score(0)).sum()
    }

    fn part2(input: &str) -> usize {
        parse(input).map(|p| p.score(1)).sum()
    }
}

fn main() {
    aoc::run!(Day13);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn parses_signatures() {
        let patterns: Vec<_> = parse(EXAMPLE).collect();
        assert_eq!(patterns.len(), 2);

        let p1 = &patterns[0];
        assert_eq!(p1.rows.len(), 7);
        assert_eq!(p1.cols.len(), 9);
        assert_eq!(p1.rows[0], 0b1100_1101);
        assert_eq!(p1.cols[0], 0b100_1101);
    }

    #[test]
    fn p1() {
        assert_eq!(Day13::part1(EXAMPLE), 405);
    }

    #[test]
    fn p2() {
        assert_eq!(Day13::part2(EXAMPLE), 400);
    }
}
