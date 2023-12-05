#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
struct Mapping {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn map_seeds(seeds: Vec<u64>, mappings: &[Vec<Mapping>]) -> Vec<u64> {
    mappings.iter().fold(seeds, |seeds, mappings| {
        let pb = indicatif::ProgressBar::new(seeds.len() as u64);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{spinner} [{elapsed_precise} / {eta_precise}] {wide_bar:.cyan/blue} {pos}/{len}")
                .unwrap(),
        );
        seeds
            .par_iter()
            .progress_with(pb)
            .map(|seed| {
                mappings
                    // .par_iter()
                    .iter()
                    .find(|mapping| {
                        mapping.source_range_start <= *seed && mapping.source_range_start + mapping.range_length > *seed
                    })
                    .map_or(*seed, |mapping| {
                        mapping.destination_range_start + seed - mapping.source_range_start
                    })
            })
            .collect()
    })
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<Mapping>>) {
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
                .map(|(destination_range_start, source_range_start, range_length)| Mapping {
                    destination_range_start,
                    source_range_start,
                    range_length,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (seeds, mappings)
}

pub fn part1(input: &str) -> u64 {
    let (seeds, mappings) = parse_input(input);
    map_seeds(seeds, &mappings).iter().min().unwrap().to_owned()
}

pub fn part2(input: &str) -> u64 {
    let (seeds, mappings) = parse_input(input);
    let real_seeds = seeds
        .iter()
        .tuples()
        .flat_map(|(start, length)| (*start..*start + *length))
        .collect::<Vec<_>>();
    map_seeds(real_seeds, &mappings).iter().min().unwrap().to_owned()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day05.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
