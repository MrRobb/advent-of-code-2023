#![allow(clippy::must_use_candidate, clippy::missing_panics_doc, clippy::range_plus_one)]

use std::ops::Range;

use itertools::Itertools;

type Mappings = Vec<Vec<(u64, u64, u64)>>;

fn map_seeds_ranges(seeds: Vec<Range<u64>>, mappings: &Mappings) -> Vec<Range<u64>> {
    mappings.iter().fold(seeds, |seeds: Vec<Range<u64>>, mappings| {
        seeds
            .iter()
            .flat_map(|seed_range| {
                let mut mapped_ranges = Vec::new();

                let final_end = mappings
                    .iter()
                    .filter(|(_, source_range_start, range_length)| {
                        source_range_start + range_length > seed_range.start && *source_range_start < seed_range.end
                    })
                    .sorted_by_key(|(_, source_range_start, _)| *source_range_start)
                    .fold(
                        seed_range.start,
                        |current, (destination_range_start, source_range_start, range_length)| {
                            // From current to start of the mapping
                            if current < *source_range_start {
                                mapped_ranges.push(current..*source_range_start);
                            }

                            // From start of the mapping or start of the seed range
                            // To end of the mapping or end of the seed range
                            let common_start = current.max(*source_range_start);
                            let common_end = seed_range.end.min(source_range_start + range_length);
                            mapped_ranges.push(
                                common_start + destination_range_start - source_range_start
                                    ..common_end + destination_range_start - source_range_start,
                            );

                            // Move to common end
                            common_end
                        },
                    );

                // From end of the last mapping to end of the seed range
                if final_end < seed_range.end {
                    mapped_ranges.push(final_end..seed_range.end);
                }

                mapped_ranges
            })
            .collect()
    })
}

fn parse_input(input: &str) -> (Vec<u64>, Mappings) {
    let mut maps = input.split("\n\n");
    let seeds = maps
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mappings = maps
        .map(|map| {
            map.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .tuples()
                .map(|(a, b, c)| (a, b, c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (seeds, mappings)
}

pub fn part1(input: &str) -> u64 {
    let (seeds, mappings) = parse_input(input);
    map_seeds_ranges(seeds.iter().map(|&seed| seed..seed + 1).collect(), &mappings)
        .iter()
        .min_by_key(|range| range.start)
        .unwrap()
        .start
}

pub fn part2(input: &str) -> u64 {
    let (seeds, mappings) = parse_input(input);
    let real_seeds = seeds
        .iter()
        .tuples()
        .map(|(start, length)| (*start..*start + *length))
        .collect::<Vec<_>>();
    map_seeds_ranges(real_seeds, &mappings)
        .iter()
        .min_by_key(|range| range.start)
        .unwrap()
        .start
}

pub fn main() {
    let input = std::fs::read_to_string("input/day05.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
