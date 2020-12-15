use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

fn sequence_nth(seed: &[u32], n: u32) -> u32 {
    let mut seen = vec![0; n as usize];
    let mut turn = 0;

    for &i in seed {
        turn += 1;
        seen[i as usize] = turn;
    }

    let mut next = 0;

    while turn < n - 1 {
        turn += 1;

        let last_seen = std::mem::replace(&mut seen[next as usize], turn);

        if last_seen == 0 {
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
