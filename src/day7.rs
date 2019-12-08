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

// #[aoc(day7, part2)]
// pub fn part2(input: &str) -> Result<usize> {
//     let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

//     convert((5..10).permutations(5).map(Ok::<Vec<usize>, Error>))
//         .map(|phases| {
//             let pipes: Vec<_> = phases
//                 .iter()
//                 .map(|phase| RefCell::new(Cursor::new(format!("{}\n", phase).into_bytes())))
//                 .collect();

//             writeln!(pipes[0].borrow_mut(), "0")?;

//             pipes
//                 .iter()
//                 .zip(pipes.iter().cycle().skip(1))
//                 .try_for_each(|(input, output)| {
//                     eprintln!("test");
//                     let (mut input, mut output) = (input.borrow_mut(), output.borrow_mut());
//                     let mut machine = Machine::new(memory.clone(), &mut *input, &mut *output);
//                     machine.run().map(|_| ())
//                 })?;

//             unimplemented!()
//         })
//         .max()?
//         .ok_or_else(|| "no maximum output thrust".into())
// }

#[cfg(test)]
mod tests {}
