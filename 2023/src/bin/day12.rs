use std::collections::HashMap;

struct Day12;

fn parse(line: &str) -> (Vec<u8>, Vec<usize>) {
    let (springs, groups) = line.split_once(' ').unwrap();
    let groups: Vec<usize> = groups.split(',').map(|n| n.parse().unwrap()).collect();
    (springs.as_bytes().to_vec(), groups)
}

fn unfold(springs: &[u8], groups: &[usize]) -> (Vec<u8>, Vec<usize>) {
    ([springs; 5].join(&b'?'), groups.repeat(5))
}

fn rows(input: &str) -> impl Iterator<Item = (Vec<u8>, Vec<usize>)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse)
}

type Memo = HashMap<(usize, usize), usize>;

/// Every legal way to lay `groups` over `springs`, in order.
fn arrangements(springs: &[u8], groups: &[usize]) -> usize {
    count(springs, groups, &mut Memo::new())
}

fn count(springs: &[u8], groups: &[usize], memo: &mut Memo) -> usize {
    // nothing left to place: this row works iff no damaged spring is left unaccounted for
    let Some(&size) = groups.first() else {
        return usize::from(!springs.contains(&b'#'));
    };

    // both args are always suffixes of the row, so the two lengths identify the state
    let key = (springs.len(), groups.len());
    if let Some(&hit) = memo.get(&key) {
        return hit;
    }

    // no room left for this group -> dead end
    let Some(last_start) = springs.len().checked_sub(size) else {
        return 0;
    };

    let mut total = 0;
    for start in 0..=last_start {
        let block_intact = !springs[start..start + size].contains(&b'.');
        let separator_clear = springs.get(start + size) != Some(&b'#');

        if block_intact && separator_clear {
            // +1 skips the separator; get(range) yields None past the end, so empty tail
            let rest = springs.get(start + size + 1..).unwrap_or_default();
            total += count(rest, &groups[1..], memo);
        }

        // sliding past a damaged spring would abandon it -> no later start is legal
        if springs[start] == b'#' {
            break;
        }
    }

    memo.insert(key, total);
    total
}

impl aoc::Day for Day12 {
    const DAY: u32 = 12;
    type Output = usize;

    fn part1(input: &str) -> usize {
        rows(input)
            .map(|(springs, groups)| arrangements(&springs, &groups))
            .sum()
    }

    fn part2(input: &str) -> usize {
        rows(input)
            .map(|(springs, groups)| unfold(&springs, &groups))
            .map(|(springs, groups)| arrangements(&springs, &groups))
            .sum()
    }
}

fn main() {
    aoc::run!(Day12);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn parses_a_row() {
        let (springs, groups) = parse("???.### 1,1,3");
        assert_eq!(springs, b"???.###");
        assert_eq!(groups, [1, 1, 3]);
    }

    #[test]
    fn unfolds_a_row() {
        let (springs, groups) = parse(".# 1");
        let (springs, groups) = unfold(&springs, &groups);
        assert_eq!(springs, b".#?.#?.#?.#?.#");
        assert_eq!(groups, [1, 1, 1, 1, 1]);
    }

    #[test]
    fn p1() {
        assert_eq!(Day12::part1(EXAMPLE), 21);
    }

    #[test]
    fn p2() {
        assert_eq!(Day12::part2(EXAMPLE), 525_152);
    }
}
