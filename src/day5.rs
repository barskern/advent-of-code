#![allow(warnings)]

use aoc_runner_derive::*;
use std::convert::{TryFrom, TryInto};
use std::io::Write;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> Result<isize> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let mut machine = Machine { pc: 0, memory };

    machine.run()
}

// #[aoc(day5, part2)]
// pub fn part2(input: &str) -> Result<isize> {
//     unimplemented!()
// }

struct Machine {
    pc: usize,
    memory: Vec<isize>,
}

enum Status {
    Continue,
    Done,
}

struct Instruction {
    opcode: u8,
    modes: [u8; 3],
}

impl From<isize> for Instruction {
    fn from(n: isize) -> Self {
        Instruction {
            opcode: (n % 100) as u8,
            modes: [
                ((n / 100) % 10) as u8,
                ((n / 1000) % 10) as u8,
                ((n / 10000) % 10) as u8,
            ],
        }
    }
}

impl Machine {
    fn step(&mut self) -> Result<Status> {
        let instr = Instruction::from(self.memory[self.pc]);
        let mut args = [
            self.pc as isize + 1,
            self.pc as isize + 2,
            self.pc as isize + 3,
        ];

        let mut set_modes = |n: usize| {
            args[..n]
                .iter_mut()
                .zip(instr.modes.iter())
                .for_each(|(arg, mode)| match mode {
                    0 => *arg = self.memory[*arg as usize],
                    1 => {}
                    _ => unreachable!(),
                });
        };

        match instr.opcode {
            1 => {
                set_modes(3);
                let [a, b, c] = args;
                self.memory[c as usize] = self.memory[a as usize] + self.memory[b as usize];
                self.pc += 4;
                Ok(Status::Continue)
            }
            2 => {
                set_modes(3);
                let [a, b, c] = args;
                self.memory[c as usize] = self.memory[a as usize] * self.memory[b as usize];
                self.pc += 4;
                Ok(Status::Continue)
            }
            3 => {
                set_modes(1);
                let [a, _, _] = args;
                self.memory[a as usize] = {
                    let mut s = String::new();
                    std::io::stdin().read_line(&mut s)?;
                    s.trim().parse()?
                };
                self.pc += 2;
                Ok(Status::Continue)
            }
            4 => {
                set_modes(1);
                let [a, _, _] = args;
                println!("{}", self.memory[a as usize]);
                std::io::stdout().flush();
                self.pc += 2;
                Ok(Status::Continue)
            }
            99 => Ok(Status::Done),
            op => Err(format!("invalid opcode at {}: {}", self.pc, op).into()),
        }
    }

    fn run(&mut self) -> Result<isize> {
        loop {
            match self.step()? {
                Status::Continue => {}
                Status::Done => break Ok(self.memory[0]),
            }
        }
    }
}
