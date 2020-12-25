use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day25)]
fn parse(input: &str) -> (u64, u64) {
   input.lines().map(|line| line.parse().unwrap()).next_tuple().unwrap() 
}

const SUBJECT_NUMBER: u64 = 7;
const MODULO: u64 = 20201227;

#[aoc(day25, part1)]
fn part1(&(pub1, pub2): &(u64, u64)) -> u64 {
    let mut value = 1;

    for loop_size in 0.. {
        if value == pub1 {
            value = 1;

            for _ in 0..loop_size {
                value = (value * pub2) % MODULO;
            }

            return value;
        }

        value = (value * SUBJECT_NUMBER) % MODULO;
    }

    unreachable!();
}