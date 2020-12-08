use aoc_runner_derive::*;

use anyhow::{anyhow, Context, Result};
use std::collections::HashSet as Set;

#[derive(serde::Deserialize, PartialEq, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(serde::Deserialize, PartialEq, Clone, Copy, Debug)]
pub struct Instruction {
    operation: Operation,
    argument: isize,
}

#[aoc_generator(day8)]
fn gen(input: &str) -> Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| serde_scan::from_str(line).context("unable to parse instruction"))
        .collect()
}

/// Returns Ok(accumulator) if run to the end, and Err(accumulator) if an infinite loop was
/// detected.
fn run_program(instructions: &[Instruction]) -> Result<isize, isize> {
    let mut instr_counter: usize = 0;
    let mut accumulator = 0;

    // keeps track of executed instructions to detect infinite loops
    let mut instr_counter_log = Set::new();


    while !instr_counter_log.contains(&instr_counter) && instr_counter != instructions.len() {
        instr_counter_log.insert(instr_counter);

        let Instruction { operation, argument } = instructions[instr_counter];
        match operation {
            Operation::Acc => {
                accumulator += argument;
                instr_counter += 1;
            }
            Operation::Jmp => instr_counter = ((instr_counter as isize) + argument) as usize,
            Operation::Nop => instr_counter += 1,
        }
    }

    if instr_counter == instructions.len() {
        Ok(accumulator)
    } else {
        Err(accumulator)
    }
}

#[aoc(day8, part1)]
pub fn part1(instructions: &[Instruction]) -> isize {
    run_program(instructions).unwrap_err()
}

#[aoc(day8, part2)]
pub fn part2(instructions: &[Instruction]) -> Result<isize> {
    let mut instructions = instructions.to_vec();

    for flip_index in 0..instructions.len() {
        let mut instruction = &mut instructions[flip_index];

        // flip operation (if either jmp or nop)
        let prev_op = instruction.operation;
        if instruction.operation == Operation::Jmp {
            instruction.operation = Operation::Nop;
        } else if instruction.operation == Operation::Nop {
            instruction.operation = Operation::Jmp;
        } else {
            continue;
        }

        // try to run with flipped operation, should return Ok(_) if fully executed
        if let Ok(acc) = run_program(&instructions) {
            return Ok(acc);
        }

        // flip operation back for next iteration
        let mut instruction = &mut instructions[flip_index];
        instruction.operation = prev_op;
    }

    Err(anyhow!("found no solutions"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

    #[test]
    fn part1_test() {
        assert_eq!(5, part1(&gen(EXAMPLE).unwrap()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(8, part2(&gen(EXAMPLE).unwrap()).unwrap());
    }
}
