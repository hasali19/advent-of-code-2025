use std::ops::{Index, IndexMut};

use aoc2025::aoc_solution;
use itertools::{Itertools, iproduct};

fn main() -> eyre::Result<()> {
    aoc_solution(4, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let grid = Grid::parse(input);

    let mut total = 0;

    for (x, y) in iproduct!(0..grid.width(), 0..grid.height()) {
        if grid[(x, y)] != '@' {
            continue;
        }

        let adjacent_rolls = iproduct!(-1..=1, -1..=1)
            .filter(|&v| v != (0, 0))
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|&(x, y)| grid.contains((x, y)) && grid[(x, y)] == '@')
            .count();

        if adjacent_rolls < 4 {
            total += 1;
        }
    }

    println!("{total}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let mut grid = Grid::parse(input);

    let mut total = 0;

    loop {
        let mut accessible = vec![];

        for (x, y) in iproduct!(0..grid.width(), 0..grid.height()) {
            if grid[(x, y)] != '@' {
                continue;
            }

            let adjacent_rolls = iproduct!(-1..=1, -1..=1)
                .filter(|&v| v != (0, 0))
                .map(|(dx, dy)| (x + dx, y + dy))
                .filter(|&(x, y)| grid.contains((x, y)) && grid[(x, y)] == '@')
                .count();

            if adjacent_rolls < 4 {
                accessible.push((x, y));
            }
        }

        if accessible.is_empty() {
            break;
        }

        total += accessible.len();

        for (x, y) in accessible {
            grid[(x, y)] = '.';
        }
    }

    println!("{total}");

    Ok(())
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn parse(input: &str) -> Grid {
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        Grid(grid)
    }

    fn width(&self) -> i32 {
        self.0[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.0.len() as i32
    }

    fn contains(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && x < self.width() && y >= 0 && y < self.height()
    }
}

impl Index<(i32, i32)> for Grid {
    type Output = char;

    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        &self.0[y as usize][x as usize]
    }
}

impl IndexMut<(i32, i32)> for Grid {
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        &mut self.0[y as usize][x as usize]
    }
}
