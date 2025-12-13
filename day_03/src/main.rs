#![feature(coroutines, iter_from_coroutine)]

use std::iter;

use aoc2025::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(3, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let mut total = 0;

    for line in input.lines() {
        let values = line.chars().map(|v| v.to_digit(10).unwrap()).collect_vec();
        total += find_max_n(&values, 2).fold(0, |acc, v| acc * 10 + v);
    }

    println!("{total}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let mut total = 0;

    for line in input.lines() {
        let values = line.chars().map(|v| v.to_digit(10).unwrap()).collect_vec();
        total += find_max_n(&values, 12).fold(0, |acc, v| acc * 10 + v as u64);
    }

    println!("{total}");

    Ok(())
}

fn find_first_max(values: &[u32]) -> Option<(usize, u32)> {
    let mut max = None;

    for (i, &v) in values.iter().enumerate() {
        let Some((_, max_v)) = max else {
            max = Some((i, v));
            continue;
        };

        if v > max_v {
            max = Some((i, v));
        }
    }

    max
}

fn find_max_n(values: &[u32], n: u32) -> impl Iterator<Item = u32> {
    iter::from_coroutine(
        #[coroutine]
        move || {
            let mut lower_bound = 0;
            for i in 0..n {
                let upper_bound = values.len() - (n - i) as usize;
                let (j, max) = find_first_max(&values[lower_bound..=upper_bound]).unwrap();
                lower_bound += j + 1;
                yield max;
            }
        },
    )
}
