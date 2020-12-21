use std::{collections::HashMap, iter::FromIterator};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
fn parse(input: &str) -> HashMap<u16, Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();

            let id = lines.next().unwrap()[5..9].parse().unwrap();
            let tile = lines.map(|l| l.chars().collect()).collect();

            (id, tile)
        })
        .collect()
}

#[derive(Default)]
struct EdgeMap {
    top: HashMap<u16, Vec<Tile>>,
    right: HashMap<u16, Vec<Tile>>,
    bottom: HashMap<u16, Vec<Tile>>,
    left: HashMap<u16, Vec<Tile>>,
}

impl EdgeMap {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, tile: Tile) {
        self.top.entry(tile.top).or_default().push(tile);
        self.right.entry(tile.right).or_default().push(tile);
        self.bottom.entry(tile.bottom).or_default().push(tile);
        self.left.entry(tile.left).or_default().push(tile);
    }

    fn get<'a>(&'a self, tile: Tile, direction: Direction) -> impl Iterator<Item = Tile> + 'a {
        match direction {
            Direction::Top => self.bottom.get(&tile.top),
            Direction::Right => self.left.get(&tile.right),
            Direction::Bottom => self.top.get(&tile.bottom),
            Direction::Left => self.right.get(&tile.left),
        }.into_iter().flatten().copied()
    }
}

impl FromIterator<Tile> for EdgeMap {
    fn from_iter<T: IntoIterator<Item = Tile>>(iter: T) -> Self {
        let mut edge_map = EdgeMap::new();

        for tile in iter {
            edge_map.insert(tile);
        }

        edge_map
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Default)]
struct Tile {
    id: u16,
    top: u16, 
    right: u16,
    bottom: u16,
    left: u16,
    orientation: u8
}

#[derive(Copy, Clone)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left
}

const DIRECTIONS: [Direction; 4] = [Direction::Top, Direction::Right, Direction::Bottom, Direction::Left];

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point(i8, i8);

impl Point {
    fn step(&self, direction: Direction) -> Point {
        match direction {
            Direction::Top => Point(self.0, self.1 - 1),
            Direction::Right => Point(self.0 + 1, self.1),
            Direction::Bottom => Point(self.0, self.1 + 1),
            Direction::Left => Point(self.0 - 1, self.1),
        }
    }
}

impl Tile {
    fn to_int(chars: impl Iterator<Item = char>) -> u16 {
        chars.fold(0, |i, c| {
            if c == '#' {
                (i << 1) | 1
            } else {
                i << 1
            }
        })
    }

    /// Only last 10 bits are reversed
    fn reverse(mut side: u16) -> u16 {
        side = (side & 0b0000001100001111) << 4 | (side & 0b0000000011110000) >> 4;
        side = (side & 0b0011000000110011) << 2 | (side & 0b0000000011001100) >> 2;
        side = (side & 0b0100000001010101) << 1 | (side & 0b1000000010101010) >> 1;
        side = (side & 0b0000000011111111) << 2 | (side & 0b1100000000000000) >> 14;

        side
    }

    fn new(id: u16, data: &Vec<Vec<char>>) -> Self {
        Self { 
            id,
            top:    Self::to_int(data.first().unwrap().iter().copied()),                     // top
            right:  Self::to_int(data.iter().map(|line| *line.last().unwrap())),   // right
            bottom: Self::to_int(data.last().unwrap().iter().copied()),                      // bottom
            left:   Self::to_int(data.iter().map(|line| *line.first().unwrap())),  // left
            orientation: 0,
        }
    }

    fn permutations(self) -> impl Iterator<Item = Tile> {
        std::iter::successors(Some(self), |prev| {
            if prev.orientation == 7 {
                return None;
            }

            let mut next = prev.rotated();

            next.orientation += 1;

            if next.orientation == 4 {
               next = next.flipped(); 
            } 

            Some(next)
        })
    }

    fn rotated(&self) -> Self {
        Self { 
            id: self.id,
            top: Self::reverse(self.left),
            right: self.top,
            bottom: Self::reverse(self.right),
            left: self.bottom,
            orientation: self.orientation,
        }
    }

    fn flipped(&self) -> Self {
        Self { 
            id: self.id,
            top: Self::reverse(self.top),
            right: self.left,
            bottom: Self::reverse(self.bottom),
            left: self.right,
            orientation: self.orientation
        }
    }
}

const LAYOUT_SIZE: usize = 12;
const TILE_SIZE: usize = 8;
const IMAGE_SIZE: usize = LAYOUT_SIZE * TILE_SIZE;

const MONSTER: [&str; 3] = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

#[derive(Default)]
struct Layout([[Tile; LAYOUT_SIZE]; LAYOUT_SIZE]);

