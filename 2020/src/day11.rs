use std::{cmp::Ordering, ops::{Index, IndexMut}, str::FromStr};

use aoc_runner_derive::*;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::iter::repeat;

#[aoc_generator(day11)]
fn gen(input: &str) -> Result<Seats> {
    input.parse().context("unable to parse seats")
}

fn run(mut seats: Seats, proximity: Proximity) -> usize{
    std::iter::repeat_with(|| {
        seats.step(proximity);
        seats.occupied_seats()
    })
    .tuple_windows()
    .find(|(a, b)| a == b)
    .map(|(a, _)| a)
    .expect("unreachable: repeat_with is never empty")
}

#[aoc(day11, part1)]
pub fn part1(input: &Seats) -> usize {
    run(input.clone(), Proximity::Immidiate)
}

#[aoc(day11, part2)]
pub fn part2(input: &Seats) -> usize {
    run(input.clone(), Proximity::Visible)
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Proximity {
    Immidiate,
    Visible,
}

impl Proximity {
    fn occupied_threshold(&self) -> usize {
        match *self {
            Self::Immidiate => 4,
            Self::Visible => 5,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tile {
    Floor,
    Vacant,
    Occupied,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Seats {
    data: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Seats {
    fn occupied_seats(&self) -> usize {
        self.data
            .iter()
            .filter(|&&seat| seat == Tile::Occupied)
            .count()
    }

    fn visible_neighbours_of<'s>(
        &'s self,
        (y, x): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + 's {
        use Ordering::*;

        let is_seat = move |(y, x)| matches!(self[(y, x)], Tile::Vacant | Tile::Occupied);

        #[rustfmt::skip]
        let neighbour_positions = self.immidiate_neighbours_of((y, x))
            .filter_map(move |(ny, nx)| match (ny.cmp(&y), nx.cmp(&x)) {
                (Less,    Less)    =>  ((0..=ny).rev()).zip((0..=nx).rev()).find(|&p| is_seat(p)),
                (Less,    Equal)   =>  ((0..=ny).rev()).zip(    repeat(nx)).find(|&p| is_seat(p)),
                (Less,    Greater) =>  ((0..=ny).rev()).zip(nx..self.width).find(|&p| is_seat(p)),
                (Equal,   Less)    =>        repeat(ny).zip((0..=nx).rev()).find(|&p| is_seat(p)),
                (Equal,   Greater) =>        repeat(ny).zip(nx..self.width).find(|&p| is_seat(p)),
                (Greater, Less)    => (ny..self.height).zip((0..=nx).rev()).find(|&p| is_seat(p)),
                (Greater, Equal)   => (ny..self.height).zip(     repeat(x)).find(|&p| is_seat(p)),
                (Greater, Greater) => (ny..self.height).zip(nx..self.width).find(|&p| is_seat(p)),
                _ => unreachable!(),
            });

        neighbour_positions
    }

    fn immidiate_neighbours_of<'s>(
        &'s self,
        (y, x): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + 's {
        let neighbour_offsets = (-1isize..=1)
            .cartesian_product(-1isize..=1)
            .filter(|&(y, x)| y != 0 || x != 0);

        neighbour_offsets
            .map(move |(dy, dx)| (y as isize + dy, x as isize + dx))
            .filter(move |&(ny, nx)| {
                0 <= nx && nx < self.width as isize && 0 <= ny && ny < self.height as isize
            })
            .map(|(ny, nx)| (ny as usize, nx as usize))
    }

    fn step(&mut self, proximity: Proximity) {
        let should_flip: Vec<_> = (0..self.height)
            .cartesian_product(0..self.width)
            .filter(|&(y, x)| {
                let tile = self[(y, x)];
                if tile == Tile::Floor {
                    return false;
                }

                let neighbours_occupied = match proximity {
                    Proximity::Immidiate => self
                        .immidiate_neighbours_of((y, x))
                        .map(|pos| self[pos])
                        .filter(|&tile| tile == Tile::Occupied)
                        .count(),
                    Proximity::Visible => self
                        .visible_neighbours_of((y, x))
                        .map(|pos| self[pos])
                        .filter(|&tile| tile == Tile::Occupied)
                        .count(),
                };

                match tile {
                    Tile::Vacant => neighbours_occupied == 0,
                    Tile::Occupied => neighbours_occupied >= proximity.occupied_threshold(),
                    Tile::Floor => unreachable!("exit early above to prevent counting"),
                }
            })
            .collect();

        for (y, x) in should_flip {
            let old_tile = self[(y as usize, x as usize)];
            let flipped_tile = match old_tile {
                Tile::Vacant => Tile::Occupied,
                Tile::Occupied => Tile::Vacant,
                Tile::Floor => unreachable!("should_flip flip filters away all floor coordinates"),
            };
            self[(y as usize, x as usize)] = flipped_tile;
        }
    }
}

impl FromStr for Seats {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .map(|line| line.len())
            .next()
            .context("empty input")?;

        let data: Vec<_> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| match c {
                'L' => Ok(Tile::Vacant),
                '.' => Ok(Tile::Floor),
                '#' => Ok(Tile::Occupied),
                _ => Err(anyhow!("expected 'L', '.' or '#', but got '{}'", c)),
            })
            .collect::<Result<_>>()
            .context("unable to tile map data")?;

        assert!(
            data.len() % width == 0,
            "data should always be multiple of width"
        );

        Ok(Seats {
            height: data.len() / width,
            width,
            data,
        })
    }
}

impl IndexMut<usize> for Seats {
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        &mut self.data[y * self.width..(y + 1) * self.width]
    }
}

impl Index<usize> for Seats {
    type Output = [Tile];

    fn index(&self, y: usize) -> &Self::Output {
        &self.data[y * self.width..(y + 1) * self.width]
    }
}

impl Index<(usize, usize)> for Seats {
    type Output = Tile;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Seats {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        &mut self.data[y * self.width + x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn part2_test() {
        assert_eq!(26, part2(&gen(EXAMPLE).unwrap()));
    }
}
