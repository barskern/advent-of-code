use aoc_runner_derive::*;
use fallible_iterator::{convert, FallibleIterator};
use std::collections::BTreeMap;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> Result<usize> {
    let m = convert(input.lines().map(Ok::<&str, Box<dyn std::error::Error>>))
        .map(|line| {
            let mut s = line.split(')');
            Ok((
                s.next().ok_or("missing center")?,
                s.next().ok_or("missing orbiter")?,
            ))
        })
        .fold(
            BTreeMap::<&str, Vec<&str>>::new(),
            |mut map, (center, orbiter)| {
                map.entry(center)
                    .and_modify(|children| children.push(orbiter))
                    .or_insert_with(|| vec![orbiter]);
                Ok(map)
            },
        )?;

    fn orbit_count(m: &BTreeMap<&str, Vec<&str>>, name: &str, depth: usize) -> usize {
        let sub_orbits = m
            .get(name)
            .map(|orbits| {
                orbits
                    .iter()
                    .map(|orbit_name| orbit_count(m, orbit_name, depth + 1))
                    .sum()
            })
            .unwrap_or(0);

        depth + sub_orbits
    }

    Ok(orbit_count(&m, "COM", 0))
}

// #[aoc(day6, part2)]
// pub fn part2(input: &str) -> Result<usize> {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let input = concat!(
            "COM)B\n", "B)C\n", "C)D\n", "D)E\n", "E)F\n", "B)G\n", "G)H\n", "D)I\n", "E)J\n",
            "J)K\n", "K)L"
        );

        assert_eq!(42, part1(input).unwrap());
    }
}
