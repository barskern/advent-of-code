use aoc_runner_derive::*;

use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::BTreeSet as Set;

#[aoc_generator(day1)]
fn gen(input: &str) -> Result<Vec<usize>> {
    input
        .lines()
        .map(|s| s.parse().context("unable to parse number"))
        .collect()
}

#[aoc(day1, part1, brute)]
pub fn part1_brute(input: &[usize]) -> Result<usize> {
    input
        .iter()
        .tuple_combinations()
        .find(|(&x, &y)| x + y == 2020)
        .context("no combination of numbers sums to 2020")
        .map(|(x, y)| x * y)
}

#[aoc(day1, part1, set)]
pub fn part1_set(input: &[usize]) -> Result<usize> {

    let mut set = Set::new();
    for value in input {
        if set.contains(&(2020 - value)) {
            if *value != 2020 - value {
                // we've only found the answer if the numbers are different
                return Ok(value * (2020 - value));
            }
        } else {
            set.insert(value);
        }
    }

    Err(anyhow::anyhow!("no numbers sum to 2020"))
}

#[aoc(day1, part1, inward)]
pub fn part1_inward(input: &[usize]) -> Result<usize> {
    let mut input = input.to_vec();

    // sort input such that smallest is at the left and biggest at the right
    input.sort_unstable();

    let mut small_index = 0;
    let mut big_index = input.len() - 1;

    while small_index < big_index {
        let sum = input[small_index] + input[big_index];

        if sum > 2020 {
            // sum is too big, hence the current big number is too big to be part of sum
            big_index -= 1;

        } else if sum < 2020 {
            // sum is too small, hence the current small number is too small to be part of sum
            small_index += 1;

        } else if input[small_index] == input[big_index] {
            // small and big numbers being equal means we have reached the center and the center
            // consists of duplicates of the same numbers.
            break;

        } else if sum == 2020 {
            return Ok(input[small_index] * input[big_index]);
        }
    }

    Err(anyhow::anyhow!("no numbers in input summed to 2020"))
}

#[aoc(day1, part2, brute)]
pub fn part2_brute(input: &[usize]) -> Result<usize> {
    input
        .iter()
        .tuple_combinations()
        .find(|(&x, &y, &z)| x + y + z == 2020)
        .context("no combination of numbers sums to 2020")
        .map(|(x, y, z)| x * y * z)
}

#[aoc(day1, part2, inward)]
pub fn part2_inward(input: &[usize]) -> Result<usize> {
    let mut input = input.to_vec();

    // sort input such that smallest is at the left and biggest at the right
    input.sort_unstable();

    // search for two-sum pair using each value in the array as a offset
    for offset in &input {
        let mut small_index = 0;
        let mut big_index = input.len() - 1;

        while small_index < big_index {
            let sum = input[small_index] + input[big_index] + offset;

            if sum > 2020 || input[big_index] == *offset {
                // sum is too big, hence the current big number is too big to be part of sum
                // number also can't be equal to offset, because numbers have to be unique
                big_index -= 1;

            } else if sum < 2020 || input[small_index] == *offset {
                // sum is too small, hence the current small number is too small to be part of sum
                // number also can't be equal to offset, because numbers have to be unique
                small_index += 1;

            } else if input[small_index] == input[big_index] {
                // small and big numbers being equal means we have reached the center and the center
                // consists of duplicates of the same numbers.
                break;

            } else if sum == 2020 {
                return Ok(input[small_index] * input[big_index] * offset);
            }
        }
    }

    Err(anyhow::anyhow!("no numbers in input summed to 2020"))
}

#[cfg(test)]
mod tests {}
