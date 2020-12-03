use aoc_runner_derive::*;

use anyhow::{Result};

#[aoc(day2, part1)]
pub fn part1(input: &str) -> Result<usize> {
    fn compliant_password(s: &str) -> bool {
        let (lower, upper, letter, password): (usize, usize, char, &str) =
            serde_scan::scan!("{}-{} {}: {}" <- s).unwrap();

        let matches = password.matches(letter).count();

        lower <= matches && matches <= upper
    }

    Ok(input.lines().filter(|s| compliant_password(s)).count())
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> Result<usize> {
    fn compliant_password(s: &str) -> bool {
        let (one, two, letter, password): (usize, usize, char, &str) =
            serde_scan::scan!("{}-{} {}: {}" <- s).unwrap();

        (password.chars().nth(one - 1).unwrap() == letter)
            ^ (password.chars().nth(two - 1).unwrap() == letter)
    }

    Ok(input.lines().filter(|s| compliant_password(s)).count())
}

#[cfg(test)]
mod tests {}
