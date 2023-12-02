#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

fn count_cubes(line: &str) -> (u64, u64, u64) {
    let mut cubes = [0, 0, 0];
    let (_, cubes_str) = line.split_once(": ").unwrap();
    let sets = cubes_str.split("; ").collect::<Vec<_>>();
    for set in sets {
        for cube in set.split(", ") {
            let (count, color) = cube.split_once(' ').unwrap();
            let count = count.parse::<u64>().unwrap();
            match color {
                "red" => cubes[0] = count.max(cubes[0]),
                "green" => cubes[1] = count.max(cubes[1]),
                "blue" => cubes[2] = count.max(cubes[2]),
                _ => unreachable!(),
            }
        }
    }
    cubes.into()
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(count_cubes)
        .enumerate()
        .filter(|(_, (red, green, blue))| (*red <= 12 && *green <= 13 && *blue <= 14))
        .map(|(game_id, _)| game_id as u64 + 1)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(count_cubes)
        .map(|(red, green, blue)| red * green * blue)
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day02.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
