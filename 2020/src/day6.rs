use aoc_runner_derive::*;

use anyhow::Result;
use std::collections::HashSet as Set;

type Answers = Set<char>;
type GroupAnswers = Vec<Answers>;

#[aoc_generator(day6)]
pub fn gen(input: &str) -> Vec<GroupAnswers> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(groups: &[GroupAnswers]) -> usize {
    groups
        .iter()
        .map(|group_answers| {
            group_answers
                .iter()
                .fold(Answers::new(), |ref state, answers| state | answers)
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(groups: &[GroupAnswers]) -> usize {
    let all_answers: Answers = ('a'..='z').collect();

    groups
        .iter()
        .map(|group_answers| {
            group_answers
                .iter()
                .fold(all_answers.clone(), |ref state, answers| state & answers)
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {}
