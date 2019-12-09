use num_enum::{IntoPrimitive, TryFromPrimitive};
use snafu::{ResultExt as _, Snafu};
use std::convert::{TryFrom, TryInto};
use std::sync::mpsc::{Receiver, SyncSender};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("operation would block"))]
    WouldBlock,
    #[snafu(display("requested input but input was closed"))]
    InputClosed,
    #[snafu(display("tried to output but output was closed"))]
    OutputClosed,
    #[snafu(display("invalid opcode in instruction '{}'", instr))]
    InvalidOpcode {
        instr: isize,
        source: num_enum::TryFromPrimitiveError<Opcode>,
    },
    #[snafu(display("invalid mode in instruction '{}'", instr))]
    InvalidMode {
        instr: isize,
        source: num_enum::TryFromPrimitiveError<Mode>,
    },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Machine {
    pc: usize,
    memory: Vec<isize>,
    input: Receiver<isize>,
    output: SyncSender<isize>,
}

impl Machine {
    pub fn new(memory: Vec<isize>, input: Receiver<isize>, output: SyncSender<isize>) -> Self {
        Machine {
            pc: 0,
            memory,
            input,
            output,
        }
    }
    fn execute(&mut self, instr: Instruction) -> Result<Status> {
        // Create arguments
        let mut args = [
            self.pc as isize + 1,
            self.pc as isize + 2,
            self.pc as isize + 3,
        ];
        args[..instr.opcode.arg_count()]
            .iter_mut()
            .zip(instr.modes.iter())
            .for_each(|(arg, mode)| match mode {
                Mode::Position => *arg = self.memory[*arg as usize],
                Mode::Immidiate => {}
            });

        match instr.opcode {
            Opcode::Add => {
                let [a, b, c] = args;
                self.memory[c as usize] = self.memory[a as usize] + self.memory[b as usize];
                Ok(Status::Advance(instr.opcode.arg_count() + 1))
            }
            Opcode::Mul => {
                let [a, b, c] = args;
                self.memory[c as usize] = self.memory[a as usize] * self.memory[b as usize];
                Ok(Status::Advance(instr.opcode.arg_count() + 1))
            }
            Opcode::Input => {
                use std::sync::mpsc::TryRecvError;
                let [a, _, _] = args;
                self.memory[a as usize] = self.input.try_recv().map_err(|e| match e {
                    TryRecvError::Empty => Error::WouldBlock,
                    TryRecvError::Disconnected => Error::InputClosed,
                })?;
                Ok(Status::Advance(instr.opcode.arg_count() + 1))
            }
            Opcode::Output => {
                use std::sync::mpsc::TrySendError;
                let [a, _, _] = args;
                // Ignore failures when reciving channel is dropped (sending to void is okay)
                self.output
                    .try_send(self.memory[a as usize])
                    .map_err(|e| match e {
                        TrySendError::Full(_) => Error::WouldBlock,
                        TrySendError::Disconnected(_) => Error::OutputClosed,
                    })?;
                Ok(Status::Advance(instr.opcode.arg_count() + 1))
            }
            Opcode::JumpNotZero => {
                let [a, b, _] = args;
                if self.memory[a as usize] != 0 {
                    Ok(Status::Jump(self.memory[b as usize] as usize))
                } else {
                    Ok(Status::Advance(instr.opcode.arg_count() + 1))
                }
            }
            Opcode::JumpZero => {
                let [a, b, _] = args;
                if self.memory[a as usize] == 0 {
                    Ok(Status::Jump(self.memory[b as usize] as usize))
                } else {
                    Ok(Status::Advance(instr.opcode.arg_count() + 1))
                }
            }
            Opcode::LessThan => {
                let [a, b, c] = args;
                if self.memory[a as usize] < self.memory[b as usize] {
                    self.memory[c as usize] = 1;
                } else {
                    self.memory[c as usize] = 0;
                }
                Ok(Status::Advance(instr.opcode.arg_count() + 1))
            }
            Opcode::Equal => {
                let [a, b, c] = args;
                if self.memory[a as usize] == self.memory[b as usize] {
                    self.memory[c as usize] = 1;
                } else {
                    self.memory[c as usize] = 0;
                }
                Ok(Status::Advance(instr.opcode.arg_count() + 1))
            }
            Opcode::Halt => Ok(Status::Halt),
        }
    }

    pub fn run(&mut self) -> Result<isize> {
        loop {
            let instr = Instruction::try_from(self.memory[self.pc])?;
            match self.execute(instr)? {
                Status::Advance(incr) => {
                    self.pc += incr;
                }
                Status::Jump(new_pc) => {
                    self.pc = new_pc;
                }
                Status::Halt => break Ok(self.memory[0]),
            }
        }
    }
}

#[derive(Debug)]
enum Status {
    Advance(usize),
    Jump(usize),
    Halt,
}

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpNotZero = 5,
    JumpZero = 6,
    LessThan = 7,
    Equal = 8,
    OffsetRBase = 9,
    Halt = 99,
}

impl Opcode {
    fn arg_count(&self) -> usize {
        match self {
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::Input => 1,
            Opcode::Output => 1,
            Opcode::JumpNotZero => 2,
            Opcode::JumpZero => 2,
            Opcode::LessThan => 3,
            Opcode::Equal => 3,
            Opcode::OffsetRBase => 1,
            Opcode::Halt => 0,
        }
    }
}

#[derive(Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Mode {
    Position = 0,
    Immidiate = 1,
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    modes: [Mode; 3],
}

impl TryFrom<isize> for Instruction {
    type Error = Error;

    fn try_from(instr: isize) -> Result<Self, Self::Error> {
        Ok(Instruction {
            opcode: ((instr % 100) as u8)
                .try_into()
                .context(InvalidOpcode { instr })?,
            modes: [
                (((instr / 100) % 10) as u8)
                    .try_into()
                    .context(InvalidMode { instr })?,
                (((instr / 1000) % 10) as u8)
                    .try_into()
                    .context(InvalidMode { instr })?,
                (((instr / 10000) % 10) as u8)
                    .try_into()
                    .context(InvalidMode { instr })?,
            ],
        })
    }
}
