#![allow(clippy::must_use_candidate, clippy::missing_panics_doc, clippy::cast_lossless)]

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line: &str| {
            let first = line.chars().find(char::is_ascii_digit).unwrap().to_digit(10).unwrap();
            let last = line.chars().rfind(char::is_ascii_digit).unwrap().to_digit(10).unwrap();
            first * 10 + last
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .replace("one", "o1e")
        .replace("two", "t2")
        .replace("three", "3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "8t")
        .replace("nine", "9e")
        .lines()
        .map(|line| {
            let first = line.chars().find(char::is_ascii_digit).unwrap().to_digit(10).unwrap();
            let last = line.chars().rfind(char::is_ascii_digit).unwrap().to_digit(10).unwrap();
            first * 10 + last
        })
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day01.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
