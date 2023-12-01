#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

fn find_digit(line: &str, reverse: bool) -> Option<u64> {
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5",
        "6", "7", "8", "9",
    ];
    let mut min_position = None;
    let mut min_digit = None;

    for digit in &digits {
        if reverse {
            if let Some(position) = line.rfind(digit) {
                if position >= min_position.unwrap_or(position) {
                    min_position = Some(position);
                    min_digit = Some(digit);
                }
            }
        } else if let Some(position) = line.find(digit) {
            if position <= min_position.unwrap_or(position) {
                min_position = Some(position);
                min_digit = Some(digit);
            }
        }
    }

    min_digit.map(|digit| match *digit {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse::<u64>().unwrap(),
    })
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line: &str| {
            let first = u64::from(line.chars().find(char::is_ascii_digit).unwrap().to_digit(10).unwrap());
            let last = u64::from(line.chars().rfind(char::is_ascii_digit).unwrap().to_digit(10).unwrap());
            first * 10 + last
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line: &str| {
            let first = find_digit(line, false).unwrap();
            let last = find_digit(line, true).unwrap();
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
