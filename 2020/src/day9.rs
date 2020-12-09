use aoc_runner_derive::*;

use anyhow::{Context, Result};

const WINDOW_SIZE: usize = 25;

#[aoc_generator(day9)]
fn gen(input: &str) -> Result<Vec<usize>> {
    input
        .lines()
        .map(|line| line.parse().context("unable to parse number"))
        .collect()
}

/// `chosable` MUST be sorted and the fuction returns the indicies of the two-sum values in the
/// sorted array chosable.
pub fn two_sum(chosable: &[usize], wanted_sum: usize) -> Option<(usize, usize)> {
    let mut small_index = 0;
    let mut big_index = chosable.len() - 1;

    while small_index < big_index {
        let sum = chosable[small_index] + chosable[big_index];

        if sum > wanted_sum {
            // sum is too big, hence the current big number is too big to be part of sum
            big_index -= 1;
        } else if sum < wanted_sum {
            // sum is too small, hence the current small number is too small to be part of sum
            small_index += 1;
        } else if chosable[small_index] == chosable[big_index] {
            // small and big numbers being equal means we have reached the center and the center
            // consists of duplicates of the same numbers.
            break;
        } else if sum == wanted_sum {
            return Some((small_index, big_index));
        }
    }
    None
}

#[aoc(day9, part1)]
pub fn part1(input: &[usize]) -> Result<usize> {
    let mut chosable = vec![0; WINDOW_SIZE];

    input
        .windows(WINDOW_SIZE + 1)
        .find_map(|window| {
            chosable.copy_from_slice(&window[0..WINDOW_SIZE]);
            chosable.sort_unstable();

            let wanted_sum = window[WINDOW_SIZE];

            if two_sum(&chosable, wanted_sum).is_none() {
                Some(wanted_sum)
            } else {
                None
            }
        })
        .context("did not find any non-two-sum number")
}

#[aoc(day9, part2)]
pub fn part2(input: &[usize]) -> Result<usize> {
    let wrong_sum = part1(input)?;

    let mut small_index = 0;
    let mut big_index = 1;

    while big_index <= input.len() {
        let window = &input[small_index..big_index];

        let sum: usize = window.iter().sum();

        if sum < wrong_sum {
            big_index += 1;
        } else if sum > wrong_sum {
            small_index += 1;
        } else if sum == wrong_sum {
            return Ok(window.iter().min().unwrap() + window.iter().max().unwrap());
        }
    }

    Err(anyhow::anyhow!("did not find any slice with wrong-sum"))
}

#[cfg(test)]
mod tests {}
