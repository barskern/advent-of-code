use aoc_runner_derive::*;
use nalgebra::{Point2, Vector2};
use std::iter;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> Result<isize> {
    let wires: Vec<_> = input.lines().map(parse_wire).collect();

    wires
        .iter()
        .enumerate()
        .flat_map(|(i, wire_a)| {
            wires
                .iter()
                .skip(i + 1)
                .flat_map(move |wire_b| intersections(wire_a, wire_b))
        })
        .filter(|&p| p != Point2::new(0, 0))
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .ok_or_else(|| "no intersections found".into())
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> Result<isize> {
    let wires: Vec<_> = input.lines().map(parse_wire).collect();

    wires
        .iter()
        .enumerate()
        .flat_map(|(i, wire_a)| {
            wires
                .iter()
                .enumerate()
                // Prevent self intersections
                .filter(move |(j, _)| i != *j)
                .flat_map(move |(_, wire_b)| intersections_with_distance(wire_a, wire_b))
        })
        .filter(|&(p, _, _)| p != Point2::new(0, 0))
        // .inspect(|x| eprintln!("{:?} {}", x, x.1 + x.2))
        .map(|(_, a, b)| a + b)
        .min()
        .ok_or_else(|| "no intersection found".into())
}

fn intersections_with_distance<'a>(
    wire_a: &'a [Point2<isize>],
    wire_b: &'a [Point2<isize>],
) -> impl Iterator<Item = (Point2<isize>, isize, isize)> + 'a {
    use std::cmp::{max, min};

    let mut dist_a = 0;

    wire_a.windows(2).flat_map(move |segment_a| {
        let (p1, p2) = (segment_a[0], segment_a[1]);
        let dp = p2 - p1;
        let da = dp.x.abs() + dp.y.abs();
        dist_a += da;
        let mut dist_b = 0;

        wire_b.windows(2).filter_map(move |segment_b| {
            let (q1, q2) = (segment_b[0], segment_b[1]);
            let dq = q2 - q1;
            let db = dq.x.abs() + dq.y.abs();
            dist_b += db;

            if dp.y == 0 && dq.x == 0 {
                // p horizontal and q vertical
                if (min(p1.x, p2.x) <= q1.x && q1.x <= max(p1.x, p2.x))
                    && (min(q1.y, q2.y) <= p1.y && p1.y <= max(q1.y, q2.y))
                {
                    return Some((
                        Point2::new(q1.x, p1.y),
                        dist_a - (p2.x - q1.x).abs(),
                        dist_b - (q2.y - p1.y).abs(),
                    ));
                }
            }
            if dp.x == 0 && dq.y == 0 {
                // p vertical and q horizontal
                if (min(q1.x, q2.x) <= p1.x && p1.x <= max(q1.x, q2.x))
                    && (min(p1.y, p2.y) <= q1.y && q1.y <= max(p1.y, p2.y))
                {
                    return Some((
                        Point2::new(p1.x, q1.y),
                        dist_a - (q1.y - p2.y).abs(),
                        dist_b - (p1.x - q2.x).abs(),
                    ));
                }
            }

            None
        })
    })
}

fn intersections<'a>(
    wire_a: &'a [Point2<isize>],
    wire_b: &'a [Point2<isize>],
) -> impl Iterator<Item = Point2<isize>> + 'a {
    use std::cmp::{max, min};

    wire_a.windows(2).flat_map(move |segment_a| {
        let (p1, p2) = (segment_a[0], segment_a[1]);
        let dp = p2 - p1;

        wire_b.windows(2).filter_map(move |segment_b| {
            let (q1, q2) = (segment_b[0], segment_b[1]);
            let dq = q2 - q1;

            if dp.y == 0 && dq.x == 0 {
                // p horizontal and q vertical
                if (min(p1.x, p2.x) <= q1.x && q1.x <= max(p1.x, p2.x))
                    && (min(q1.y, q2.y) <= p1.y && p1.y <= max(q1.y, q2.y))
                {
                    return Some(Point2::new(q1.x, p1.y));
                }
            }
            if dp.x == 0 && dq.y == 0 {
                // p vertical and q horizontal
                if (min(q1.x, q2.x) <= p1.x && p1.x <= max(q1.x, q2.x))
                    && (min(p1.y, p2.y) <= q1.y && q1.y <= max(p1.y, p2.y))
                {
                    return Some(Point2::new(p1.x, q1.y));
                }
            }

            None
        })
    })
}

fn parse_movement(s: &str) -> Vector2<isize> {
    let dir = match &s[..1] {
        "R" => Vector2::x(),
        "U" => Vector2::y(),
        "L" => -Vector2::x(),
        "D" => -Vector2::y(),
        _ => panic!("invalid direction"),
    };
    let len = s[1..].parse::<isize>().unwrap();

    dir * len
}

fn parse_wire(s: &str) -> Vec<Point2<isize>> {
    let start = Point2::new(0, 0);
    iter::once(start)
        .chain(
            s.split(',')
                .map(parse_movement)
                .scan(start, |offset, movement| {
                    let vertex = *offset + movement;
                    *offset += movement;
                    Some(vertex)
                }),
        )
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = concat!(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\n",
            "U62,R66,U55,R34,D71,R55,D58,R83"
        );

        assert_eq!(159, part1(input).unwrap());
    }

    #[test]
    fn example_two() {
        let input = concat!(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        );

        assert_eq!(135, part1(input).unwrap());
    }

    #[test]
    fn example_three() {
        let input = concat!(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\n",
            "U62,R66,U55,R34,D71,R55,D58,R83"
        );

        assert_eq!(610, part2(input).unwrap());
    }

    #[test]
    fn example_four() {
        let input = concat!(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        );

        assert_eq!(410, part2(input).unwrap());
    }

    #[test]
    fn simple_example() {
        let input = concat!("R8,U5,L5,D3\n", "U7,R6,D4,L4");

        assert_eq!(30, part2(input).unwrap());
    }
}
