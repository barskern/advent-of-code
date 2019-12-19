#![allow(warnings)]

use aoc_runner_derive::*;
use itertools::Itertools;
use nalgebra::{Point3, Vector3};
use serde_scan::scan;
use std::collections::HashSet;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

const SIMULATION_STEPS: usize = 1000;

#[aoc_generator(day12)]
fn gen(input: &str) -> Result<Vec<Point3<isize>>, serde_scan::ScanError> {
    input
        .lines()
        .map(|line| scan!("<x={}, y={}, z={}>" <- line))
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(positions: &[Point3<isize>]) -> isize {
    let (positions, velocities) = (0..SIMULATION_STEPS).fold(
        (positions.to_vec(), vec![Vector3::zeros(); positions.len()]),
        |(mut positions, mut velocities), _| {
            // Apply gravity
            (0..positions.len())
                .tuple_combinations()
                .for_each(|(a, b)| {
                    let delta = (positions[b] - positions[a]).map(|n| n.signum());
                    velocities[a] += delta;
                    velocities[b] += -delta;
                });

            // Apply velocity
            positions
                .iter_mut()
                .zip(velocities.iter())
                .for_each(|(pos, vel)| *pos += vel);

            (positions, velocities)
        },
    );

    // Calculate total energy
    positions
        .iter()
        .zip(velocities.iter())
        .map(|(pos, vel)| pos.coords.abs().iter().sum::<isize>() * vel.abs().iter().sum::<isize>())
        .sum::<isize>()
}

#[aoc(day12, part2)]
pub fn part2(positions: &[Point3<isize>]) -> isize {
    let deltas: Vector3<_> = positions
        .iter()
        .flat_map(|a| positions.iter().map(move |b| (b - a).abs()))
        .sum();

    let (positions, velocities) = (0..SIMULATION_STEPS).fold(
        (positions.to_vec(), vec![Vector3::zeros(); positions.len()]),
        |(mut positions, mut velocities), i| {
            eprintln!("### {} ###", i);
            // Print positions
            positions
                .iter()
                .zip(velocities.iter())
                .for_each(|(pos, vel)| eprintln!("({}, {}) <{}, {}>", pos.x, pos.y, vel.x, vel.y));

            // Apply gravity
            (0..positions.len())
                .tuple_combinations()
                .for_each(|(a, b)| {
                    let delta = (positions[b] - positions[a]).map(|n| n.signum());
                    velocities[a] += delta;
                    velocities[b] += -delta;
                });

            // Apply velocity
            positions
                .iter_mut()
                .zip(velocities.iter())
                .for_each(|(pos, vel)| *pos += vel);

            std::io::stdin().read_line(&mut String::new());

            (positions, velocities)
        },
    );

    deltas.iter().sum::<isize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn super_simple_example() {
        let input = concat!("<x=-1, y=0, z=0>\n", "<x=2, y=0, z=0>",);

        assert_eq!(6, part2(&gen(input).unwrap()));
    }

    #[test]
    fn super_simple_example2() {
        let input = concat!("<x=-1, y=1, z=0>\n", "<x=2, y=2, z=0>",);

        assert_eq!(8, part2(&gen(input).unwrap()));
    }

    #[test]
    fn super_simple_example3() {
        let input = concat!(
            "<x=-1, y=-1, z=0>\n",
            "<x=1, y=-2, z=0>\n",
            "<x=2, y=2, z=0>\n",
        );

        assert_eq!(6, part2(&gen(input).unwrap()));
    }

    #[test]
    fn super_simple_example4() {
        let input = concat!(
            "<x=-1, y=0, z=0>\n",
            "<x=1, y=0, z=0>\n",
            "<x=0, y=1, z=0>\n",
        );

        assert_eq!(6, part2(&gen(input).unwrap()));
    }

    // #[test]
    // fn simple_example() {
    //     let input = concat!(
    //         "<x=-1, y=0, z=2>\n",
    //         "<x=2, y=-10, z=-7>\n",
    //         "<x=4, y=-8, z=8>\n",
    //         "<x=3, y=5, z=-1>",
    //     );

    //     assert_eq!(2772, part2(&gen(input).unwrap()));
    // }
}
