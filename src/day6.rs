use std::ops::{BitAnd, BitOr};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|l| l.chars().fold(0, |acc, c| acc | char_value(c)))
                .collect()
        })
        .collect()
}

fn char_value(c: char) -> u32 {
    1 << (c as usize - 'a' as usize)
}

fn sum_by_rule(groups: &Vec<Vec<u32>>, rule: fn(u32, u32) -> u32) -> u32 {
    groups
        .iter()
        .map(|group| group.iter().copied().fold1(rule).unwrap().count_ones())
        .sum()
}

#[aoc(day6, part1)]
fn part1(groups: &Vec<Vec<u32>>) -> u32 {
    sum_by_rule(groups, BitOr::bitor)
}

#[aoc(day6, part2)]
fn part2(groups: &Vec<Vec<u32>>) -> u32 {
    sum_by_rule(groups, BitAnd::bitand)
}
