#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::cast_possible_truncation
)]

fn get_points(line: &str) -> usize {
    let (_, numbers) = line.split_once(": ").unwrap();
    let (winning, yours) = numbers.split_once(" | ").unwrap();
    let winning = winning
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let yours = yours
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    yours.iter().filter(|&number| winning.contains(number)).count()
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(get_points)
        .filter(|&points| points > 0)
        .map(|points| 2u64.pow(points as u32 - 1))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(get_points)
        .enumerate()
        .fold(vec![1; input.lines().count()], |mut how_many_cards, (card, points)| {
            let cards = how_many_cards[card];
            for n in &mut how_many_cards[card + 1..card + 1 + points] {
                *n += cards;
            }
            how_many_cards
        })
        .iter()
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day04.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
