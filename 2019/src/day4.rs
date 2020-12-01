#![allow(warnings)]

use aoc_runner_derive::*;
use itertools::Itertools;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> Result<usize> {
    let (start, end): (u32, u32) = {
        let mut ns = input.split('-').map(|s| s.parse());
        (
            ns.next().ok_or_else(|| "missing number")??,
            ns.next().ok_or_else(|| "missing number")??,
        )
    };

    fn valid_password(n: &u32) -> bool {
        let digits: Vec<_> = digits(*n).collect();

        let is_increasing = digits.windows(2).all(|d| d[0] <= d[1]);
        let has_double = digits.windows(2).any(|d| d[0] == d[1]);

        is_increasing && has_double
    }

    Ok((start..=end).filter(valid_password).count())
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> Result<usize> {
    let (start, end): (u32, u32) = {
        let mut ns = input.split('-').map(|s| s.parse());
        (
            ns.next().ok_or_else(|| "missing number")??,
            ns.next().ok_or_else(|| "missing number")??,
        )
    };

    fn valid_password(n: &u32) -> bool {
        let digits: Vec<_> = digits(*n).collect();

        let is_increasing = digits.windows(2).all(|d| d[0] <= d[1]);
        let has_exact_double = digits
            .iter()
            .group_by(|&k| k)
            .into_iter()
            .any(|(_, g)| g.count() == 2);

        is_increasing && has_exact_double
    }

    Ok((start..=end).filter(valid_password).count())
}

fn digits(n: u32) -> impl Iterator<Item = u8> + Clone {
    (0..6).rev().map(move |x| ((n / 10u32.pow(x)) % 10) as u8)
}
