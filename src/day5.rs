#![allow(warnings)]

use crate::intcode::Machine;
use aoc_runner_derive::*;
use std::convert::{TryFrom, TryInto};
use std::io::BufReader;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> Result<String> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let input = "1\n";
    let mut output = Vec::new();

    let mut machine = Machine::new(memory, BufReader::new(input.as_bytes()), &mut output);
    machine.run()?;

    Ok(String::from_utf8_lossy(&output).to_string())
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> Result<String> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let input = "5\n";
    let mut output = Vec::new();

    let mut machine = Machine::new(memory, BufReader::new(input.as_bytes()), &mut output);
    machine.run()?;

    Ok(String::from_utf8_lossy(&output).to_string())
}
