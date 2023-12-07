#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::cast_possible_truncation
)]

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    N(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char, with_joker: bool) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => {
                if with_joker {
                    Self::Joker
                } else {
                    Self::J
                }
            },
            'T' => Self::T,
            n => Self::N(n.to_digit(10).unwrap() as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_cards(cards: &[Card]) -> Self {
        match cards
            .iter()
            .counts()
            .iter()
            .map(|(&k, &v)| (v, k))
            .sorted_unstable()
            .rev()
            .collect::<Vec<_>>()
            .as_slice()
        {
            // Five of a kind
            [(5, _)] => Self::FiveOfAKind,
            // Four of a kind with joker
            [(4, _), (_, Card::Joker)] => Self::FiveOfAKind,

            // Four of a kind with joker
            [(4, Card::Joker), (_, _)] => Self::FiveOfAKind,
            // Four of a kind
            [(4, _), (_, _)] => Self::FourOfAKind,

            // Full house with joker
            [(3, _), (2, Card::Joker)] => Self::FiveOfAKind,
            // Full house with joker
            [(3, Card::Joker), (2, _)] => Self::FiveOfAKind,
            // Full house
            [(3, _), (2, _)] => Self::FullHouse,

            // Three of a kind with joker
            [(3, _), (_, _), (_, Card::Joker)] => Self::FourOfAKind,
            // Three of a kind with joker
            [(3, _), (_, Card::Joker), (_, _)] => Self::FourOfAKind,
            // Three of a kind with joker
            [(3, Card::Joker), (_, _), (_, _)] => Self::FourOfAKind,
            // Three of a kind
            [(3, _), (_, _), (_, _)] => Self::ThreeOfAKind,

            // Two pair with joker
            [(2, _), (2, _), (_, Card::Joker)] => Self::FullHouse,
            // Two pair with joker
            [(2, _), (2, Card::Joker), (_, _)] => Self::FourOfAKind,
            // Two pair with joker
            [(2, Card::Joker), (2, _), (_, _)] => Self::FourOfAKind,
            // Two pair
            [(2, _), (2, _), (_, _)] => Self::TwoPair,

            // One pair with joker
            [(2, _), (_, _), (_, _), (_, Card::Joker)] => Self::ThreeOfAKind,
            // One pair with joker
            [(2, _), (_, _), (_, Card::Joker), (_, _)] => Self::ThreeOfAKind,
            // One pair with joker
            [(2, _), (_, Card::Joker), (_, _), (_, _)] => Self::ThreeOfAKind,
            // One pair with joker
            [(2, Card::Joker), (_, _), (_, _), (_, _)] => Self::ThreeOfAKind,
            // One pair
            [(2, _), (_, _), (_, _), (_, _)] => Self::OnePair,

            // High card with joker
            [(1, _), (_, _), (_, _), (_, _), (_, Card::Joker)] => Self::OnePair,
            // High card with joker
            [(1, _), (_, _), (_, _), (_, Card::Joker), (_, _)] => Self::OnePair,
            // High card with joker
            [(1, _), (_, _), (_, Card::Joker), (_, _), (_, _)] => Self::OnePair,
            // High card with joker
            [(1, _), (_, Card::Joker), (_, _), (_, _), (_, _)] => Self::OnePair,
            // High card with joker
            [(1, Card::Joker), (_, _), (_, _), (_, _), (_, _)] => Self::OnePair,
            // High card
            [(1, _), (_, _), (_, _), (_, _), (_, _)] => Self::HighCard,

            hand => panic!("Invalid hand {hand:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        let hand_type = HandType::from_cards(&cards);
        Self { cards, hand_type }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    match c1.cmp(c2) {
                        std::cmp::Ordering::Equal => continue,
                        o => return o.reverse(),
                    }
                }
                std::cmp::Ordering::Equal
            },
            o => o,
        }
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<usize>().unwrap();
            let hand = Hand::new(cards.chars().map(|card| Card::from_char(card, false)).collect());
            (hand, bid)
        })
        .sorted_unstable_by_key(|(hand, _)| hand.clone())
        .rev()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<usize>().unwrap();
            let hand = Hand::new(cards.chars().map(|card| Card::from_char(card, true)).collect());
            (hand, bid)
        })
        .sorted_by_key(|(hand, _)| hand.clone())
        .rev()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day07.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
