use crate::intcode::Machine;
use aoc_runner_derive::*;
use fallible_iterator::{convert, FallibleIterator};
use itertools::Itertools;
use std::cell::RefCell;
use std::io::Cursor;
use std::io::Write;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> Result<usize> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    convert((0..5).permutations(5).map(Ok::<Vec<usize>, Error>))
        .map(|phases| {
            phases.iter().try_fold(0, |input_value, phase| {
                // Create input and output for amplifier node
                let input = format!("{}\n{}\n", phase, input_value);
                let mut output = Vec::new();

                let mut machine = Machine::new(memory.clone(), input.as_bytes(), &mut output);
                machine.run()?;

                String::from_utf8_lossy(&output)
                    .trim()
                    .parse()
                    .map_err(Error::from)
            })
        })
        .max()?
        .ok_or_else(|| "no maximum output thrust".into())
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> Result<usize> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    convert((5..10).permutations(5).map(Ok::<Vec<usize>, Error>))
        .map(|phases| {
            let pipes: Vec<_> = phases
                .iter()
                .map(|phase| RefCell::new(Cursor::new(format!("{}\n", phase).into_bytes())))
                .collect();

            writeln!(pipes[0].borrow_mut(), "0")?;

            pipes
                .iter()
                .zip(pipes.iter().cycle().skip(1))
                .try_for_each(|(input, output)| {
                    eprintln!("test");
                    let (mut input, mut output) = (input.borrow_mut(), output.borrow_mut());
                    let mut machine = Machine::new(memory.clone(), &mut *input, &mut *output);
                    machine.run().map(|_| ())
                })?;

            unimplemented!()
        })
        .max()?
        .ok_or_else(|| "no maximum output thrust".into())
}

#[cfg(test)]
mod tests {}
