use aoc2025::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(5, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let (fresh_ranges, ids) = input.split_once("\n\n").unwrap();

    let fresh_ranges = fresh_ranges
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            let a = a.parse::<u64>().unwrap();
            let b = b.parse::<u64>().unwrap();
            a..=b
        })
        .collect_vec();

    let ids = ids
        .lines()
        .map(|id| id.parse::<u64>().unwrap())
        .collect_vec();

    let count = ids
        .iter()
        .filter(|id| fresh_ranges.iter().any(|range| range.contains(id)))
        .count();

    println!("{count}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let (fresh_ranges, _) = input.split_once("\n\n").unwrap();

    let fresh_ranges = fresh_ranges
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            let a = a.parse::<u64>().unwrap();
            let b = b.parse::<u64>().unwrap();
            (a, b)
        })
        .collect_vec();

    let ranges = dedup_ranges(&fresh_ranges);
    let count = ranges.iter().map(|(a, b)| b - a + 1).sum::<u64>();

    println!("{count}");

    Ok(())
}

fn dedup_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut result = ranges.to_vec();

    loop {
        let mut deduped_ranges = vec![];

        for &(a, b) in &result {
            let mut did_combine = false;

            for range in &mut deduped_ranges {
                if let Some(combined) = combine_ranges(*range, (a, b)) {
                    *range = combined;
                    did_combine = true;
                    break;
                }
            }

            if !did_combine {
                deduped_ranges.push((a, b));
            }
        }

        if deduped_ranges.len() >= result.len() {
            break;
        }

        result = deduped_ranges;
    }

    result
}

fn combine_ranges(a: (u64, u64), b: (u64, u64)) -> Option<(u64, u64)> {
    // a entirely covers b
    if a.0 <= b.0 && a.1 >= b.1 {
        return Some(a);
    }

    // b entirely covers a
    if b.0 <= a.0 && b.1 >= a.1 {
        return Some(b);
    }

    // b partially overlaps end of a
    if b.0 >= a.0 && b.0 <= a.1 + 1 {
        return Some((a.0, b.1));
    }

    // b partially overlaps start of a
    if a.0 >= b.0 && a.0 <= b.1 + 1 {
        return Some((b.0, a.1));
    }

    None
}
