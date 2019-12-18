#![allow(warnings)]

use aoc_runner_derive::*;
use itertools::Itertools;
use nalgebra::{Point3, Vector3};
use serde_scan::scan;

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

// #[aoc(day12, part2)]
// pub fn part2(input: &str) -> Result<usize> {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple() {}
}
