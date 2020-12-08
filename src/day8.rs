use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr, ParseError};

#[derive(Display, FromStr, Debug, Copy, Clone)]
#[display("{} {0}", style = "lowercase")]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

enum ExitCode {
    Terminated,
    InfiniteLoop,
}

#[derive(Default)]
struct Computer<'a> {
    program: &'a [Instruction],
    seen: HashSet<usize>,
    pc: usize,
    acc: i32
}

impl<'a> Computer<'a> {
    fn new(program: &'a [Instruction]) -> Self {
        Self {
            program,
            ..Default::default()
        }
    }

    fn run(&mut self) -> ExitCode {
        loop {
            if let Err(code) = self.step() {
                return code;
            }
        }
    }

    fn step(&mut self) -> Result<(), ExitCode> {
        self.execute(self.program[self.pc])
    }

    fn execute(&mut self, instruction: Instruction) -> Result<(), ExitCode> {
        if !self.seen.insert(self.pc) {
            return Err(ExitCode::InfiniteLoop);
        } 

        match instruction {
            Instruction::Acc(i) => {
                self.acc += i;
                self.pc += 1;
            },
            Instruction::Jmp(i) => {
                self.pc = (self.pc as i32 + i) as usize;
            },
            Instruction::Nop(_) => {
                self.pc += 1;
            }
        }

        if self.pc == self.program.len() {
            return Err(ExitCode::Terminated);
        } 

        Ok(())
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input
        .lines()
        .map(str::parse)
        .collect()
}

#[aoc(day8, part1)]
fn part1(program: &[Instruction]) -> i32 {
    let mut computer = Computer::new(program);

    computer.run();

    computer.acc
}

#[aoc(day8, part2)]
fn part2(program: &[Instruction]) -> i32 {
    for (i, &instruction) in program.iter().enumerate() {
        let flipped = match instruction {
            Instruction::Jmp(n) => Instruction::Nop(n),
            Instruction::Nop(n) => Instruction::Jmp(n),
            _ => continue
        };

        let mut computer = Computer::new(program);

        loop {
            let result = if computer.pc == i {
                computer.execute(flipped)
            } else {
                computer.step()
            };

            match result {
                Err(ExitCode::Terminated) => return computer.acc,
                Err(ExitCode::InfiniteLoop) => break,
                Ok(()) => (),
            }
        }
    }

    panic!("no solution");
}