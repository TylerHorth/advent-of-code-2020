use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

fn sorted_with_edges(adapters: &[u64]) -> Vec<u64> {
    let mut result: Vec<u64> = adapters.iter().copied().collect();

    result.push(0);
    result.push(*adapters.iter().max().unwrap() + 3);
    result.sort();

    result
}

#[aoc(day10, part1)]
fn part1(adapters: &[u64]) -> u64 {
    let (diff1, diff3) = sorted_with_edges(adapters)
        .iter()
        .tuple_windows()
        .fold((0u64, 0u64), |(diff1, diff3), (a, b)| {
            match b - a {
                1 => (diff1 + 1, diff3),
                3 => (diff1, diff3 + 1),
                _ => (diff1, diff3)
            }
        });

    diff1 * diff3
}

#[aoc(day10, part2)]
fn part2(adapters: &[u64]) -> u64 {
    let adapters = sorted_with_edges(adapters);

    let mut dp = [1, 0, 0];

    for i in 1..adapters.len() {
        let mut count = 0;

        for j in i.saturating_sub(3)..i {
            if adapters[i] - adapters[j] <= 3 {                    
                count += dp[j % 3];
            }
        }

        dp[i % 3] = count;
    }

    dp[(adapters.len() - 1) % 3]
}