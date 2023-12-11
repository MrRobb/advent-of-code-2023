#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use std::collections::BTreeSet;

use pathfinding::matrix::Matrix;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const fn to_direction(self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
    }

    const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeType {
    Vertical,
    Horizontal,
    BendL,
    BendJ,
    Bend7,
    BendF,
    Ground,
    Start,
}

impl NodeType {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::BendL,
            'J' => Self::BendJ,
            '7' => Self::Bend7,
            'F' => Self::BendF,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }

    const fn move_to(self, from_direction: Direction) -> Option<Direction> {
        match (self, from_direction) {
            (Self::Vertical, Direction::South) | (Self::BendJ, Direction::West) | (Self::BendL, Direction::East) => {
                Some(Direction::North)
            },
            (Self::Horizontal, Direction::West) | (Self::BendL, Direction::North) | (Self::BendF, Direction::South) => {
                Some(Direction::East)
            },
            (Self::Horizontal, Direction::East) | (Self::BendJ, Direction::North) | (Self::Bend7, Direction::South) => {
                Some(Direction::West)
            },

            (Self::Bend7, Direction::West) | (Self::Vertical, Direction::North) | (Self::BendF, Direction::East) => {
                Some(Direction::South)
            },
            _ => None,
        }
    }
}

fn find_path(map: &Matrix<NodeType>) -> Vec<((usize, usize), Direction)> {
    let starting_position = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&node| node == NodeType::Start).map(|x| (y, x)))
        .unwrap();

    [Direction::North, Direction::South, Direction::East, Direction::West]
        .into_iter()

        // Follow the pipe
        .map(|starting_direction| {
            std::iter::successors(
                Some((starting_position, starting_direction)),
                |(position, direction)| {
                    map.move_in_direction(*position, direction.to_direction())
                        .and_then(|new_position| {
                            map[new_position]
                                .move_to(direction.opposite())
                                .map(|new_direction| (new_position, new_direction))
                        })
                },
            )
            .collect::<Vec<_>>()
        })

        // Find the path that ends in the starting position
        .find(|path| {
            let (last_position, last_direction) = path.last().unwrap();
            map.move_in_direction(*last_position, last_direction.to_direction()) == Some(starting_position)
        }).unwrap()
}

pub fn part1(input: &str) -> usize {
    let map = Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().map(NodeType::from_char).collect::<Vec<_>>()),
    )
    .unwrap();

    find_path(&map).len().div_ceil(2)
}

pub fn part2(input: &str) -> usize {
    let map = Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().map(NodeType::from_char).collect::<Vec<_>>()),
    )
    .unwrap();

    //  Calculate path
    let path = find_path(&map);

    // Extract the directions of start
    let start_directions = [path.first().unwrap().1, path.last().unwrap().1];

    let path = path.into_iter().map(|(position, _)| position).collect::<BTreeSet<_>>();

    // Apply even-odd algorithm
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            let mut is_inside = false;
            row.iter()
                .enumerate()
                .filter_map(|(j, node)| {
                    if !path.contains(&(i, j)) {
                        return if is_inside { Some(()) } else { None };
                    }

                    match *node {
                        NodeType::Vertical | NodeType::BendL | NodeType::BendJ => is_inside = !is_inside,
                        NodeType::Start if start_directions.contains(&Direction::North) => is_inside = !is_inside,
                        _ => {},
                    }

                    None
                })
                .count()
        })
        .sum()
}

pub fn main() {
    let input = std::fs::read_to_string("input/day10.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", part1(&input));
    println!("PART 2 = {}", part2(&input));
    println!("Execution time: {:?}", now.elapsed());
}
