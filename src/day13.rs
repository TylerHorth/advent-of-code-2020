use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day13)]
fn parse(input: &str) -> (u64, Vec<Option<u64>>) {
    let (arrival_time, busses) = input.lines().next_tuple().unwrap();

    let arrival_time = arrival_time.parse().unwrap();
    let busses = busses.split(',').map(|bus| bus.parse().ok()).collect();

    (arrival_time, busses)
}

#[aoc(day13, part1)]
fn part1((arrival_time, busses): &(u64, Vec<Option<u64>>)) -> u64 {
    let (departure_time, bus_id) = busses
        .iter()
        .flatten()
        .map(|&id| ((arrival_time + id - 1) / id * id, id))
        .min_by_key(|&(time, _)| time)
        .unwrap();

    bus_id * (departure_time - arrival_time)
}

fn mod_inv(a: i64, n: i64) -> Option<i64> {
    let mut r = (n, a);
    let mut t = (0, 1);

    while r.1 != 0 {
        let quot = r.0 / r.1;
        r = (r.1, r.0 - quot * r.1);
        t = (t.1, t.0 - quot * t.1);
    }

    if r.0 > 1 {
        None
    } else if t.0 < 0 {
        Some(t.0 + n)
    } else {
        Some(t.0)
    }
}

fn crt(mods: &[i64], remainders: &[i64]) -> Option<i64> {
    assert_eq!(mods.len(), remainders.len());

    let prod: i64 = mods.iter().product();
    let mut sum = 0;

    for (&m, &r) in mods.iter().zip(remainders) {
        let y = prod / m;
        sum += r * y * mod_inv(y, m)?;
    }

    Some(sum.rem_euclid(prod))
}

#[aoc(day13, part2)]
fn part2((_, busses): &(u64, Vec<Option<u64>>)) -> i64 {
    let mut mods = Vec::new();
    let mut remainders = Vec::new();

    for (i, bus) in busses.iter().enumerate() {
        if let &Some(id) = bus {
            mods.push(id as i64);
            remainders.push(-(i as i64));
        }
    }

    crt(&mods, &remainders).expect("no solution")
}