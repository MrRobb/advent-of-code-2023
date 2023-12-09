use std::fs::read_to_string;

#[allow(clippy::wildcard_imports)]
use advent_of_code_2023::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench1(c: &mut Criterion) {
    // let input01 = read_to_string("input/day01.txt").expect("Input file not found");
    // c.bench_function("Day 1 | Part 1", |b| b.iter(|| day01::part1(&input01)));
    // c.bench_function("Day 1 | Part 2", |b| b.iter(|| day01::part2(&input01)));

    // let input02 = read_to_string("input/day02.txt").expect("Input file not found");
    // c.bench_function("Day 2 | Part 1", |b| b.iter(|| day02::part1(&input02)));
    // c.bench_function("Day 2 | Part 2", |b| b.iter(|| day02::part2(&input02)));

    // let input03 = read_to_string("input/day03.txt").expect("Input file not found");
    // c.bench_function("Day 3 | Part 1", |b| b.iter(|| day03::part1(&input03)));
    // c.bench_function("Day 3 | Part 2", |b| b.iter(|| day03::part2(&input03)));

    // let input04 = read_to_string("input/day04.txt").expect("Input file not found");
    // c.bench_function("Day 4 | Part 1", |b| b.iter(|| day04::part1(&input04)));
    // c.bench_function("Day 4 | Part 2", |b| b.iter(|| day04::part2(&input04)));

    // let input05 = read_to_string("input/day05.txt").expect("Input file not found");
    // c.bench_function("Day 5 | Part 1", |b| b.iter(|| day05::part1(&input05)));
    // c.bench_function("Day 5 | Part 2", |b| b.iter(|| day05::part2(&input05)));

    // let input06 = read_to_string("input/day06.txt").expect("Input file not found");
    // c.bench_function("Day 6 | Part 1", |b| b.iter(|| day06::part1(&input06)));
    // c.bench_function("Day 6 | Part 2", |b| b.iter(|| day06::part2(&input06)));

    // let input07 = read_to_string("input/day07.txt").expect("Input file not found");
    // c.bench_function("Day 7 | Part 1", |b| b.iter(|| day07::part1(&input07)));
    // c.bench_function("Day 7 | Part 2", |b| b.iter(|| day07::part2(&input07)));

    // let input08 = read_to_string("input/day08.txt").expect("Input file not found");
    // c.bench_function("Day 8 | Part 1", |b| b.iter(|| day08::part1(&input08)));
    // c.bench_function("Day 8 | Part 2", |b| b.iter(|| day08::part2(&input08)));

    let input09 = read_to_string("input/day09.txt").expect("Input file not found");
    c.bench_function("Day 9 | Part 1", |b| b.iter(|| day09::part1(&input09)));
    c.bench_function("Day 9 | Part 2", |b| b.iter(|| day09::part2(&input09)));
}

criterion_group!(benches, bench1);
criterion_main!(benches);
