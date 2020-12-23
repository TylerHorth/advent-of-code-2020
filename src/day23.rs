use std::iter::FromIterator;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<usize> {
   input.lines().next().unwrap().chars().map(|c| (c as usize) - ('0' as usize)).collect()
}

#[aoc(day23, part1)]
fn part1(cups: &Vec<usize>) -> usize {
    let mut crab_game: CrabGame = cups.iter().copied().collect();
    
    crab_game.play(100);

    crab_game.iter_after(1).fold(0, |acc, cup| acc * 10 + cup)
}

#[aoc(day23, part2)]
fn part2(cups: &Vec<usize>) -> usize {
    let mut crab_game: CrabGame = cups.iter().copied().chain(10..=1_000_000).collect();
    
    crab_game.play(10_000_000);

    let c1 = crab_game.cups[1];
    let c2 = crab_game.cups[c1];
    
    c1 * c2
}

struct CrabGame {
    cups: Vec<usize>,
    cur: usize,
    min: usize,
    max: usize,
}

impl FromIterator<usize> for CrabGame {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut cups_iter = iter.into_iter();
        let mut cups = vec![0; cups_iter.size_hint().0];

        let first = cups_iter.next().unwrap();
        let mut last = first;

        let mut min = last;
        let mut max = last;

        for cup in cups_iter {
            if last >= cups.len() {
                cups.extend((last..=cups.len()).map(|_| 0))
            }
            min = min.min(cup);
            max = max.max(cup);
            cups[last] = cup;
            last = cup;
        }

        if last >= cups.len() {
            cups.extend((last..=cups.len()).map(|_| 0))
        }

        cups[last] = first;

        Self {
            cups,
            cur: first,
            min,
            max
        }
    }
}

impl CrabGame {
    fn remove_after(&mut self, after: usize) -> usize {
        let remove = self.cups[after];
        self.cups[after] = self.cups[remove];
        remove
    }

    fn insert_after(&mut self, after: usize, new: usize) {
        self.cups[new] = self.cups[after];
        self.cups[after] = new;
    }

    fn iter_after<'a>(&'a self, after: usize) -> impl Iterator<Item = usize> + 'a {
        std::iter::successors(Some(self.cups[after]), move |&prev| {
            let next = self.cups[prev];
            if next == after {
                None
            } else {
                Some(next)
            }
        })
    }

    fn round(&mut self) {
        let c1 = self.remove_after(self.cur);
        let c2 = self.remove_after(self.cur);
        let c3 = self.remove_after(self.cur);

        let mut dst = self.cur;
        while [self.cur, c1, c2, c3].contains(&dst) {
            if dst == self.min {
                dst = self.max;
            } else {
                dst -= 1;
            }
        }

        self.insert_after(dst, c3);
        self.insert_after(dst, c2);
        self.insert_after(dst, c1);

        self.cur = self.cups[self.cur];
    }

    fn play(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.round();
        }
    }
}