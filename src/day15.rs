use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

struct IntMap {
    data: Vec<usize>,
}

impl IntMap {
    const SENTINEL: usize = 0;

    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn insert(&mut self, key: usize, value: usize) -> usize {
        if key >= self.data.len() {
            self.data.extend((self.data.len()..key).map(|_| Self::SENTINEL));
            self.data.push(value);

            Self::SENTINEL
        } else {
            std::mem::replace(&mut self.data[key], value)
        }
    }
}

fn sequence_nth(seed: &[usize], n: usize) -> usize {
    let mut seen = IntMap::new();
    let mut turn = 0usize;

    for &i in seed {
        turn += 1;
        seen.insert(i, turn);
    }

    let mut next = 0usize;

    while turn < n - 1 {
        turn += 1;
        let last_seen = seen.insert(next, turn);
        if last_seen == IntMap::SENTINEL {
            next = 0;
        } else {
            next = turn - last_seen;
        }
    }

    next
}

#[aoc(day15, part1)]
fn part1(seed: &[usize]) -> usize {
    sequence_nth(seed, 2020)
}

#[aoc(day15, part2)]
fn part2(seed: &[usize]) -> usize {
    sequence_nth(seed, 30000000)
}
