#![allow(warnings)]

use crate::intcode::Machine;
use aoc_runner_derive::*;
use std::convert::{TryFrom, TryInto};
use std::sync::mpsc::sync_channel;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> Result<isize> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let (input_tx, input_rx) = sync_channel::<isize>(4);
    let _ = input_tx.send(1);

    let (output_tx, output_rx) = sync_channel::<isize>(10);

    let mut machine = Machine::new(memory, input_rx, output_tx);
    machine.run()?;

    let results: Vec<_> = output_rx.try_iter().collect();
    if results.len() == 0 || results.iter().take(results.len() - 1).any(|&x| x != 0) {
        return Err("diagnostic test failed or no output".into());
    }

    Ok(*results.last().unwrap())
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> Result<isize> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let (input_tx, input_rx) = sync_channel::<isize>(4);
    let _ = input_tx.send(5);

    let (output_tx, output_rx) = sync_channel::<isize>(1);

    let mut machine = Machine::new(memory, input_rx, output_tx);
    machine.run()?;

    Ok(output_rx.recv()?)
}
