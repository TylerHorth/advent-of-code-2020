use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|l| l.chars().fold(0, |id, c| id << 1 | char_value(c)))
        .collect()
}

fn char_value(c: char) -> u16 {
    match c {
        'F' | 'L' => 0,
        'B' | 'R' => 1,
        _ => unreachable!(),
    }
}

#[aoc(day5, part1)]
fn part1(seats: &[u16]) -> u16 {
    seats.iter().copied().max().unwrap()
}

#[aoc(day5, part2)]
fn part2(seats: &[u16]) -> u16 {
    let seats: HashSet<u16> = seats.iter().copied().collect();

    (0..=0b1111111111)
        .skip_while(|id| !seats.contains(id))
        .find(|id| !seats.contains(id))
        .unwrap()
}
