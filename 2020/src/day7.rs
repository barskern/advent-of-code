use aoc_runner_derive::*;

use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::BTreeMap as Map;
use std::collections::HashSet as Set;

type BagTree = Map<String, Map<String, usize>>;

const MY_BAG: &str = "shiny gold";

#[aoc_generator(day7)]
fn gen(input: &str) -> Result<BagTree> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split("bags contain").map(|s| s.trim());
            let (holder, contain_str) = (
                it.next().context("input missing 'bags contain'")?,
                it.next().context("input missing 'bags contain'")?,
            );

            let contained_bags = contain_str
                .split(',')
                .filter(|contain_str| !contain_str.contains("no other bags"))
                .map(|contain_str| {
                    Ok((
                        // extract color (skip number and take next to words)
                        contain_str.split_whitespace().skip(1).take(2).join(" "),
                        // extract count (parse first word as number)
                        contain_str
                            .split_whitespace()
                            .next()
                            .context("missing count")
                            .and_then(|count_str| {
                                count_str.parse::<usize>().context("unable to parse count")
                            })?,
                    ))
                })
                .collect::<Result<_>>()?;

            Ok((holder.to_string(), contained_bags))
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(bag_tree: &BagTree) -> usize {
    let mut stack = Vec::new();
    let mut can_contain = Set::new();

    // for each bag color (holder), check if can contain MY_BAG, and add to `can_contain` if it
    // does (enables a small optimization).
    for (outer_bag, inner_bags) in bag_tree.iter() {
        stack.extend(inner_bags);
        while let Some((inner_bag, _)) = stack.pop() {
            if inner_bag == MY_BAG || can_contain.contains(inner_bag) {
                // either the contained bag is my bag or we have previously found the inner bag can
                // contain my bag (optimization)
                can_contain.insert(outer_bag);
                stack.clear();
                break;
            } else if let Some(inner_inner_bags) = bag_tree.get(inner_bag) {
                stack.extend(inner_inner_bags);
            }
        }
    }

    can_contain.len()
}

#[aoc(day7, part2)]
pub fn part2(bag_tree: &BagTree) -> usize {
    let mut stack = Vec::new();
    let mut total_count = 0;

    stack.push((MY_BAG, 1usize));
    while let Some((outer_bag, count)) = stack.pop() {
        total_count += count;

        if let Some(inner_bags) = bag_tree.get(outer_bag) {
            // add contained bags to stack, and multiply their count by the current count, this
            // ensures that the "depth" of the containment is taken into account
            stack.extend(
                inner_bags
                    .iter()
                    .map(|(inner_bag, inner_count)| (&**inner_bag, count * inner_count)),
            );
        }
    }

    // subtract one because the top level bag isn't part of the count
    total_count - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;

    #[test]
    fn part2_test() {
        assert_eq!(126, part2(&gen(EXAMPLE).unwrap()));
    }
}
