#![allow(clippy::must_use_candidate, clippy::missing_panics_doc, clippy::identity_op)]

fn calculate_distances(galaxies: &Vec<usize>, columns_to_add: usize) -> usize {
    galaxies.iter()
        .enumerate()
        .filter(|(_, g)| **g != 0) // Remove empty rows
        .map(|(i, g)| {
            (i + 1..galaxies.len())
                .scan(0,|traversed_distance, j| {
                    *traversed_distance += if galaxies[j] == 0 { columns_to_add } else { 1 };
                    Some(*traversed_distance * *g * galaxies[j])
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    // Read input
    let map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // Calculate galaxies per row
    let galaxies_per_row = map
        .iter()
        .map(|row| row.iter().filter(|&c| *c == '#').count())
        .collect::<Vec<_>>();

    // Calculate galaxies per column
    let galaxies_per_column = (0..map[0].len())
        .map(|j| map.iter().filter(|row| row[j] == '#').count())
        .collect::<Vec<_>>();

    // Calculate distances
    calculate_distances(&galaxies_per_row, 2) + calculate_distances(&galaxies_per_column, 2)
}

pub fn part2(input: &str) -> usize {
    // Read input
    let map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // Calculate galaxies per row
    let galaxies_per_row = map
        .iter()
        .map(|row| row.iter().filter(|&c| *c == '#').count())
        .collect::<Vec<_>>();

    // Calculate galaxies per column
    let galaxies_per_column = (0..map[0].len())
        .map(|j| map.iter().filter(|row| row[j] == '#').count())
        .collect::<Vec<_>>();

    // Calculate distances
    calculate_distances(&galaxies_per_row, 1_000_000) + calculate_distances(&galaxies_per_column, 1_000_000)
}

pub fn main() {
    let input = std::fs::read_to_string("input/day11.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
