use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day22)]
fn parse(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let (p1, p2) = input.split("\n\n").next_tuple().unwrap();

    let p1 = p1.lines().skip(1).map(|l| l.parse().unwrap()).collect();
    let p2 = p2.lines().skip(1).map(|l| l.parse().unwrap()).collect();

    (p1, p2)
}

fn score(deck: VecDeque<usize>) -> usize {
   deck.into_iter().rev().zip(1..).map(|(c, i)| c * i).sum()
}

#[aoc(day22, part1)]
fn part1(decks: &(VecDeque<usize>, VecDeque<usize>)) -> usize {
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

   let winner = if p1.is_empty() {
       p2
   } else {
       p1
   };

   score(winner)
}

enum Winner {
    P1(VecDeque<usize>),
    P2(VecDeque<usize>),
}

fn recursive_combat(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>) -> Winner {
    let mut seen = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if !seen.insert((p1.clone(), p2.clone())) {
            return Winner::P1(p1);
        }

        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());

        if p1.len() >= c1 && p2.len() >= c2 {
            match recursive_combat(p1.iter().copied().take(c1).collect(), p2.iter().copied().take(c2).collect()) {
                Winner::P1(_) => {
                    p1.push_back(c1);
                    p1.push_back(c2);
                },
                Winner::P2(_) => {
                    p2.push_back(c2);
                    p2.push_back(c1);
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
        Winner::P2(p2)
    } else {
        Winner::P1(p1)
    }
}

#[aoc(day22, part2)]
fn part2(decks: &(VecDeque<usize>, VecDeque<usize>)) -> usize {
   let (p1, p2) = decks.clone();

   let winner = match recursive_combat(p1, p2) {
       Winner::P1(deck) => deck,
       Winner::P2(deck) => deck,
   };

   score(winner)
}