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
    Halt,
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

impl Opcode {
    fn arg_count(&self) -> usize {
        match self {
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::Input => 1,
            Opcode::Output => 1,
            Opcode::IsNotZero => 2,
            Opcode::IsZero => 2,
            Opcode::LessThan => 3,
            Opcode::Equal => 3,
            Opcode::Halt => 1,
        }
    }
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

        // Update needed argument modes
        args[..instr.opcode.arg_count()]
            .iter_mut()
            .zip(instr.modes.iter())
            .for_each(|(arg, mode)| match mode {
                Mode::Position => *arg = self.memory[*arg as usize],
                Mode::Immidiate => {}
            });

        // Update program counter
        self.pc += instr.opcode.arg_count() + 1;

        match instr.opcode {
            Opcode::Add => {
                let [a, b, c] = args;
                self.memory[c as usize] = self.memory[a as usize] + self.memory[b as usize];
                Ok(Status::Continue)
            }
            Opcode::Mul => {
                let [a, b, c] = args;
                self.memory[c as usize] = self.memory[a as usize] * self.memory[b as usize];
                Ok(Status::Continue)
            }
            Opcode::Input => {
                let [a, _, _] = args;
                self.memory[a as usize] = {
                    let mut s = String::new();
                    std::io::stdin().read_line(&mut s)?;
                    s.trim().parse()?
                };
                Ok(Status::Continue)
            }
            Opcode::Output => {
                let [a, _, _] = args;
                println!("{}", self.memory[a as usize]);
                std::io::stdout().flush()?;
                Ok(Status::Continue)
            }
            Opcode::IsNotZero => {
                let [a, b, _] = args;
                if self.memory[a as usize] != 0 {
                    self.pc = self.memory[b as usize] as usize;
                }
                Ok(Status::Continue)
            }
            Opcode::IsZero => {
                let [a, b, _] = args;
                if self.memory[a as usize] == 0 {
                    self.pc = self.memory[b as usize] as usize;
                }
                Ok(Status::Continue)
            }
            Opcode::LessThan => {
                let [a, b, c] = args;
                if self.memory[a as usize] < self.memory[b as usize] {
                    self.memory[c as usize] = 1;
                } else {
                    self.memory[c as usize] = 0;
                }
                Ok(Status::Continue)
            }
            Opcode::Equal => {
                let [a, b, c] = args;
                if self.memory[a as usize] == self.memory[b as usize] {
                    self.memory[c as usize] = 1;
                } else {
                    self.memory[c as usize] = 0;
                }
                Ok(Status::Continue)
            }
            Opcode::Halt => Ok(Status::Halt),
        }
    }

    fn run(&mut self) -> Result<isize> {
        loop {
            match self.step()? {
                Status::Continue => {}
                Status::Halt => break Ok(self.memory[0]),
            }
        }
    }
}
