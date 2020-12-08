use aoc_runner_derive::*;

use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Seat {
    column: usize,
    row: usize,
}

impl Seat {
    fn decode(s: &str) -> Result<Self> {
        let row = s[0..=6]
            .chars()
            .rev()
            .enumerate()
            .try_fold(0, |state, (offset, c)| match c {
                'F' => Ok(state),
                'B' => Ok(state + (1 << offset)),
                _ => Err(anyhow!("got '{}', expected 'F' or 'B'", c)),
            })
            .context("unable to decode row")?;

        let column = s[7..=9]
            .chars()
            .rev()
            .enumerate()
            .try_fold(0, |state, (offset, c)| match c {
                'L' => Ok(state),
                'R' => Ok(state + (1 << offset)),
                _ => Err(anyhow!("got '{}', expected 'L' or 'R'", c)),
            })
            .context("unable to decode column")?;

        Ok(Seat { column, row })
    }

    fn seat_id(&self) -> usize {
        8 * self.row + self.column
    }
}

#[aoc_generator(day5)]
pub fn gen(input: &str) -> Result<Vec<Seat>> {
    input.lines().map(|s| Seat::decode(s)).collect()
}

#[aoc(day5, part1)]
pub fn part1(seats: &[Seat]) -> Result<usize> {
    seats
        .iter()
        .map(|s| s.seat_id())
        .max()
        .context("no valid seats")
}

#[aoc(day5, part2)]
pub fn part2(seats: &[Seat]) -> Result<usize> {
    let mut seats = seats.to_vec();

    seats.sort_by_key(|s| s.seat_id());

    let neighbours = seats
        .windows(2)
        // our seat is where there is a gap in seat ids (e.g. seat_id diff > 1)
        .find(|neighbours| (neighbours[1].seat_id() - neighbours[0].seat_id()) > 1)
        .context("no holes in seats")?;

    // my seat is between the neighbours
    let my_seat = Seat {
        column: neighbours[0].column + (neighbours[1].column - neighbours[0].column) / 2,
        row: neighbours[0].row + (neighbours[1].row - neighbours[0].row) / 2,
    };

    Ok(my_seat.seat_id())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const EXAMPLES: &[(&str, Seat)] = &[
        ("BFFFBBFRRR", Seat { row: 70, column: 7 }),
        ("FFFBBBFRRR", Seat { row: 14, column: 7 }),
        ("BBFFBBFRLL", Seat { row: 102, column: 4, }),
    ];

    #[test]
    fn parsing_valid_seats_test() {
        EXAMPLES
            .iter()
            .for_each(|(s, seat)| assert_eq!(seat, &Seat::decode(s).unwrap()));
    }
}
