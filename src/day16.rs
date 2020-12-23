use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr)]
#[display("{name}: {a}-{b} or {c}-{d}")]
struct Field {
    name: String,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
}

impl Field {
    fn contains(&self, value: usize) -> bool {
        (value >= self.a && value <= self.b) || (value >= self.c && value <= self.d)
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> (Vec<Field>, Vec<usize>, Vec<Vec<usize>>) {
    let (fields, my_ticket, nearby_tickets) = input.split("\n\n").next_tuple().unwrap();

    let fields = fields.lines().map(|l| l.parse().unwrap()).collect();

    let my_ticket = my_ticket
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let nearby_tickets = nearby_tickets
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (fields, my_ticket, nearby_tickets)
}

#[aoc(day16, part1)]
fn part1((fields, _, nearby_tickets): &(Vec<Field>, Vec<usize>, Vec<Vec<usize>>)) -> usize {
    nearby_tickets
        .iter()
        .flatten()
        .filter(|&&value| !fields.iter().any(|field| field.contains(value)))
        .sum()
}

#[aoc(day16, part2)]
fn part2((fields, my_ticket, nearby_tickets): &(Vec<Field>, Vec<usize>, Vec<Vec<usize>>)) -> usize {
    let valid_tickets = nearby_tickets.iter().filter(|ticket| {
        ticket
            .iter()
            .all(|&value| fields.iter().any(|field| field.contains(value)))
    });

    let mut possible_fields: Vec<(usize, HashSet<usize>)> = fields
        .iter()
        .map(|_| (0..fields.len()).collect())
        .enumerate()
        .collect();

    for ticket in valid_tickets {
        for (&value, (_, possible)) in ticket.iter().zip(&mut possible_fields) {
            for (i, field) in fields.iter().enumerate() {
                if !field.contains(value) {
                    possible.remove(&i);
                }
            }
        }
    }

    possible_fields.sort_by_key(|(_, possible_fields)| possible_fields.len());

    let mut used = HashSet::new();
    let mut product = 1;

    for (i, possible) in possible_fields {
        let field_index = (&possible - &used).into_iter().exactly_one().unwrap();

        if fields[field_index].name.starts_with("departure") {
            product *= my_ticket[i];
        }

        used.insert(field_index);
    }

    product
}
