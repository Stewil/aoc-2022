use aoc_runner_derive::{aoc, aoc_generator};
//use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, &str> {
    if input.is_empty() {
        return Err("Input is empty");
    }
    let mut results: Vec<i32> = Vec::new();
    let mut lines = input.lines();
    results.push(0);
    while let Some(line) = lines.next() {
        if line == "" {
            results.push(0);
            continue;
        } else {
            let cal: i32 = line.parse().unwrap();
            *results.last_mut().unwrap() += cal;
        }
    }
    Ok(results)
}

#[aoc(day1, part1)]
fn part1(elves: &[i32]) -> i32 {
    match elves.iter().max() {
        Some(max) => return *max,
        None => return 0,
    }
}

#[aoc(day1, part2)]
fn part2(elves: &[i32]) -> i32 {
    let mut sorted = elves.to_vec();
    sorted.sort();
    // ..sorted.len() because the range is non-inclusive
    sorted[sorted.len() - 3..sorted.len()].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let e1: Elf = Elf {
            total_cals: 2400,
            cal_list: vec![600, 800, 1000],
        };
        let e2: Elf = Elf {
            total_cals: 3400,
            cal_list: vec![1600, 800, 1000],
        };
        assert_eq!(part1(&[e1, e2]), 3400);
    }
}
