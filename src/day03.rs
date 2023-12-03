#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

use std::{borrow::BorrowMut, collections::BTreeMap};

use pathfinding::matrix::Matrix;

fn build_part_map(map: &Matrix<char>) -> BTreeMap<(usize, usize), Vec<u64>> {
    let mut part_numbers = BTreeMap::<(usize, usize), Vec<u64>>::new();

    // Filter only part numbers
    for (i, row) in map.iter().enumerate() {
        let mut row_iterator = row.iter().enumerate().peekable();

        while row_iterator.peek().is_some() {
            let number = row_iterator
                .borrow_mut()
                .skip_while(|(_, c)| !c.is_ascii_digit())
                .take_while(|(_, c)| c.is_ascii_digit())
                .collect::<Vec<_>>();

            // Check if is part number
            if let Some(gear) = number
                .iter()
                .flat_map(|(j, _)| map.neighbours((i, *j), true))
                .find(|(i, j)| map[(*i, *j)] != '.' && !map[(*i, *j)].is_numeric())
            {
                // Parse number
                let n = number
                    .into_iter()
                    .map(|(_, c)| c)
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();

                // Add to part numbers
                part_numbers.entry(gear).or_default().push(n);
            }
        }
    }

    part_numbers
}

pub fn part1(input: &str) -> u64 {
    let map = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    let part_numbers = build_part_map(&map);
    part_numbers.values().flatten().sum()
}

pub fn part2(input: &str) -> u64 {
    let map = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    let part_numbers = build_part_map(&map);
    part_numbers
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day03.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
