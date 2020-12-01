use aoc_runner_derive::*;
use std::error::Error;

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> Result<usize> {
    let mut memory: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    // Setup memory crash state
    memory[1] = 12;
    memory[2] = 2;

    // Initialize machine with memory and program counter
    let mut machine = Machine { pc: 0, memory };

    machine.run()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> Result<usize> {
    let initial_memory: Vec<usize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    (0..100)
        .flat_map(|noun| (0..100).map(move |verb| (noun, verb)))
        .filter_map(|(noun, verb)| {
            let mut memory = initial_memory.clone();

            // Setup memory crash state
            memory[1] = noun;
            memory[2] = verb;

            // Initialize machine with memory and program counter
            let mut machine = Machine { pc: 0, memory };

            Some((noun, verb, machine.run().ok()?))
        })
        .find(|&(_, _, v)| v == 19_690_720)
        .ok_or_else(|| "did not find a solution".into())
        .map(|(noun, verb, _)| (noun * 100) + verb)
}

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

    fn run(&mut self) -> Result<usize> {
        loop {
            match self.step()? {
                Status::Continue => {}
                Status::Done => break Ok(self.memory[0]),
            }
        }
    }
}
