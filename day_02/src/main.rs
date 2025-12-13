#![feature(iterator_try_collect)]

use aoc2025::aoc_solution;
use eyre::OptionExt;
use odds::string::StrChunksWindows;

fn main() -> eyre::Result<()> {
    aoc_solution(2, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let ranges = input
        .split(',')
        .map(|range| range.trim().split_once('-'))
        .collect::<Option<Vec<_>>>()
        .ok_or_eyre("failed to parse ranges")?;

    let mut invalid_count = 0;

    for (a, b) in ranges {
        let a = a.parse::<u64>()?;
        let b = b.parse::<u64>()?;

        invalid_count += (a..=b)
            .filter(|id| {
                let id = id.to_string();

                if id.len() % 2 != 0 {
                    return false;
                }

                let (u, v) = id.split_at(id.len() / 2);

                u == v
            })
            .sum::<u64>();
    }

    println!("{invalid_count}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let ranges = input
        .split(',')
        .map(|range| range.trim().split_once('-'))
        .collect::<Option<Vec<_>>>()
        .ok_or_eyre("failed to parse ranges")?;

    let mut invalid_count = 0;

    for (a, b) in ranges {
        let a = a.parse::<u64>()?;
        let b = b.parse::<u64>()?;

        invalid_count += (a..=b)
            .filter(|id| {
                let id = id.to_string();

                let mut is_valid = true;

                for i in 1..=id.len() / 2 {
                    if id.len() % i != 0 {
                        continue;
                    }

                    let mut chunks = id.char_chunks(i);
                    let mut all_same = true;
                    let first = chunks.next().unwrap();
                    for chunk in chunks {
                        if chunk != first {
                            all_same = false;
                            break;
                        }
                    }

                    if all_same {
                        is_valid = false;
                        break;
                    }
                }

                !is_valid
            })
            .sum::<u64>();
    }

    println!("{invalid_count}");

    Ok(())
}
