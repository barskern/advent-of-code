use aoc_runner_derive::*;
use std::error::Error;

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

struct Machine {
    pc: usize,
    memory: Vec<usize>,
}

enum Status {
    Continue,
    Done,
}

impl Machine {
    fn step(&mut self) -> Result<Status> {
        match self.memory[self.pc] {
            1 => {
                let (a, b, c) = (
                    self.memory[self.pc + 1],
                    self.memory[self.pc + 2],
                    self.memory[self.pc + 3],
                );
                self.memory[c] = self.memory[a] + self.memory[b];
                self.pc += 4;
                Ok(Status::Continue)
            }
            2 => {
                let (a, b, c) = (
                    self.memory[self.pc + 1],
                    self.memory[self.pc + 2],
                    self.memory[self.pc + 3],
                );
                self.memory[c] = self.memory[a] * self.memory[b];
                self.pc += 4;
                Ok(Status::Continue)
            }
            99 => Ok(Status::Done),
            op => Err(format!("invalid opcode at {}: {}", self.pc, op).into()),
        }
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> Result<usize> {
    let mut memory: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    // Setup memory crash state
    memory[1] = 12;
    memory[2] = 2;

    // Initialize machine with memory and program counter
    let mut machine = Machine { pc: 0, memory };

    loop {
        match machine.step()? {
            Status::Continue => {}
            Status::Done => break Ok(machine.memory[0]),
        }
    }
}
