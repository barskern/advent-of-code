use crate::intcode::Machine;
use aoc_runner_derive::*;
use nalgebra::{Matrix2, Point2, Unit, Vector2};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::mpsc::sync_channel;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> Result<usize> {
    let memory: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let (mut machine, input, output) = {
        let (input_tx, input_rx) = sync_channel::<isize>(10);
        let (output_tx, output_rx) = sync_channel::<isize>(10);
        (
            Machine::new(memory, input_rx, output_tx),
            input_tx,
            output_rx,
        )
    };

    let mut panel = Panel::default();
    let mut robot = Robot::new();

    loop {
        let color = panel.color(&robot.pos);
        input.send((*color).into()).unwrap();

        // Run robot program until next requested input
        match machine.run() {
            Err(crate::intcode::Error::WouldBlock) => {}
            r @ Ok(_) | r @ Err(_) => break r,
        }

        // Convert output
        let (new_color, turn): (Color, Turn) =
            (output.recv()?.try_into()?, output.recv()?.try_into()?);

        // Update panel
        panel.paint(robot.pos, new_color);

        // Rotate and move robot
        robot.rotate(turn);
        robot.step();
    }?;

    Ok(panel.state.len())
}

// #[aoc(day11, part2)]
// pub fn part2(input: &str) -> Result<usize> {
//     unimplemented!()
// }

#[derive(Debug)]
struct Robot {
    pos: Point2<isize>,
    dir: Unit<Vector2<isize>>,
}

impl Robot {
    fn new() -> Self {
        Robot {
            pos: Point2::new(0, 0),
            dir: -Vector2::y_axis(),
        }
    }

    fn rotate(&mut self, turn: Turn) {
        let rot: Matrix2<isize> = match turn {
            Turn::Left => Matrix2::new(0, -1, 1, 0),
            Turn::Right => Matrix2::new(0, 1, -1, 0),
        };

        // This is safe because the matrix only rotates (doesn't scale the vector)
        self.dir = Unit::new_unchecked(rot * self.dir.into_inner());
    }

    fn step(&mut self) {
        self.pos += self.dir.into_inner();
    }
}

#[derive(Debug, Default)]
struct Panel {
    state: HashMap<Point2<isize>, Color>,
}

impl Panel {
    fn color(&self, p: &Point2<isize>) -> &Color {
        self.state.get(p).unwrap_or(&Color::Black)
    }

    fn paint(&mut self, p: Point2<isize>, color: Color) {
        self.state.insert(p, color);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(isize)]
enum Color {
    Black = 0,
    White = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(isize)]
enum Turn {
    Left = 0,
    Right = 1,
}

#[cfg(test)]
mod tests {}
