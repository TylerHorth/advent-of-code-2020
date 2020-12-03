use aoc_runner_derive::{aoc, aoc_generator};

const SLOPE_P1: (usize, usize) = (3, 1);
const SLOPES_P2: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

struct Map {
    trees: Vec<Vec<bool>>,
}

impl Map {
    fn is_tree(&self, x: usize, y: usize) -> bool {
        let row = &self.trees[y];
        row[x % row.len()]
    }

    fn height(&self) -> usize {
        self.trees.len()
    }

    fn trees_hit(&self, (mx, my): (usize, usize)) -> usize {
        std::iter::successors(Some((0, 0)), |(x, y)| Some((x + mx, y + my)))
            .take_while(|&(_, y)| y < self.height())
            .filter(|&(x, y)| self.is_tree(x, y))
            .count()
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Map {
    let trees = input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    Map { trees }
}

#[aoc(day3, part1)]
fn part1(map: &Map) -> usize {
    map.trees_hit(SLOPE_P1)
}

#[aoc(day3, part2)]
fn part2(map: &Map) -> usize {
    SLOPES_P2.iter().map(|&s| map.trees_hit(s)).product()
}
