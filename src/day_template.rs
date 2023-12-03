#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

pub fn part1(input: &str) -> u64 {
    0
}

pub fn part2(input: &str) -> u64 {
    0
}

pub fn main() {
    let input = std::fs::read_to_string("input/day00.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
