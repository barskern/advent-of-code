use std::collections::HashMap;

use aoc_runner_derive::*;

use anyhow::{anyhow, Result};
use nom::Finish;

const BITS: u64 = 36;

#[derive(Clone, Debug)]
pub struct Mask {
    ones: u64,
    zeros: u64,
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        (value & self.zeros) | self.ones
    }

    fn apply_v2(&self, value: u64) -> impl Iterator<Item = u64> {
        // X are denoted by ones and zeros being different.
        let floating_mask = self.ones ^ self.zeros;

        // prepare value so it can simply be combined with a floating mask. we also have to ensure
        // that all the ones in the mask are constant ones. all floating bits ares set to 0.
        let prepared_value = (value | self.ones) & !floating_mask;

        let floating_bits: Vec<_> = (0..BITS)
            .filter_map(|i| {
                if floating_mask & (1 << i) > 0 {
                    Some(1 << i)
                } else {
                    None
                }
            })
            .collect();

        (0..(1 << floating_bits.len())).map(move |i| {
            let float_mask = floating_bits
                .iter()
                .enumerate()
                .filter(|&(bi, _)| i & (1 << bi) > 0)
                .fold(0, |acc, (_, bit)| acc | bit);

            prepared_value | float_mask
        })
    }
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            ones: 0,
            zeros: (1 << BITS) - 1,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Mask(Mask),
    Mem { addr: u64, value: u64 },
}

#[derive(Default, Debug)]
struct Cpu {
    memory: HashMap<u64, u64>,
    mask: Mask,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Version {
    V1,
    V2,
}

impl Cpu {
    fn execute(&mut self, operation: &Operation, version: Version) {
        match operation {
            Operation::Mask(mask) => self.mask = mask.clone(),
            Operation::Mem { addr, value } => match version {
                Version::V1 => {
                    self.memory.insert(*addr, self.mask.apply(*value));
                }
                Version::V2 => {
                    let mem_writes = self.mask.apply_v2(*addr).map(|addr| (addr, *value));
                    self.memory.extend(mem_writes);
                }
            },
        }
    }
}

#[aoc_generator(day14)]
fn gen(input: &str) -> Result<Vec<Operation>> {
    input
        .lines()
        .map(|line| {
            parser::operation(line)
                .finish()
                .map(|(_, o)| o)
                .map_err(|e| anyhow!("unable to parse operation: {}", e))
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(operations: &[Operation]) -> u64 {
    let cpu = operations
        .iter()
        .fold(Cpu::default(), |mut cpu, operation| {
            cpu.execute(operation, Version::V1);
            cpu
        });

    cpu.memory.values().sum()
}

#[aoc(day14, part2)]
pub fn part2(operations: &[Operation]) -> u64 {
    let cpu = operations
        .iter()
        .fold(Cpu::default(), |mut cpu, operation| {
            cpu.execute(operation, Version::V2);
            cpu
        });

    cpu.memory.values().sum()
}

mod parser {
    use super::*;
    use nom::{
        branch::*, bytes::complete::*, character::complete::*, combinator::*, sequence::*, IResult,
    };

    fn number(s: &str) -> IResult<&str, u64> {
        map_res(digit1, |d: &str| d.parse())(s)
    }

    fn mem(s: &str) -> IResult<&str, Operation> {
        let addr = preceded(tag("["), terminated(number, tag("]")));

        map(
            preceded(tag("mem"), separated_pair(addr, tag(" = "), number)),
            |(addr, value)| Operation::Mem { addr, value },
        )(s)
    }

    fn mask(s: &str) -> IResult<&str, Operation> {
        let mask_str = preceded(
            tag("mask = "),
            take_while1(|c| matches!(c, 'X' | '0' | '1')),
        );

        map(mask_str, |mask_str: &str| {
            let mask =
                mask_str
                    .chars()
                    .rev()
                    .enumerate()
                    .fold(Mask::default(), |mut mask, (i, bit)| {
                        match bit {
                            '0' => mask.zeros ^= 1 << i,
                            '1' => mask.ones |= 1 << i,
                            _ => {}
                        };
                        mask
                    });

            Operation::Mask(mask)
        })(s)
    }

    pub fn operation(s: &str) -> IResult<&str, Operation> {
        alt((mask, mem))(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const EXAMPLE2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn part1_test() {
        assert_eq!(165, part1(&gen(EXAMPLE).unwrap()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(208, part2(&gen(EXAMPLE2).unwrap()));
    }
}
