use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

type Rules = HashMap<String, HashMap<String, usize>>;

const MY_BAG: &str = "shiny gold";

lazy_static! {
    static ref PARENT_REGEX: Regex = Regex::new(r"^(.*?) bags contain").unwrap();
    static ref CHILDREN_REGEX: Regex = Regex::new(r"(\d+) (.*?) bag").unwrap();
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Rules {
    input
        .lines()
        .map(|line| {
            let parent = PARENT_REGEX.captures(line).unwrap()[1].to_string();
            let children = CHILDREN_REGEX
                .captures_iter(line)
                .map(|c| (c[2].to_string(), c[1].parse().unwrap()))
                .collect();

            (parent, children)
        })
        .collect()
}

fn may_contain<'a>(rules: &'a Rules, bag: &str, seen: &mut HashSet<&'a str>) {
    for (parent, children) in rules {
        if children.contains_key(bag) && seen.insert(parent) {
            may_contain(rules, parent, seen)
        }
    }
}

#[aoc(day7, part1)]
fn part1(rules: &Rules) -> usize {
    let mut seen = HashSet::new();

    may_contain(rules, MY_BAG, &mut seen);

    seen.len()
}

fn sum_bags<'a>(rules: &Rules, bag: &str) -> usize {
    rules[bag]
        .iter()
        .map(|(color, num)| num * sum_bags(rules, color))
        .sum::<usize>()
        + 1
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<String, HashMap<String, usize>>) -> usize {
    sum_bags(input, MY_BAG) - 1
}
