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

fn xor_up_to(n: u16) -> u16 {
    match n & 0b11 {
        0b00 => n,
        0b01 => 1,
        0b10 => n | 1,
        0b11 => 0,
        _ => unreachable!(),
    }
}

fn xor_between(low: u16, high: u16) -> u16 {
    xor_up_to(low - 1) ^ xor_up_to(high)
}

#[aoc(day5, part1)]
fn part1(seats: &[u16]) -> u16 {
    seats.iter().copied().max().unwrap()
}

#[aoc(day5, part2)]
fn part2(seats: &[u16]) -> u16 {
    let (min, max, xor) = seats.iter().fold((u16::MAX, 0, 0), |(min, max, xor), &id| {
        (min.min(id), max.max(id), xor ^ id)
    });

    xor_between(min, max) ^ xor
}
