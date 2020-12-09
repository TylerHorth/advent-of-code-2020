use std::{collections::HashSet, num::ParseIntError};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

const WINDOW_SIZE: usize = 25;

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

fn contains_2sum(nums: &[u64], target: u64) -> bool {
    let mut seen = HashSet::new();

    for &num in nums {
        if num > target {
            continue;
        }

        let complement = target - num;
        if seen.contains(&complement) {
            return true;
        }

        seen.insert(num);
    }

    return false;
}

#[aoc(day9, part1)]
fn part1(nums: &[u64]) -> u64 {
    nums.windows(WINDOW_SIZE + 1)
        .find(|window| !contains_2sum(&window[..WINDOW_SIZE], window[WINDOW_SIZE]))
        .map(|window| window[WINDOW_SIZE])
        .unwrap()
}

#[aoc(day9, part2)]
fn part2(nums: &[u64]) -> u64 {
    let target = part1(nums);

    let mut low = 0;
    let mut high = 0;
    let mut sum = 0;

    loop {
        if sum == target {
            let (min, max) = nums[low..high].iter().minmax().into_option().unwrap();
            return min + max;
        } else if sum < target {
            sum += nums[high];
            high += 1;
        } else {
            sum -= nums[low];
            low += 1;
        }
    }
}
