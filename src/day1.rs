use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashSet, num::ParseIntError};

const TARGET: i32 = 2020;

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day1, part1)]
fn part1(nums: &[i32]) -> i32 {
    let (a, b) = sum2(nums, TARGET).expect("no solution");

    a * b
}

#[aoc(day1, part2)]
fn part2(nums: &[i32]) -> i32 {
    for &num in nums {
        if let Some((a, b)) = sum2(nums, TARGET - num) {
            return num * a * b;
        }
    }

    panic!("no solution")
}

fn sum2(nums: &[i32], sum: i32) -> Option<(i32, i32)> {
    let mut seen = HashSet::new();

    for &num in nums {
        let target = sum - num;

        if seen.contains(&target) {
            return Some((num, target));
        }

        seen.insert(num);
    }

    None
}
