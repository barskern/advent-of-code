use crate::intcode::Machine;
use aoc_runner_derive::*;
use std::sync::mpsc::sync_channel;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> Result<isize> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let (input_tx, input_rx) = sync_channel::<isize>(4);
    let (output_tx, output_rx) = sync_channel::<isize>(10);

    let mut machine = Machine::new(memory, input_rx, output_tx);

    // Run in test mode
    let _ = input_tx.send(1);
    machine.run()?;

    let results: Vec<_> = output_rx.try_iter().collect();
    if results.len() != 1 {
        return Err(format!("failed running opcodes: {:?}", results).into());
    }

    Ok(*results.last().unwrap())
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> Result<isize> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let (input_tx, input_rx) = sync_channel::<isize>(4);
    let (output_tx, output_rx) = sync_channel::<isize>(4);

    let mut machine = Machine::new(memory, input_rx, output_tx);

    // Run in boost mode
    let _ = input_tx.send(2);
    machine.run()?;

    let results: Vec<_> = output_rx.try_iter().collect();
    if results.len() != 1 {
        return Err(format!("failed running opcodes: {:?}", results).into());
    }

    Ok(*results.last().unwrap())
}

#[cfg(test)]
mod tests {}
