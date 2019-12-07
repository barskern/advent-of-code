use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::{TryFrom, TryInto};
use std::io::Write;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
struct Machine {
    pc: usize,
    memory: Vec<isize>,
}

#[derive(Debug)]
enum Status {
    Continue,
    Done,
}

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum Opcode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    IsNotZero = 5,
    IsZero = 6,
    LessThan = 7,
    Equal = 8,
    Halt = 99,
}

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum Mode {
    Position = 0,
    Immidiate = 1,
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    modes: [Mode; 3],
}

impl TryFrom<isize> for Instruction {
    type Error = Error;

    fn try_from(n: isize) -> Result<Self, Self::Error> {
        Ok(Instruction {
            opcode: ((n % 100) as u8).try_into()?,
            modes: [
                (((n / 100) % 10) as u8).try_into()?,
                (((n / 1000) % 10) as u8).try_into()?,
                (((n / 10000) % 10) as u8).try_into()?,
            ],
        })
    }
}

impl Machine {
    fn step(&mut self) -> Result<Status> {
        let instr = Instruction::try_from(self.memory[self.pc])?;
        let mut args = [
            self.pc as isize + 1,
            self.pc as isize + 2,
            self.pc as isize + 3,
        ];

        let mut set_arg_modes = |n: usize| {
            args[..n]
                .iter_mut()
                .zip(instr.modes.iter())
                .for_each(|(arg, mode)| match mode {
                    Mode::Position => *arg = self.memory[*arg as usize],
                    Mode::Immidiate => {}
                    _ => unreachable!(),
                });
        };

        match instr.opcode {
            Opcode::Add => {
                set_arg_modes(3);
                let [a, b, c] = args;
                self.memory[c as usize] = self.memory[a as usize] + self.memory[b as usize];
                self.pc += 4;
                Ok(Status::Continue)
            }
            Opcode::Mul => {
                set_arg_modes(3);
                let [a, b, c] = args;
                self.memory[c as usize] = self.memory[a as usize] * self.memory[b as usize];
                self.pc += 4;
                Ok(Status::Continue)
            }
            Opcode::Input => {
                set_arg_modes(1);
                let [a, _, _] = args;
                self.memory[a as usize] = {
                    let mut s = String::new();
                    std::io::stdin().read_line(&mut s)?;
                    s.trim().parse()?
                };
                self.pc += 2;
                Ok(Status::Continue)
            }
            Opcode::Output => {
                set_arg_modes(1);
                let [a, _, _] = args;
                println!("{}", self.memory[a as usize]);
                std::io::stdout().flush();
                self.pc += 2;
                Ok(Status::Continue)
            }
            Opcode::IsNotZero => {
                set_arg_modes(2);
                let [a, b, _] = args;
                if self.memory[a as usize] != 0 {
                    self.pc = self.memory[b as usize] as usize;
                } else {
                    self.pc += 3;
                }
                Ok(Status::Continue)
            }
            Opcode::IsZero => {
                set_arg_modes(2);
                let [a, b, _] = args;
                if self.memory[a as usize] == 0 {
                    self.pc = self.memory[b as usize] as usize;
                } else {
                    self.pc += 3;
                }
                Ok(Status::Continue)
            }
            Opcode::LessThan => {
                set_arg_modes(3);
                let [a, b, c] = args;
                if self.memory[a as usize] < self.memory[b as usize] {
                    self.memory[c as usize] = 1;
                } else {
                    self.memory[c as usize] = 0;
                }
                self.pc += 4;
                Ok(Status::Continue)
            }
            Opcode::Equal => {
                set_arg_modes(3);
                let [a, b, c] = args;
                if self.memory[a as usize] == self.memory[b as usize] {
                    self.memory[c as usize] = 1;
                } else {
                    self.memory[c as usize] = 0;
                }
                self.pc += 4;
                Ok(Status::Continue)
            }
            Opcode::Halt => Ok(Status::Done),
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
