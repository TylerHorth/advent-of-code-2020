use aoc_runner_derive::{aoc, aoc_generator};

use parse_display::{Display, FromStr, ParseError};

#[derive(Display, FromStr, Copy, Clone)]
enum Action {
    #[display("N{0}")]
    North(i64),
    #[display("S{0}")]
    South(i64),
    #[display("E{0}")]
    East(i64),
    #[display("W{0}")]
    West(i64),
    #[display("L{0}")]
    Left(i64),
    #[display("R{0}")]
    Right(i64),
    #[display("F{0}")]
    Forward(i64),
}

trait Navigate {
    fn shift(&mut self, x: i64, y: i64);
    fn rotate(&mut self, degrees: i64);
    fn forward(&mut self, units: i64);

    fn navigate(&mut self, actions: &[Action]) {
        for &action in actions {
            match action {
                Action::North(value) => self.shift(0, value),
                Action::South(value) => self.shift(0, -value),
                Action::East(value) => self.shift(value, 0),
                Action::West(value) => self.shift(-value, 0),
                Action::Left(value) => self.rotate(360 - value),
                Action::Right(value) => self.rotate(value),
                Action::Forward(value) => self.forward(value),
            }
        }
    }
}

struct SimpleShip {
    position: (i64, i64),
    facing: i64,
}

impl SimpleShip {
    fn new() -> Self {
        Self {
            position: (0, 0),
            facing: 90,
        }
    }
}

impl Navigate for SimpleShip {
    fn shift(&mut self, x: i64, y: i64) {
        self.position.0 += x;
        self.position.1 += y;
    }

    fn rotate(&mut self, degrees: i64) {
        self.facing = (self.facing + degrees) % 360;
    }

    fn forward(&mut self, units: i64) {
        match self.facing {
            0 => self.position.1 += units,
            90 => self.position.0 += units,
            180 => self.position.1 -= units,
            270 => self.position.0 -= units,
            rotation => panic!("invalid rotation: {}", rotation),
        }
    }
}

struct WaypointShip {
    position: (i64, i64),
    waypoint: (i64, i64),
}

impl WaypointShip {
    fn new() -> Self {
        Self {
            position: (0, 0),
            waypoint: (10, 1),
        }
    }
}

impl Navigate for WaypointShip {
    fn shift(&mut self, x: i64, y: i64) {
        self.waypoint.0 += x;
        self.waypoint.1 += y;
    }

    fn rotate(&mut self, degrees: i64) {
        let (x, y) = self.waypoint;

        self.waypoint = match degrees {
            90 => (y, -x),
            180 => (-x, -y),
            270 => (-y, x),
            rotation => panic!("invalid rotation: {}", rotation),
        }
    }

    fn forward(&mut self, units: i64) {
        self.position.0 += units * self.waypoint.0;
        self.position.1 += units * self.waypoint.1;
    }
}

fn manhattan_distance((x, y): (i64, i64)) -> i64 {
    x.abs() + y.abs()
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Result<Vec<Action>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day12, part1)]
fn part1(actions: &[Action]) -> i64 {
    let mut ship = SimpleShip::new();

    ship.navigate(actions);

    manhattan_distance(ship.position)
}

#[aoc(day12, part2)]
fn part2(actions: &[Action]) -> i64 {
    let mut ship = WaypointShip::new();

    ship.navigate(actions);

    manhattan_distance(ship.position)
}
