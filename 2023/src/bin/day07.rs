use std::cmp::Reverse;
use std::mem;
use std::str::FromStr;

/// Part 2 demotes the jack to the weakest card and lets it stand in for any
/// other when deciding a hand's type.
#[derive(Clone, Copy, Debug)]
enum Rule {
    Standard,
    Jokers,
}

/// Declaration order is the card ranking, weakest first, and `derive(Ord)`
/// reads it. The discriminants stay implicit (`0..13`) so `card as usize`
/// indexes a histogram — see [`Hand::hand_type`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    /// The ace is the last variant, so its discriminant is one short of the
    /// number of labels — no constant to keep in sync.
    const COUNT: usize = Card::Ace as usize + 1;

    /// How a card compares when breaking a tie. Under [`Rule::Jokers`] the jack
    /// drops below the two and everything under it shifts up one, so the rest
    /// of the ranking is untouched.
    fn rank(self, rule: Rule) -> u8 {
        match (rule, self) {
            (Rule::Jokers, Card::Jack) => 0,
            (Rule::Jokers, card) if card < Card::Jack => card as u8 + 1,
            (_, card) => card as u8,
        }
    }
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(format!("bad card {c:?}")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    /// A hand's type depends only on how often labels repeat, never on which
    /// labels or where they sit, so the hand collapses to a histogram and then
    /// to its counts sorted high-to-low — `[3, 2, ..]` is a full house whether
    /// it is `23332` or `QQQAA`.
    fn hand_type(&self, rule: Rule) -> HandType {
        let mut counts = [0u8; Card::COUNT];
        for card in self.cards {
            counts[card as usize] += 1;
        }
        // Jokers have no group of their own; they all join the largest one,
        // which is always the strongest type reachable from this hand.
        let jokers = match rule {
            Rule::Jokers => mem::take(&mut counts[Card::Jack as usize]),
            Rule::Standard => 0,
        };
        counts.sort_by_key(|&count| Reverse(count));
        counts[0] += jokers;

        match counts {
            [5, ..] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    /// Type first, then card by card from the left — which is exactly how
    /// tuples and arrays already compare. A joker is wild for the type but
    /// still a jack here, so the ranks come from the cards as written.
    fn strength(&self, rule: Rule) -> (HandType, [u8; 5]) {
        (self.hand_type(rule), self.cards.map(|card| card.rank(rule)))
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or("missing ' '")?;
        let cards: Vec<Card> = cards
            .chars()
            .map(Card::try_from)
            .collect::<Result<_, _>>()?;

        Ok(Hand {
            cards: cards.try_into().map_err(|_| "expected 5 cards")?,
            bid: bid.parse().map_err(|e| format!("bad bid {bid:?}: {e}"))?,
        })
    }
}

#[derive(Debug)]
struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn winnings(mut self, rule: Rule) -> usize {
        self.hands.sort_by_key(|hand| hand.strength(rule));
        self.hands
            .into_iter()
            .zip(1..)
            .map(|(hand, rank)| rank * hand.bid)
            .sum()
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Game {
            hands: s.lines().map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

struct Day07;

impl aoc::Day for Day07 {
    const DAY: u32 = 7;
    type Output = usize;

    fn part1(input: &str) -> usize {
        let game = input.parse::<Game>().expect("parsed Game");
        game.winnings(Rule::Standard)
    }

    fn part2(input: &str) -> usize {
        let game = input.parse::<Game>().expect("parsed Game");
        game.winnings(Rule::Jokers)
    }
}

fn main() {
    aoc::run!(Day07);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Day;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn p1() {
        assert_eq!(Day07::part1(EXAMPLE), 6440);
    }

    #[test]
    fn p2() {
        assert_eq!(Day07::part2(EXAMPLE), 5905);
    }

    #[test]
    fn parses() {
        let game: Game = EXAMPLE.parse().unwrap();
        assert_eq!(game.hands.len(), 5);
        assert_eq!(
            game.hands[3].cards,
            [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten]
        );
        assert_eq!(game.hands[3].bid, 220);
    }
}
