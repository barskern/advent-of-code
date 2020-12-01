use crate::intcode::Machine;
use aoc_runner_derive::*;
use fallible_iterator::{convert, FallibleIterator};
use itertools::Itertools;
use std::sync::mpsc::sync_channel;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

const AMPLIFIER_COUNT: usize = 5;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> Result<isize> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    convert(
        (0..AMPLIFIER_COUNT as isize)
            .permutations(AMPLIFIER_COUNT)
            .map(Ok::<Vec<isize>, Error>),
    )
    .map(|phases| {
        let (mut txs, mut rxs): (Vec<_>, Vec<_>) =
            std::iter::repeat_with(|| sync_channel::<isize>(10))
                .take(AMPLIFIER_COUNT + 1)
                .unzip();

        // Set phase settings
        txs.iter_mut()
            .take(AMPLIFIER_COUNT)
            .zip(phases)
            .for_each(|(tx, phase)| tx.send(phase).unwrap());

        // First tx and last rx is the input and output of the system
        let (input, output) = (txs.remove(0), rxs.remove(AMPLIFIER_COUNT));

        let mut machines: Vec<_> = rxs
            .drain(..)
            .zip(txs.drain(..))
            .map(|(rx, tx)| Machine::new(memory.clone(), rx, tx))
            .collect();

        input.send(0).unwrap();

        for machine in &mut machines {
            match machine.run() {
                Ok(_) | Err(crate::intcode::Error::WouldBlock) => {}
                Err(e) => return Err(e.into()),
            }
        }

        Ok(output.recv().unwrap())
    })
    .max()?
    .ok_or_else(|| "no maximum thrust".into())
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> Result<isize> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    convert(
        (0..AMPLIFIER_COUNT as isize)
            .map(|p| p + 5)
            .permutations(AMPLIFIER_COUNT)
            .map(Ok::<Vec<isize>, Error>),
    )
    .map(|phases| {
        let (mut txs, mut rxs): (Vec<_>, Vec<_>) =
            std::iter::repeat_with(|| sync_channel::<isize>(10))
                .take(AMPLIFIER_COUNT + 1)
                .unzip();

        // Set phase settings
        txs.iter_mut()
            .take(AMPLIFIER_COUNT)
            .zip(phases)
            .for_each(|(tx, phase)| tx.send(phase).unwrap());

        // First tx and last rx is the input and output of the system
        let (input, output) = (txs.remove(0), rxs.remove(AMPLIFIER_COUNT));

        let mut machines: Vec<_> = rxs
            .drain(..)
            .zip(txs.drain(..))
            .map(|(rx, tx)| Machine::new(memory.clone(), rx, tx))
            .collect();

        // Send input to start of first amplifier
        input.send(0).unwrap();

        let mut running = true;
        while running {
            for machine in &mut machines {
                match machine.run() {
                    Ok(_) => running = false,
                    Err(crate::intcode::Error::WouldBlock) => {}
                    Err(e) => return Err(e.into()),
                }
            }
            if running {
                // We are still running so we pipe the output into the input (feedback loop)
                output.try_iter().try_for_each(|v| input.send(v))?;
            }
        }

        Ok(output.recv().unwrap())
    })
    .max()?
    .ok_or_else(|| "no maximum thrust".into())
}

#[cfg(test)]
mod tests {}
