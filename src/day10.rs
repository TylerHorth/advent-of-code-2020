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
    result.sort_unstable();
    result.push(result.last().unwrap() + 3);

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
    sorted_with_edges(adapters)
        .iter()
        .tuple_windows()
        .fold((1u64, 0u64, 0u64), |(diff1, diff2, diff3), (a, b)| {
            match b - a {
                1 => (diff1 + diff2 + diff3, diff1, diff2),
                2 => (diff1 + diff2, 0, diff1),
                3 => (diff1, 0, 0),
                _ => unreachable!()
            }
        })
        .0
}