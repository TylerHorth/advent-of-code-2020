use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

struct IntMap {
    data: Vec<u32>,
}

impl IntMap {
    const SENTINEL: u32 = 0;

    fn with_capacity(capacity: u32) -> Self {
        Self { data: vec![0; capacity as usize] }
    }

    fn insert(&mut self, key: u32, value: u32) -> u32 {
        std::mem::replace(&mut self.data[key as usize], value)
    }
}

fn sequence_nth(seed: &[u32], n: u32) -> u32 {
    let mut seen = IntMap::with_capacity(n);
    let mut turn = 0;

    for &i in seed {
        turn += 1;
        seen.insert(i, turn);
    }

    let mut next = 0;

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
fn part1(seed: &[u32]) -> u32 {
    sequence_nth(seed, 2020)
}

#[aoc(day15, part2)]
fn part2(seed: &[u32]) -> u32 {
    sequence_nth(seed, 30000000)
}
