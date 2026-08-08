use std::str::FromStr;

/// `T` is chosen by the call site, so one parser fills any collection.
fn parse_numbers<T: FromIterator<u64>>(s: &str) -> Result<T, String> {
    s.split_whitespace()
        .map(|n| n.parse().map_err(|e| format!("{n:?}: {e}")))
        .collect()
}

#[derive(Clone, Copy, Debug)]
struct Span {
    start: u64,
    end: u64, // exclusive
}

#[derive(Clone, Copy, Debug)]
struct Rule {
    dest: u64,
    source: u64,
    len: u64,
}

#[derive(Debug)]
struct Map {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u64> = parse_numbers(line)?;
        match nums[..] {
            [dest, source, len] => Ok(Rule { dest, source, len }),
            _ => Err(format!("expected 3 numbers: {line:?}")),
        }
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(block: &str) -> Result<Self, Self::Err> {
        let (_, rules) = block.split_once('\n').ok_or("missing header")?;
        rules
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map(|rules| Map { rules })
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, rest) = s.split_once("\n\n").ok_or("missing seed list")?;
        let (_, seeds) = header.split_once(':').ok_or("missing ':'")?;
        Ok(Almanac {
            seeds: parse_numbers(seeds)?,
            maps: rest
                .split("\n\n")
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Span {
    /// `None` when the two ends meet, so an empty `Span` never exists and
    /// callers can drop the degenerate cases with `flatten`.
    fn between(start: u64, end: u64) -> Option<Self> {
        (start < end).then_some(Span { start, end })
    }

    /// Every span in the input is written as a start and a length.
    fn new(start: u64, len: u64) -> Option<Self> {
        Self::between(start, start + len)
    }
}

impl Rule {
    /// The covered part shifted into the destination, plus the parts left
    /// over for the remaining rules to claim.
    fn split(&self, span: Span) -> (Option<Span>, Vec<Span>) {
        let Some(overlap) = Span::between(
            span.start.max(self.source),
            span.end.min(self.source + self.len),
        ) else {
            return (None, vec![span]);
        };

        let shift = |value| value - self.source + self.dest;
        let leftovers = [
            Span::between(span.start, overlap.start),
            Span::between(overlap.end, span.end),
        ];
        (
            Span::between(shift(overlap.start), shift(overlap.end)),
            leftovers.into_iter().flatten().collect(),
        )
    }
}

impl Map {
    fn apply_span(&self, span: Span) -> Vec<Span> {
        let mut unclaimed = vec![span];
        let mut mapped = Vec::new();
        for rule in &self.rules {
            // A leftover is only unmapped as far as *this* rule knows, so it
            // goes back on the worklist for the rules that follow.
            let mut leftovers = Vec::new();
            for span in unclaimed {
                let (hit, rest) = rule.split(span);
                mapped.extend(hit);
                leftovers.extend(rest);
            }
            unclaimed = leftovers;
        }
        // Anything no rule claimed maps to itself.
        mapped.extend(unclaimed);
        mapped
    }

    fn apply_spans(&self, spans: Vec<Span>) -> Vec<Span> {
        spans.into_iter().flat_map(|s| self.apply_span(s)).collect()
    }
}

impl Almanac {
    fn seeds_as_values(&self) -> Vec<Span> {
        self.seeds
            .iter()
            .filter_map(|&start| Span::new(start, 1))
            .collect()
    }

    fn seeds_as_ranges(&self) -> Vec<Span> {
        self.seeds
            .chunks_exact(2)
            .filter_map(|pair| Span::new(pair[0], pair[1]))
            .collect()
    }

    fn lowest_location(&self, seeds: Vec<Span>) -> u64 {
        self.maps
            .iter()
            .fold(seeds, |spans, map| map.apply_spans(spans))
            .into_iter()
            .map(|span| span.start)
            .min()
            .expect("at least one seed")
    }
}

struct Day05;

impl aoc::Day for Day05 {
    const DAY: u32 = 5;
    type Output = u64;

    fn part1(input: &str) -> u64 {
        let almanac = input.parse::<Almanac>().expect("parsed almanac");
        almanac.lowest_location(almanac.seeds_as_values())
    }

    fn part2(input: &str) -> u64 {
        let almanac = input.parse::<Almanac>().expect("parsed almanac");
        almanac.lowest_location(almanac.seeds_as_ranges())
    }
}

fn main() {
    aoc::run!(Day05);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn p1() {
        assert_eq!(Day05::part1(EXAMPLE), 35);
    }

    #[test]
    fn p2() {
        assert_eq!(Day05::part2(EXAMPLE), 46);
    }

    #[test]
    fn parses() {
        let almanac: Almanac = EXAMPLE.parse().unwrap();
        assert_eq!(almanac.seeds, [79, 14, 55, 13]);
        assert_eq!(almanac.maps.len(), 7);
        assert_eq!(almanac.maps[0].rules.len(), 2);
    }
}
