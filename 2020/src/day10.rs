use aoc_runner_derive::*;

use anyhow::{Context, Result};
use itertools::Itertools;

#[aoc_generator(day10)]
fn gen(input: &str) -> Result<Vec<usize>> {
    input
        .lines()
        .map(|line| line.parse().context("invalid number"))
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut adapters = input.to_vec();
    adapters.sort_unstable();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);

    let diffs = adapters.windows(2).map(|win| win[1] - win[0]);

    let (count_diff_1, count_diff_3) =
        diffs.fold((0, 0), |(count_diff_1, count_diff_3), diff| match diff {
            1 => (count_diff_1 + 1, count_diff_3),
            3 => (count_diff_1, count_diff_3 + 1),
            _ => (count_diff_1, count_diff_3),
        });

    count_diff_1 * count_diff_3
}

fn tribonacci(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        2 => 2,
        n => tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3),
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut adapters = input.to_vec();
    adapters.sort_unstable();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);

    let diffs = std::iter::once(adapters[0]).chain(adapters.windows(2).map(|win| win[1] - win[0]));

    let gap_indices = diffs.positions(|diff| diff == 3);

    let continous_slice_indices = std::iter::once(0)
        .chain(gap_indices)
        .chain(std::iter::once(adapters.len()))
        .tuple_windows();

    continous_slice_indices
        .map(|(start, end)| tribonacci(end - start - 1))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

    const LONG_EXAMPLE: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;

    #[test]
    fn part1_test() {
        assert_eq!(7 * 5, part1(&gen(EXAMPLE).unwrap()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(8, part2(&gen(EXAMPLE).unwrap()));
    }

    #[test]
    fn part2_long_test() {
        assert_eq!(19208, part2(&gen(LONG_EXAMPLE).unwrap()));
    }
}
