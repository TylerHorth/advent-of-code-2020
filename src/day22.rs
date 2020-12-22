use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use aoc_runner_derive::{aoc, aoc_generator};
use fnv::{FnvHasher, FnvHashSet};
use itertools::Itertools;

#[aoc_generator(day22)]
fn parse(input: &str) -> (VecDeque<u8>, VecDeque<u8>) {
    let (p1, p2) = input.split("\n\n").next_tuple().unwrap();

    let p1 = p1.lines().skip(1).map(|l| l.parse().unwrap()).collect();
    let p2 = p2.lines().skip(1).map(|l| l.parse().unwrap()).collect();

    (p1, p2)
}

fn score(deck: VecDeque<u8>) -> u64 {
    deck.into_iter()
        .rev()
        .zip(1..)
        .map(|(c, i)| (c as u64) * i)
        .sum()
}

#[aoc(day22, part1)]
fn part1(decks: &(VecDeque<u8>, VecDeque<u8>)) -> u64 {
    let (mut p1, mut p2) = decks.clone();

    while !p1.is_empty() && !p2.is_empty() {
        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    let winner = if p1.is_empty() { p2 } else { p1 };

    score(winner)
}

enum Winner {
    P1,
    P2,
}

fn recursive_combat(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> Winner {
    let mut seen = FnvHashSet::default();

    while !p1.is_empty() && !p2.is_empty() {
        let mut hasher = FnvHasher::default();
        p1.hash(&mut hasher);
        p2.hash(&mut hasher);

        // Collisions are extremely unlikely
        if !seen.insert(hasher.finish()) {
            return Winner::P1;
        }

        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());

        if p1.len() >= c1 as usize && p2.len() >= c2 as usize {
            // Hypothesis: the deck with the largest card wins, or there is a cycle.
            // Base case: If not enough cards remain to do a recursive step, then the
            //      largest card can never be stolen. Thus, either the player with the
            //      largest card wins, or there is a cycle.
            // Inductive step: If the largest card remains in the deck for the recursive
            //      step, then by the inductive hypothesis either the player with said
            //      card wins, or there is a cycle. If the largest card is drawn, since
            //      all numbers are positive and there are no duplicates, it is impossible
            //      for enough cards to remain in the deck to do a recursive step. Thus,
            //      we are in the base case.
            //
            // Since a cycle results in a win for player 1, if player 1 has the largest
            // card, he will win the subgame.

            let p1_iter = p1.iter().copied().take(c1 as usize);
            let p2_iter = p2.iter().copied().take(c2 as usize);

            if p1_iter.clone().max() > p2_iter.clone().max() {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                match recursive_combat(&mut p1_iter.collect(), &mut p2_iter.collect()) {
                    Winner::P1 => {
                        p1.push_back(c1);
                        p1.push_back(c2);
                    }
                    Winner::P2 => {
                        p2.push_back(c2);
                        p2.push_back(c1);
                    }
                }
            }
        } else {
            if c1 > c2 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        }
    }

    if p1.is_empty() {
        Winner::P2
    } else {
        Winner::P1
    }
}

#[aoc(day22, part2)]
fn part2(decks: &(VecDeque<u8>, VecDeque<u8>)) -> u64 {
    let (mut p1, mut p2) = decks.clone();

    match recursive_combat(&mut p1, &mut p2) {
        Winner::P1 => score(p1),
        Winner::P2 => score(p2),
    }
}
