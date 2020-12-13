use aoc_runner_derive::*;

use anyhow::{Context, Result};

const SEARCH_LIMIT: i64 = 100000;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BusId {
    Id(i64),
    X,
}

#[aoc_generator(day13)]
fn gen(input: &str) -> Result<(i64, Vec<BusId>)> {
    let mut lines = input.lines();
    Ok((
        lines
            .next()
            .context("empty input")
            .and_then(|line| line.parse().context("invalid early timestamp"))?,
        lines
            .next()
            .context("only one line in input")
            .and_then(|line| {
                line.split(',')
                    .map(|id_str| match id_str {
                        "x" => Ok(BusId::X),
                        id_str => id_str.parse().context("got invalid bus-id").map(BusId::Id),
                    })
                    .collect()
            })?,
    ))
}

#[aoc(day13, part1)]
pub fn part1((earliest_departure, bus_ids): &(i64, Vec<BusId>)) -> Result<i64> {
    let bus_ids = bus_ids.iter().filter_map(|id| match id {
        BusId::Id(id_) => Some(id_),
        BusId::X => None,
    });

    bus_ids
        .map(|bus_id| (bus_id, bus_id * ((earliest_departure / bus_id) + 1)))
        .map(|(bus_id, departure_time)| (bus_id, departure_time - earliest_departure))
        .min_by_key(|&(_, wait_time)| wait_time)
        .map(|(bus_id, wait_time)| wait_time * bus_id)
        .context("no buses given")
}

#[aoc(day13, part2)]
pub fn part2((_, bus_ids): &(i64, Vec<BusId>)) -> Result<i64> {
    let mut bus_data: Vec<_> = bus_ids
        .iter()
        .zip(0..)
        .filter_map(|(id, offset)| match id {
            BusId::Id(id_) => Some((*id_, offset)),
            BusId::X => None,
        })
        .collect();

    // start by finding lcm of the biggest numbers, this speeds up the algorithm substantially!
    bus_data.sort_unstable_by(|&(a, _), &(b, _)| b.cmp(&a));

    {
        let bus_ids = bus_data.iter().map(|&(x, _)| x);
        let max_id = bus_ids.clone().max().context("got zero bus-ids")?;

        let sieve = primal_sieve::Sieve::new(max_id as usize);
        debug_assert_eq!(
            true,
            bus_ids
                .clone()
                .all(|bus_id| sieve.is_prime(bus_id as usize))
        );
    }

    let ((first_bus_id, first_bus_offset), remaining) =
        bus_data.split_first().context("got zero bus-ids")?;

    std::iter::successors(
        Some((*first_bus_id, *first_bus_offset, 0)),
        |&(factor, offset, factor_count)| {
            let (bus_id, bus_offset) = remaining.get(factor_count)?;

            (0..SEARCH_LIMIT)
                .map(|x| factor * x - offset)
                .find(|search_for| (search_for + bus_offset) % bus_id == 0)
                .map(|search_for| (factor * bus_id, -search_for, factor_count + 1))
        },
    )
    .find(|&(_, _, factor_count)| factor_count == remaining.len())
    .map(|(_, offset, _)| -offset)
    .context("found no solution, try increasing SEARCH_LIMIT")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: &[(&str, i64)] = &[("0\n17,x,13,19", 3417), ("0\n67,7,59,61", 754018)];

    #[test]
    fn part2_test() {
        for (t, n) in EXAMPLES {
            assert_eq!(*n, part2(&gen(t).unwrap()).unwrap());
        }
    }
}
