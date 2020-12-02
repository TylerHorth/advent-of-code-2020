use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt;

struct Line {
    start: usize,
    end: usize,
    char: char,
    password: String,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let (start, end, char, password) =
                scan_fmt!(l, "{}-{} {}: {}", usize, usize, char, String).unwrap();
            Line {
                start,
                end,
                char,
                password,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(lines: &[Line]) -> usize {
    lines
        .iter()
        .filter(|line| {
            let count = line.password.chars().filter(|&c| c == line.char).count();
            (line.start..=line.end).contains(&count)
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(lines: &[Line]) -> usize {
    lines
        .iter()
        .filter(|line| {
            char_at_equals(&line.password, line.start, line.char)
                ^ char_at_equals(&line.password, line.end, line.char)
        })
        .count()
}

fn char_at_equals(string: &str, position: usize, char: char) -> bool {
    string
        .chars()
        .nth(position - 1)
        .map(|c| c == char)
        .unwrap_or(false)
}
