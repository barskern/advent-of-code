use aoc_runner_derive::*;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> Result<usize> {
    (0..)
        .map(|i| (i * WIDTH * HEIGHT, (i + 1) * WIDTH * HEIGHT))
        .scan((), |_, (start, end)| input.get(start..end))
        .min_by_key(|layer| layer.matches('0').count())
        .map(|layer| layer.matches('1').count() * layer.matches('2').count())
        .ok_or_else(|| "no layers found".into())
}

// #[aoc(day8, part2)]
// pub fn part2(input: &str) -> Result<usize> {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {}
