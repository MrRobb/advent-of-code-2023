# 🎄 Advent of Code 2023

[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/MrRobb/advent-of-code-2023/blob/master/LICENSE)

|        | Problem                                            | Solution                                                                         | Execution time        | Lines of code | Finished |
|--------|----------------------------------------------------|----------------------------------------------------------------------------------|-----------------------|---------------|----------|
| Day 1  | [Problem 1](https://adventofcode.com/2023/day/1)   | [day01.rs](https://github.com/MrRobb/advent-of-code-2023/blob/main/src/day01.rs) | 61.209 µs + 463.01 µs | 37            | ✓        |
| Day 2  | [Problem 2](https://adventofcode.com/2023/day/2)   | [day02.rs](https://github.com/MrRobb/advent-of-code-2023/blob/main/src/day02.rs) | 95.278 µs + 92.764 µs | 46            | ✓        |
| Day 3  | [Problem 3](https://adventofcode.com/2023/day/3)   | [day03.rs](https://github.com/MrRobb/advent-of-code-2023/blob/main/src/day03.rs) | 598.86 µs + 541.84 µs | 51            | ✓        |
| Day 4  | [Problem 4](https://adventofcode.com/2023/day/4)   | [day04.rs](https://github.com/MrRobb/advent-of-code-2023/blob/main/src/day04.rs) | 337.20 µs + 326.75 µs | 48            | ✓        |
| Day 5  | [Problem 5](https://adventofcode.com/2023/day/5)   | [day05.rs](https://github.com/MrRobb/advent-of-code-2023/blob/main/src/day05.rs) | 61.339 µs + 113.32 µs | 92            | ✓        |
| Day 6  | [Problem 6](https://adventofcode.com/2023/day/6)   | [day06.rs](https://github.com/MrRobb/advent-of-code-2023/blob/main/src/day06.rs) | 537.83 ns + 5.8270 ms | 34            | ✓        |
| Day 7  | [Problem 7](https://adventofcode.com/2023/day/7)   | [day07.rs](https://github.com/MrRobb/advent-of-code-2023/blob/main/src/day07.rs) | 2.9551 ms + 2.8716 ms | 155           | ✓        |
| Day 8  | [Problem 8](https://adventofcode.com/2023/day/8)   | [day08.rs](https://github.com/MrRobb/advent-of-code-2023/blob/main/src/day08.rs) | 1.9298 ms + 10.211 ms | 72            | ✓        |
| Day 9  | [Problem 9](https://adventofcode.com/2023/day/9)   | [day09.rs](https://github.com/MrRobb/advent-of-code-2023/blob/main/src/day09.rs) | 572.13 µs + 565.22 µs | 49            | ✓        |


> The benchmarks are measured (non-scientifically) with [cargo-criterion](https://github.com/bheisler/cargo-criterion) on a AMD Ryzen 5 3600 Desktop. More in the [benchmarks](#benchmarks) section.
> The lines of code are measured using [ghloc](https://github.com/MrRobb/ghloc-rs), excluding comments and empty lines.

## Install Rust

If you don't have Rust installed ([how dare you](https://media.giphy.com/media/U1aN4HTfJ2SmgB2BBK/giphy.gif)) just run this:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> If you are not using a Unix-like OS, check the instructions [here](https://www.rust-lang.org/tools/install)
## Usage

### Clone

```sh
git clone https://github.com/MrRobb/advent-of-code-2023.git
cd advent-of-code-2023
```

### Build

```sh
cargo build --release
```

### Run

#### Run all

```sh
cargo run --release
```

#### Run a specific day

```sh
cargo run --release --bin day01
```

## Benchmarks

### Install Criterion

To run the benchmarks you need to install [cargo-criterion](https://github.com/bheisler/cargo-criterion) first:

```sh
cargo install cargo-criterion
```

### Run benchmarks

Once you have Criterion installed, you can run the benchmarks with:

```sh
cargo criterion
```
