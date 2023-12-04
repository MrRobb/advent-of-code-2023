#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
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
            u32::try_from(yours.iter().filter(|&number| winning.contains(number)).count()).unwrap()
        })
        .filter(|&points| points > 0)
        .map(|points| 2u64.pow(points - 1))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let how_many_cards =
        input
            .lines()
            .enumerate()
            .fold(vec![0; input.lines().count()], |mut how_many_cards, (card, line)| {
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
                let points = yours.iter().filter(|&number| winning.contains(number)).count();
                let cards = how_many_cards[card];
                how_many_cards[card + 1..card + 1 + points]
                    .iter_mut()
                    .for_each(|n| *n += cards + 1);
                how_many_cards
            });
    how_many_cards.iter().sum::<u64>() + input.lines().count() as u64
}

pub fn main() {
    let input = std::fs::read_to_string("input/day04.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
