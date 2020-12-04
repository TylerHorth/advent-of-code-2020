use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use parse_display::{Display, FromStr};
use regex::Regex;

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

lazy_static! {
    static ref HAIR_COLOR_REGEX: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new("^[0-9]{9}$").unwrap();
}

#[derive(Display, FromStr)]
enum Height {
    #[display("{0}cm")]
    Cm(u32),
    #[display("{0}in")]
    In(u32),
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .map(|entry| entry.split(":").map(str::to_string).next_tuple().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(passports: &Vec<HashMap<String, String>>) -> usize {
    passports
        .iter()
        .filter(|passport| FIELDS.iter().all(|&f| passport.contains_key(f)))
        .count()
}

#[aoc(day4, part2)]
fn part2(passports: &Vec<HashMap<String, String>>) -> usize {
    passports
        .iter()
        .filter(|passport| {
            FIELDS.iter().all(|&f| {
                passport
                    .get(f)
                    .map(|v| match f {
                        "byr" => num_in_range(v, 1920, 2002),
                        "iyr" => num_in_range(v, 2010, 2020),
                        "eyr" => num_in_range(v, 2020, 2030),
                        "hgt" => match v.parse::<Height>() {
                            Ok(Height::Cm(h)) => h >= 150 && h <= 193,
                            Ok(Height::In(h)) => h >= 59 && h <= 76,
                            Err(_) => false,
                        },
                        "hcl" => HAIR_COLOR_REGEX.is_match(v),
                        "ecl" => EYE_COLORS.contains(&v.as_str()),
                        "pid" => PID_REGEX.is_match(v),
                        _ => unreachable!(),
                    })
                    .unwrap_or(false)
            })
        })
        .count()
}

fn num_in_range(value: &String, low: usize, high: usize) -> bool {
    value.len() == 4
        && value
            .parse::<usize>()
            .map(|num| num >= low && num <= high)
            .unwrap_or(false)
}
