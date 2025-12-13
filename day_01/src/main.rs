use aoc2025::aoc_solution;

fn main() -> eyre::Result<()> {
    aoc_solution(1, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let mut current = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let (d, n) = line.split_at(1);
        let n = n.parse::<i32>()?;

        match d {
            "L" => current -= n,
            "R" => current += n,
            _ => unreachable!(),
        }

        current = current.rem_euclid(100);

        if current == 0 {
            zero_count += 1;
        }
    }

    println!("{zero_count}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let mut current = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let (d, n) = line.split_at(1);
        let n = n.parse::<i32>()?;

        zero_count += n / 100;

        let n = n % 100;

        if current != 0 && (d == "L" && n >= current || d == "R" && n >= (100 - current)) {
            zero_count += 1;
        }

        match d {
            "L" => current -= n,
            "R" => current += n,
            _ => unreachable!(),
        }

        current = current.rem_euclid(100);
    }

    println!("{zero_count}");

    Ok(())
}
