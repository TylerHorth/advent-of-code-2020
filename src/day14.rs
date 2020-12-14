use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr, ParseError};

#[derive(Display, FromStr)]
enum Instruction {
    #[display("mask = {0}")]
    Mask(String),
    #[display("mem[{0}] = {1}")]
    Assignment(u64, u64),
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input.lines().map(str::parse).collect()
}

struct Computer {
    mask: (u64, u64, u64),
    memory: HashMap<u64, u64>,
    set_method: fn(&mut Self, u64, u64),
}

impl Computer {
    fn new(set_method: fn(&mut Self, u64, u64)) -> Self {
        Self {
            mask: (0, 0, 0),
            memory: HashMap::new(),
            set_method,
        }
    }

    fn run(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::Mask(mask) => self.set_mask(mask),
                Instruction::Assignment(address, value) => {
                    (self.set_method)(self, *address, *value)
                }
            }
        }
    }

    fn set_fixed(&mut self, address: u64, value: u64) {
        self.memory
            .insert(address, (value | self.mask.0) & !self.mask.1);
    }

    const MAX_INDEX: u64 = (1 << 36);

    fn set_floating_acc(&mut self, address: u64, value: u64, index: u64) {
        if index == Self::MAX_INDEX {
            self.memory.insert(address, value);
        } else {
            self.set_floating_acc(address | (self.mask.0 & index), value, index << 1);
            if self.mask.2 & index == index {
                self.set_floating_acc(address ^ index, value, index << 1);
            }
        }
    }

    fn set_floating(&mut self, address: u64, value: u64) {
        self.set_floating_acc(address, value, 1);
    }

    fn set_mask(&mut self, mask: &str) {
        let (mut ones, mut zeros, mut exes) = (0, 0, 0);

        for c in mask.chars() {
            ones <<= 1;
            zeros <<= 1;
            exes <<= 1;

            match c {
                '0' => zeros |= 1,
                '1' => ones |= 1,
                'X' => exes |= 1,
                _ => unreachable!(),
            };
        }

        self.mask = (ones, zeros, exes);
    }

    fn total(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[aoc(day14, part1)]
fn part1(program: &[Instruction]) -> u64 {
    let mut computer = Computer::new(Computer::set_fixed);

    computer.run(program);

    computer.total()
}

#[aoc(day14, part2)]
fn part2(program: &[Instruction]) -> u64 {
    let mut computer = Computer::new(Computer::set_floating);

    computer.run(program);

    computer.total()
}