impl From<EdgeMap> for Layout {
    fn from(edge_map: EdgeMap) -> Self {
        let mut tile_map = HashMap::new();

        let seed_point = Point(0, 0);
        let seed_tile = *edge_map.top.values().next().unwrap().first().unwrap();

        tile_map.insert(seed_tile.id, (seed_point, seed_tile));

        let mut stack = vec![(seed_point, seed_tile)];
        let mut min_point = seed_point;

        while let Some((start_point, start_tile)) = stack.pop() {
            for (point, tile) in DIRECTIONS
                .iter()
                .map(|&direction| (start_point.step(direction), direction))
                .flat_map(|(point, direction)| edge_map.get(start_tile, direction).map(move |tile| (point, tile))) 
            {
                if !tile_map.contains_key(&tile.id) {
                    tile_map.insert(tile.id, (point, tile));
                    stack.push((point, tile));
                    min_point = min_point.min(point);
                }
            }
        }

        let mut layout = Layout::default();

        for (Point(x, y), tile) in tile_map.values() {
            layout.0[(y - min_point.1) as usize][(x - min_point.0) as usize] = *tile;
        }

        layout
    }
}

struct Image([[char; IMAGE_SIZE]; IMAGE_SIZE]);

impl Image {
    fn rotate(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let height = data.len();
        let width = data[0].len();

        let mut result = vec![vec!['\0'; height]; width];

        for i in 0..width {
            for j in 0..height {
                result[i][height - j - 1] = data[j][i];
            }
        }

        result
    }

    fn flip(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        data
            .iter()
            .map(|line| line.iter().copied().rev().collect())
            .collect()
    }

    fn orient(data: &Vec<Vec<char>>, orientation: u8) -> Vec<Vec<char>> {
        let mut data = data.clone();

        if orientation >= 4 {
            data = Image::flip(&data);
        }

        for _ in 0..(orientation % 4) {
            data = Image::rotate(&data);
        }

        data
    }

    fn new(layout: Layout, tiles: &HashMap<u16, Vec<Vec<char>>>) -> Self {
        let mut image = Image([['\0'; IMAGE_SIZE]; IMAGE_SIZE]);

        for i in 0..LAYOUT_SIZE {
            for j in 0..LAYOUT_SIZE {
                let tile = layout.0[i][j];
                let data = Image::orient(&tiles[&tile.id], tile.orientation);

                for y in 0..TILE_SIZE {
                    for x in 0..TILE_SIZE {
                        image.0[i * TILE_SIZE + y][j * TILE_SIZE + x] = data[y + 1][x + 1];
                    }
                }
            }
        }

        image
    }

    fn count_occurences(&self, key: &Vec<Vec<char>>) -> usize {
        let positions: Vec<(usize, usize)> = key
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line
                    .iter()
                    .enumerate()
                    .filter(|(_, &c)| c == '#')
                    .map(move |(j, _)| (i, j))
            })
            .collect();

        let mut count = 0;

        for i in 0..IMAGE_SIZE {
            for j in 0..IMAGE_SIZE {
                if positions
                    .iter()
                    .all(|&(x, y)| {
                        self.0
                            .get(i + y)
                            .and_then(|line| line.get(j + x))
                            .map(|&c| c == '#')
                            .unwrap_or(false)
                    })
                {
                    count += 1;
                }
            }
        }

        count
    }
}

#[aoc(day20, part1)]
fn part1(tiles: &HashMap<u16, Vec<Vec<char>>>) -> u64 {
    let edge_map: EdgeMap = tiles
        .iter()
        .flat_map(|(id, data)| Tile::new(*id, data).permutations())
        .collect();

    let layout = Layout::from(edge_map);

    let max = LAYOUT_SIZE - 1;
    let corners = [(0, 0), (0, max), (max, 0), (max, max)];

    corners
        .iter()
        .map(|&(i, j)| layout.0[i][j].id as u64)
        .product()
}

#[aoc(day20, part2)]
fn part2(tiles: &HashMap<u16, Vec<Vec<char>>>) -> usize {
    let edge_map: EdgeMap = tiles
        .iter()
        .flat_map(|(id, data)| Tile::new(*id, data).permutations())
        .collect();

    let layout = Layout::from(edge_map);
    let image = Image::new(layout, tiles);

    let monster = MONSTER.iter().map(|l| l.chars().collect()).collect();
    let monster_count = (0..8)
        .map(|i| Image::orient(&monster, i))
        .map(|key| image.count_occurences(&key))
        .find(|&count| count > 0)
        .unwrap();

    let monster_chars = MONSTER.iter().flat_map(|l| l.chars()).filter(|&c| c == '#').count();
    let image_chars = image.0.iter().flatten().filter(|&&c| c == '#').count();

    image_chars - monster_count * monster_chars
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bit_reverse() {
        assert_eq!(0b1111100000, Tile::reverse(0b0000011111));
        assert_eq!(0b1010101010, Tile::reverse(0b0101010101));
        assert_eq!(0b0110100101, Tile::reverse(0b1010010110));
    }

    #[test]
    fn test_image_rotate() {
        let before = vec![vec!['1', '2', '3'], vec!['4', '5', '6']];
        let after = Image::rotate(&before);
        let expected = vec![vec!['4', '1'], vec!['5', '2'], vec!['6', '3']];

        assert_eq!(after, expected);
    }
}