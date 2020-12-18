use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
fn parse(input: &str) -> HashSet<(i32, i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(move |(j, c)| if c == '#' { Some((i as i32, j as i32, 0)) } else { None })
            }
        )
        .collect()
}

fn sim(active: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let mut update = active.clone();

    let (mut min_x, mut min_y, mut min_z) = (0, 0, 0);
    let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);

    for &(x, y, z) in active.iter() {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        min_z = min_z.min(z);

        max_x = max_x.max(x);
        max_y = max_y.max(y);
        max_z = max_z.max(z);

        let mut num_active = 0;

        for i in (x - 1)..=(x + 1) {
            for j in (y - 1)..=(y + 1) {
                for k in (z - 1)..=(z + 1) {
                    if i == x && j == y && k == z {
                        continue
                    }
                    if active.contains(&(i, j, k)) {
                        num_active += 1;
                    }
                }
            }
        }

        if num_active != 2 && num_active != 3 {
            update.remove(&(x, y, z));
        }
    }

    for x in (min_x-1)..=(max_x+1) {
        for y in (min_y-1)..=(max_y+1) {
            for z in (min_z-1)..=(max_z+1) {
                if active.contains(&(x, y, z)) {
                    continue
                }

                let mut num_active = 0;

                for i in (x - 1)..=(x + 1) {
                    for j in (y - 1)..=(y + 1) {
                        for k in (z - 1)..=(z + 1) {
                            if i == x && j == y && k == z {
                                continue
                            }
                            if active.contains(&(i, j, k)) {
                                num_active += 1;
                            }
                        }
                    }
                }

                if num_active == 3 {
                    update.insert((x, y, z));
                }
            }
        }
    }

    update
}

fn sim4(active: &HashSet<(i32, i32, i32, i32)>) -> HashSet<(i32, i32, i32, i32)> {
    let mut update = active.clone();

    let (mut min_x, mut min_y, mut min_z, mut min_q) = (0, 0, 0, 0);
    let (mut max_x, mut max_y, mut max_z, mut max_q) = (0, 0, 0, 0);

    for &(x, y, z, q) in active.iter() {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        min_z = min_z.min(z);
        min_q = min_q.min(q);

        max_x = max_x.max(x);
        max_y = max_y.max(y);
        max_z = max_z.max(z);
        max_q = max_q.max(q);

        let mut num_active = 0;

        for i in (x - 1)..=(x + 1) {
            for j in (y - 1)..=(y + 1) {
                for k in (z - 1)..=(z + 1) {
                    for l in (q - 1)..=(q + 1) {
                        if i == x && j == y && k == z && l == q {
                            continue
                        }
                        if active.contains(&(i, j, k, l)) {
                            num_active += 1;
                        }
                    }
                }
            }
        }

        if num_active != 2 && num_active != 3 {
            update.remove(&(x, y, z, q));
        }
    }

    for x in (min_x-1)..=(max_x+1) {
        for y in (min_y-1)..=(max_y+1) {
            for z in (min_z-1)..=(max_z+1) {
                for q in (min_q-1)..=(max_q+1) {
                    if active.contains(&(x, y, z, q)) {
                        continue
                    }

                    let mut num_active = 0;

                    for i in (x - 1)..=(x + 1) {
                        for j in (y - 1)..=(y + 1) {
                            for k in (z - 1)..=(z + 1) {
                                for l in (q - 1)..=(q + 1) {
                                    if i == x && j == y && k == z && l == q {
                                        continue
                                    }
                                    if active.contains(&(i, j, k, l)) {
                                        num_active += 1;
                                    }
                                }
                            }
                        }
                    }

                    if num_active == 3 {
                        update.insert((x, y, z, q));
                    }
                }
            }
        }
    }

    update
}

#[aoc(day17, part1)]
fn part1(active: &HashSet<(i32, i32, i32)>) -> usize {
    let mut active = sim(active);
    for _ in 1..6 {
        active = sim(&active);
    }

    active.len()
}

#[aoc(day17, part2)]
fn part2(active: &HashSet<(i32, i32, i32)>) -> usize {
    let mut active = active.iter().copied().map(|(x, y, z)| (x, y, z, 0)).collect();
    
    for _ in 0..6 {
        active = sim4(&active);
    }

    active.len()
}
