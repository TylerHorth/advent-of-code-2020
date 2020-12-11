use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone)]
enum Tile {
    Floor,
    Occupied,
    Empty,
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Map {
    const DIRECTIONS: [(i64, i64); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn new(tiles: Vec<Vec<Tile>>) -> Map {
        let height = tiles.len();
        let width = tiles[0].len();
        Map {
            tiles,
            width,
            height,
        }
    }

    fn sim(
        &mut self,
        exit_rule: usize,
        see_rule: fn(&Self, (usize, usize), (i64, i64)) -> bool,
    ) -> bool {
        let mut changed = false;
        let mut newmap = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                if matches!(self.tiles[y][x], Tile::Floor) {
                    continue;
                }

                let num_occupied = Self::DIRECTIONS
                    .iter()
                    .filter(|&&dir| see_rule(&self, (x, y), dir))
                    .count();

                match self.tiles[y][x] {
                    Tile::Empty if num_occupied == 0 => {
                        newmap.tiles[y][x] = Tile::Occupied;
                        changed = true;
                    }
                    Tile::Occupied if num_occupied >= exit_rule => {
                        newmap.tiles[y][x] = Tile::Empty;
                        changed = true;
                    }
                    _ => (),
                }
            }
        }

        *self = newmap;

        changed
    }

    fn step_pos(&self, (x, y): (usize, usize), (dx, dy): (i64, i64)) -> Option<(usize, usize)> {
        let (x, y) = (x as i64 + dx, y as i64 + dy);

        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }

    fn adj_occupied(&self, pos: (usize, usize), dir: (i64, i64)) -> bool {
        self.step_pos(pos, dir)
            .map(|(x, y)| matches!(self.tiles[y][x], Tile::Occupied))
            .unwrap_or_default()
    }

    fn see_occupied(&self, pos: (usize, usize), dir: (i64, i64)) -> bool {
        std::iter::successors(Some(pos), |&pos| self.step_pos(pos, dir))
            .map(|(x, y)| self.tiles[y][x])
            .skip(1)
            .find_map(|tile| match tile {
                Tile::Occupied => Some(true),
                Tile::Empty => Some(false),
                Tile::Floor => None,
            })
            .unwrap_or_default()
    }

    fn total_occupied(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|t| matches!(t, Tile::Occupied)).count())
            .sum()
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Map {
    let tiles = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Floor,
                    'L' => Tile::Empty,
                    '#' => Tile::Occupied,
                    _ => unimplemented!(),
                })
                .collect()
        })
        .collect();

    Map::new(tiles)
}

#[aoc(day11, part1)]
fn part1(map: &Map) -> usize {
    let mut map = map.clone();

    while map.sim(4, Map::adj_occupied) {}

    map.total_occupied()
}

#[aoc(day11, part2)]
fn part2(map: &Map) -> usize {
    let mut map = map.clone();

    while map.sim(5, Map::see_occupied) {}

    map.total_occupied()
}
