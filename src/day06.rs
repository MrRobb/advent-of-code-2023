#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::str::Lines;

use rayon::prelude::*;

fn parse_numbers_str<'a>(input: &'a mut Lines<'_>) -> &'a str {
    input.next().unwrap().split_once(": ").unwrap().1
}

pub fn part1(input: &str) -> usize {
    let mut input = input.lines();
    let time = parse_numbers_str(&mut input)
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distances = parse_numbers_str(&mut input)
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    time.iter()
        .zip(distances.iter())
        .map(|(t, d)| (0..*t).filter(|i| i * (t - i) > *d).count())
        .product()
}

pub fn part2(input: &str) -> usize {
    let mut input = input.lines();
    let time = parse_numbers_str(&mut input).replace(' ', "").parse::<u64>().unwrap();
    let distances = parse_numbers_str(&mut input).replace(' ', "").parse::<u64>().unwrap();
    (0..time).into_par_iter().filter(|i| i * (time - i) > distances).count()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day06.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
