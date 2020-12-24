use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE
}

const DIRECTIONS: [Direction; 6] = [Direction::E, Direction::SE, Direction::SW, Direction::W, Direction::NW, Direction::NE];

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn step(&self, direction: Direction) -> Self {
        match direction {
            Direction::E => Coordinate {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z
            },
            Direction::SE => Coordinate {
                x: self.x,
                y: self.y - 1,
                z: self.z + 1
            },
            Direction::SW => Coordinate {
                x: self.x - 1,
                y: self.y,
                z: self.z + 1
            },
            Direction::W => Coordinate {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z
            },
            Direction::NW => Coordinate {
                x: self.x,
                y: self.y + 1,
                z: self.z - 1
            },
            Direction::NE => Coordinate {
                x: self.x + 1,
                y: self.y,
                z: self.z - 1
            },
        }
    }
}

struct Floor {
    black_tiles: HashSet<Coordinate>,
}

impl Floor {
    fn new(tiles: &Vec<Vec<Direction>>) -> Self {
        let mut black_tiles = HashSet::new();

        for tile in tiles {
            let mut coord = Coordinate::default();

            for &direction in tile {
                coord = coord.step(direction);
            }

            if black_tiles.contains(&coord) {
                black_tiles.remove(&coord);
            } else {
                black_tiles.insert(coord);
            }
        }

        Self { black_tiles }
    }

    fn sim(&mut self) {
        let mut num_neighbors = HashMap::new();

        self.black_tiles
            .iter()
            .flat_map(|coord| DIRECTIONS.iter().map(move |&dir| coord.step(dir)))
            .for_each(|coord| *num_neighbors.entry(coord).or_default() += 1);

        self.black_tiles
            .retain(|coord| (1..=2).contains(num_neighbors.get(coord).unwrap_or(&0)));

        self.black_tiles
            .extend(num_neighbors.into_iter().filter_map(|(coord, neighbors)| {
                if neighbors == 2 {
                    Some(coord)
                } else {
                    None
                }
            }));
    }

    fn run(&mut self) -> usize {
        for _ in 0..100 {
            self.sim();
        }

        self.black_tiles.len()
    }
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| {
            let mut result = Vec::new();
            let mut chars = line.chars();

            while let Some(c) = chars.next() {
                let dir = match c {
                    'e' => Direction::E,
                    's' => match chars.next().unwrap() {
                        'e' => Direction::SE,
                        'w' => Direction::SW,
                        _ => unreachable!()
                    },
                    'w' => Direction::W,
                    'n' => match chars.next().unwrap() {
                        'w' => Direction::NW,
                        'e' => Direction::NE,
                        _ => unreachable!()
                    },
                    _ => unreachable!(),
                };

                result.push(dir);
            }

            result
        })
        .collect()
}

#[aoc(day24, part1)]
fn part1(tiles: &Vec<Vec<Direction>>) -> usize {
    Floor::new(tiles).black_tiles.len()
}

#[aoc(day24, part2)]
fn part2(tiles: &Vec<Vec<Direction>>) -> usize {
    Floor::new(tiles).run()
}
