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

#[aoc(day6, part2)]
pub fn part2(input: &str) -> Result<usize> {
    let parents: BTreeMap<_, _> =
        convert(input.lines().map(Ok::<&str, Box<dyn std::error::Error>>))
            .map(|line| {
                let mut s = line.split(')');
                Ok((
                    s.next().ok_or("missing center")?,
                    s.next().ok_or("missing orbiter")?,
                ))
            })
            // Swap places for center and orbiter, so the map will have the child as key and parent
            // as value
            .map(|(center, orbiter)| Ok((orbiter, center)))
            .collect()?;

    let mut you_parents = std::iter::successors(parents.get("YOU"), |&n| parents.get(n));
    let san_parents = std::iter::successors(parents.get("SAN"), |&n| parents.get(n));

    // Calculate the distance from YOU and SAN to the first common parent and sum the distances
    you_parents
        .enumerate()
        .find_map(|(you_dist, you_parent)| {
            san_parents
                .clone()
                .enumerate()
                .find(|&(_, san_parent)| you_parent == san_parent)
                .map(|(san_dist, _)| san_dist + you_dist)
        })
        .ok_or_else(|| "did not find common parent".into())
}

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

    #[test]
    fn common_parent() {
        let input = concat!(
            "COM)B\n", "B)C\n", "C)D\n", "D)E\n", "E)F\n", "B)G\n", "G)H\n", "D)I\n", "E)J\n",
            "J)K\n", "K)L\n", "K)YOU\n", "I)SAN",
        );

        assert_eq!(4, part2(input).unwrap());
    }
}
