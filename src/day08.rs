#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::{collections::HashMap, string::ToString};

use itertools::{FoldWhile, Itertools};
use num::Integer;

fn parse_input(input: &str) -> (String, HashMap<String, Vec<String>>) {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let map = nodes
        .lines()
        .map(|line| {
            let (node, children) = line.split_once(" = ").unwrap();
            let children = children
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(", ")
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            (node.to_string(), children)
        })
        .collect::<HashMap<_, _>>();

    (instructions.to_string(), map)
}

pub fn part1(input: &str) -> u64 {
    let (instructions, map) = parse_input(input);
    instructions
        .chars()
        .cycle()
        .fold_while((0, "AAA".to_string()), |(steps, mut current), instruction| {
            if current == "ZZZ" {
                return FoldWhile::Done((steps, current));
            }

            let children = map.get(&current).unwrap();
            current = match instruction {
                'L' => children[0].clone(),
                'R' => children[1].clone(),
                _ => unreachable!(),
            };

            FoldWhile::Continue((steps + 1, current))
        })
        .into_inner()
        .0
}

pub fn part2(input: &str) -> u64 {
    let (instructions, map) = parse_input(input);
    map.keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .map(|mut current| {
            instructions
                .chars()
                .cycle()
                .fold_while(0, |steps, instruction| {
                    if current.ends_with('Z') {
                        return FoldWhile::Done(steps);
                    }

                    let children = map.get(&current).unwrap();
                    current = match instruction {
                        'L' => children[0].clone(),
                        'R' => children[1].clone(),
                        _ => unreachable!(),
                    };

                    FoldWhile::Continue(steps + 1)
                })
                .into_inner()
        })
        .fold(1, |acc, current| acc.lcm(&current))
}

pub fn main() {
    let input = std::fs::read_to_string("input/day08.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
