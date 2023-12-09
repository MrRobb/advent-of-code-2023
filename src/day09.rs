#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use itertools::Itertools;

fn build_differences(line: &str) -> Vec<Vec<i64>> {
    let values = line
        .split_whitespace()
        .map(|value| value.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    std::iter::successors(Some(values), |values| {
        Some(values.iter().tuple_windows().map(|(a, b)| b - a).collect::<Vec<_>>())
    })
    .take_while(|values| values.iter().any(|value| *value != 0))
    .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            build_differences(line).iter().rev().fold(0, |extrapolated, values| {
                let last_value = values.last().unwrap();
                extrapolated + last_value
            })
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            build_differences(line).iter().rev().fold(0, |extrapolated, values| {
                let first_value = values.first().unwrap();
                first_value - extrapolated
            })
        })
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day09.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
