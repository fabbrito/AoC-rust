use std::collections::HashMap;

struct Day15;

struct Lens<'a> {
    label: &'a str,
    focal: u8,
}
/// HASH: fold each byte into a rolling value, `* 17` then wrap to a byte per step.
fn hash(s: &str) -> u64 {
    s.bytes()
        .fold(0u64, |acc, b| (acc + u64::from(b)) * 17 % 256)
}

/// Run the init sequence; result maps box number -> lenses, front to back.
fn boxes(input: &str) -> HashMap<u64, Vec<Lens<'_>>> {
    let mut boxes: HashMap<u64, Vec<Lens<'_>>> = HashMap::new();
    for step in input.trim().split(',') {
        // "label=focal": insert or replace.
        if let Some((label, focal)) = step.split_once('=') {
            let Ok(focal) = focal.parse::<u8>() else {
                continue;
            };
            // box for this label, empty Vec if it's the first lens there
            let b = boxes.entry(hash(label)).or_default();
            match b.iter_mut().find(|lens| lens.label == label) {
                Some(lens) => lens.focal = focal, // present: swap focal, keep slot
                None => b.push(Lens { label, focal }), // absent: append behind the rest
            }
        } else {
            // "label-": drop that lens, later ones shift forward (retain keeps order)
            let label = step.trim_end_matches('-');
            boxes
                .entry(hash(label))
                .or_default()
                .retain(|lens| lens.label != label);
        }
    }
    boxes
}

impl aoc::Day for Day15 {
    const DAY: u32 = 15;
    type Output = u64;

    fn part1(input: &str) -> u64 {
        // verification number: sum of HASH over every comma-separated step
        input.trim().split(',').map(hash).sum()
    }

    fn part2(input: &str) -> u64 {
        // focusing power: sum over every lens of (box + 1) * slot * focal.
        // key is the box number, so no outer enumerate; a sum needs no ordering.
        boxes(input)
            .iter()
            .flat_map(|(box_i, lenses)| {
                lenses.iter().enumerate().map(move |(slot, lens)| {
                    (*box_i + 1) * (slot as u64 + 1) * u64::from(lens.focal)
                })
            })
            .sum()
    }
}

fn main() {
    aoc::run!(Day15);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn p1() {
        assert_eq!(Day15::part1(EXAMPLE), 1320);
    }

    #[test]
    fn p2() {
        assert_eq!(Day15::part2(EXAMPLE), 145);
    }
}
