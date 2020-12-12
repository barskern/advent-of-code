use aoc_runner_derive::*;

use anyhow::{anyhow, Context, Result};
use nalgebra::{Matrix2, Point2, Rotation2, Unit, Vector2};

#[derive(Copy, Clone, PartialEq, serde::Deserialize, Debug)]
enum Compass {
    #[serde(rename = "N")]
    North,
    #[serde(rename = "E")]
    East,
    #[serde(rename = "S")]
    South,
    #[serde(rename = "W")]
    West,
}

impl Into<Unit<Vector2<i32>>> for Compass {
    fn into(self) -> Unit<Vector2<i32>> {
        match self {
            Compass::North => Vector2::y_axis(),
            Compass::East => Vector2::x_axis(),
            Compass::South => -Vector2::y_axis(),
            Compass::West => -Vector2::x_axis(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, serde::Deserialize, Debug)]
enum Direction {
    #[serde(rename = "L")]
    Left,
    #[serde(rename = "R")]
    Right,
    #[serde(rename = "F")]
    Forward,
}

#[derive(Copy, Clone, PartialEq, serde::Deserialize, Debug)]
#[serde(untagged)]
enum Control {
    Compass(Compass),
    Direction(Direction),
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
pub struct Instruction {
    control: Control,
    amount: usize,
}

#[aoc_generator(day12)]
fn gen(input: &str) -> Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            let control = serde_scan::from_str(&line[0..1])
                .map(Control::Compass)
                .or_else(|_| serde_scan::from_str(&line[0..1]).map(Control::Direction))
                .context(anyhow!("unable to parse control for '{}'", &line[0..1]))?;

            Ok(Instruction {
                control,
                amount: serde_scan::from_str(&line[1..])
                    .context(anyhow!("unable to parse amount for '{}'", &line[1..]))?,
            })
        })
        .collect()
}

#[derive(Clone, PartialEq, Debug)]
struct Ship {
    facing: Unit<Vector2<i32>>,
    position: Point2<i32>,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            facing: Vector2::x_axis(),
            position: Point2::new(0, 0),
        }
    }
}

impl Ship {
    fn execute(&mut self, instruction: &Instruction) {
        let Instruction { control, amount } = *instruction;
        match control {
            Control::Compass(control) => {
                let d: Unit<Vector2<i32>> = control.into();
                self.position += amount as i32 * d.into_inner();
            }
            Control::Direction(Direction::Right) => {
                let rot: Rotation2<i32> =
                    Rotation2::from_matrix_unchecked(Matrix2::new(0, 1, -1, 0));
                self.facing = Unit::new_unchecked(rot * self.facing.into_inner());
            }
            Control::Direction(Direction::Left) => {
                let rot: Rotation2<i32> =
                    Rotation2::from_matrix_unchecked(Matrix2::new(0, -1, 1, 0));
                self.facing = Unit::new_unchecked(rot * self.facing.into_inner());
            }
            Control::Direction(Direction::Forward) => {
                self.position += amount as i32 * self.facing.into_inner();
            }
        }
    }

    fn manhatten_distance(&self) -> i32 {
        self.position.x + self.position.y
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    input
        .iter()
        .fold(Ship::default(), |mut ship, instruction| {
            ship.execute(instruction);
            ship
        })
        .manhatten_distance()
}

// #[aoc(day12, part2)]
// pub fn part2(input: &str) -> Result<usize> {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {}
