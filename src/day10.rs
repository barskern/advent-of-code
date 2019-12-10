#![allow(warnings)]

use aoc_runner_derive::*;
use nalgebra::{Point2, Vector2};
use std::cmp::{max, min};
use std::collections::{BTreeMap, HashSet};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[aoc_generator(day10)]
fn gen(input: &str) -> HashSet<Point2<isize>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| match c {
                '#' => Some(Point2::new(x as isize, y as isize)),
                _ => None,
            })
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(asteroids: &HashSet<Point2<isize>>) -> Result<usize> {
    asteroids
        .iter()
        .map(|&p| detect_count(p, asteroids))
        .max()
        .ok_or_else(|| "no asteroids provided".into())
}

#[aoc(day10, part2)]
pub fn part2(asteroids: &HashSet<Point2<isize>>) -> Result<usize> {
    let center = asteroids
        .iter()
        .max_by_key(|&&p| detect_count(p, asteroids));

    eprintln!("{:?}", center);

    unimplemented!()
}

fn detect_count(p: Point2<isize>, asteroids: &HashSet<Point2<isize>>) -> usize {
    asteroids
        .iter()
        // ensure that the currently check asteroid doesn't count itself
        .filter(|&&q| p != q)
        .filter(|&q| {
            let delta = q - p;
            if delta.x == 0 {
                (min(p.y, q.y)..max(p.y, q.y))
                    .map(|ny| Point2::new(p.x, ny))
                    // the first generated point is either `p` or `q`
                    .skip(1)
                    .all(|p| !asteroids.contains(&p))
            } else if delta.y == 0 {
                (min(p.x, q.x)..max(p.x, q.x))
                    .map(|nx| Point2::new(nx, p.y))
                    // the first generated point is either `p` or `q`
                    .skip(1)
                    .all(|p| !asteroids.contains(&p))
            } else {
                let gcd = gcd(delta.x, delta.y).abs();
                (1..gcd)
                    .map(|i| p + i * (delta / gcd))
                    .all(|p| !asteroids.contains(&p))
            }
        })
        .count()
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let input = concat!(".#..#\n", ".....\n", "#####\n", "....#\n", "...##",);

        assert_eq!(8, part1(&gen(input)).unwrap());
    }

    #[test]
    fn simple_example_part2() {
        let input = concat!(".#..#\n", ".....\n", "#####\n", "....#\n", "...##",);

        assert_eq!(8, part2(&gen(input)).unwrap());
    }

    #[test]
    fn medium_example() {
        let input = concat!(
            "......#.#.\n",
            "#..#.#....\n",
            "..#######.\n",
            ".#.#.###..\n",
            ".#..#.....\n",
            "..#....#.#\n",
            "#..#....#.\n",
            ".##.#..###\n",
            "##...#..#.\n",
            ".#....####",
        );

        assert_eq!(33, part1(&gen(input)).unwrap());
    }

    #[test]
    fn large_example() {
        let input = concat!(
            ".#..##.###...#######\n",
            "##.############..##.\n",
            ".#.######.########.#\n",
            ".###.#######.####.#.\n",
            "#####.##.#.##.###.##\n",
            "..#####..#.#########\n",
            "####################\n",
            "#.####....###.#.#.##\n",
            "##.#################\n",
            "#####.##.###..####..\n",
            "..######..##.#######\n",
            "####.##.####...##..#\n",
            ".#####..#.######.###\n",
            "##...#.##########...\n",
            "#.##########.#######\n",
            ".####.#.###.###.#.##\n",
            "....##.##.###..#####\n",
            ".#.#.###########.###\n",
            "#.#.#.#####.####.###\n",
            "###.##.####.##.#..##",
        );

        assert_eq!(210, part1(&gen(input)).unwrap());
    }
}
