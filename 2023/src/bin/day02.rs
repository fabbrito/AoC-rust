use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Cubes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Cubes::default();
        for piece in s.split(',') {
            let (count, name) = piece
                .trim()
                .split_once(' ')
                .ok_or("expected '<n> <colour>'")?;
            let count: u32 = count.parse().map_err(|_| format!("bad count: {count}"))?;
            match name {
                "red" => cubes.red += count,
                "green" => cubes.green += count,
                "blue" => cubes.blue += count,
                other => return Err(format!("unknown colour: {other}")),
            }
        }
        Ok(cubes)
    }
}

impl Cubes {
    fn fits_in(&self, bag: &Cubes) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    fn max_each(&self, other: &Cubes) -> Cubes {
        Cubes {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Cubes>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(':').ok_or("expected ': '")?;
        let id = left
            .strip_prefix("Game ")
            .ok_or("no id found")?
            .parse()
            .map_err(|e| format!("bad id: {e}"))?;
        let draws = right.split(';').map(str::parse).collect::<Result<_, _>>()?;
        Ok(Game { id, draws })
    }
}

impl Game {
    fn possible_with(&self, bag: &Cubes) -> bool {
        self.minimum_set().fits_in(bag)
    }

    fn minimum_set(&self) -> Cubes {
        self.draws
            .iter()
            .fold(Cubes::default(), |acc, item| acc.max_each(item))
    }
}

struct Day02;

impl aoc::Day for Day02 {
    const DAY: u32 = 2;
    type Output = usize;

    fn part1(input: &str) -> usize {
        const BAG: Cubes = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };
        input
            .lines()
            .map(|s| s.parse::<Game>().expect("bad game"))
            .filter(|g| g.possible_with(&BAG))
            .map(|g| g.id as usize)
            .sum()
    }

    fn part2(input: &str) -> usize {
        input
            .lines()
            .map(|s| s.parse::<Game>().expect("bad game"))
            .map(|g| g.minimum_set().power() as usize)
            .sum()
    }
}

fn main() {
    aoc::run!(Day02);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
      Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
      Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
      Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
      Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
      Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn p1() {
        assert_eq!(Day02::part1(EXAMPLE), 8);
    }

    #[test]
    fn p2() {
        assert_eq!(Day02::part2(EXAMPLE), 2286);
    }

    #[test]
    fn parses() {
        let cubes: Cubes = "3 blue, 4 red".parse().unwrap();
        assert_eq!((cubes.red, cubes.green, cubes.blue), (4, 0, 3));

        let game: Game = "Game 12: 1 red; 2 green, 3 blue".parse().unwrap();
        assert_eq!(game.id, 12);
        assert_eq!(game.draws.len(), 2);
        assert_eq!(game.draws[1].blue, 3);
    }
}
