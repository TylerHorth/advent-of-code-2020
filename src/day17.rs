use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day17)]
fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| {
                if c == '#' {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

struct Space {
    active: HashSet<Vec<i32>>,
}

impl Space {
    fn new(initial: &Vec<(i32, i32)>, dimensions: usize) -> Self {
        assert!(dimensions >= 2);

        let active = initial
            .iter()
            .map(|&(x, y)| {
                let mut point = vec![0; dimensions];

                point[0] = x;
                point[1] = y;

                point
            })
            .collect();

        Self { active }
    }

    fn neighbors<'a>(&self, point: &'a Vec<i32>) -> impl Iterator<Item = Vec<i32>> + 'a {
        point
            .iter()
            .map(|n| (n - 1)..=(n + 1))
            .multi_cartesian_product()
            .filter(move |neighbor| neighbor != point)
    }

    fn sim(&mut self) {
        let mut num_active = HashMap::new();

        self.active
            .iter()
            .flat_map(|point| self.neighbors(point))
            .for_each(|point| *num_active.entry(point).or_default() += 1);

        self.active
            .retain(|point| (2..=3).contains(num_active.get(point).unwrap_or(&0)));

        self.active
            .extend(num_active.into_iter().filter_map(|(point, active)| {
                if active == 3 {
                    Some(point)
                } else {
                    None
                }
            }));
    }

    fn run(&mut self) -> usize {
        for _ in 0..6 {
            self.sim();
        }

        self.active.len()
    }
}

#[aoc(day17, part1)]
fn part1(active: &Vec<(i32, i32)>) -> usize {
    Space::new(active, 3).run()
}

#[aoc(day17, part2)]
fn part2(active: &Vec<(i32, i32)>) -> usize {
    Space::new(active, 4).run()
}
