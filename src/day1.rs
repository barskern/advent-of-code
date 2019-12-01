use aoc_runner_derive::*;
use std::iter::successors;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .map(|w| (w / 3) - 2)
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .flat_map(|w| successors(Some(w), |w| (w / 3).checked_sub(2)).skip(1))
        .sum()
}
