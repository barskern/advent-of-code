use aoc_runner_derive::*;

#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn crashes(map: &[Vec<bool>], x_slope: usize, y_slope: usize) -> usize {
    let ys = (0..map.len()).step_by(y_slope);
    let xs = (0..map[0].len()).cycle().step_by(x_slope);

    ys.zip(xs).filter(|&(y, x)| map[y][x]).count()
}

#[aoc(day3, part1)]
pub fn part1(map: &[Vec<bool>]) -> usize {
    crashes(map, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(map: &[Vec<bool>]) -> usize {
    crashes(map, 1, 1)
        * crashes(map, 3, 1)
        * crashes(map, 5, 1)
        * crashes(map, 7, 1)
        * crashes(map, 1, 2)
}

#[cfg(test)]
mod tests {}
