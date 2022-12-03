use aoc_runner_derive::{aoc, aoc_generator};
//use std::num::ParseIntError;

enum Move {
    Rock,
    Paper,
    Scissors,
}

fn match_char(c: &str) -> Result<Move, ()> {
    match c {
        "A" | "X" => Ok(Move::Rock),
        "B" | "Y" => Ok(Move::Paper),
        "C" | "Z" => Ok(Move::Scissors),
        _ => Err(()),
    }
}

fn match_pair(pair: &str) -> Vec<Move> {
    pair.split(" ").map(|c| match_char(c).unwrap()).collect()
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<Vec<Move>> {
    input.lines().map(|l| match_pair(l)).collect()
}

#[aoc(day2, part1)]
fn day2_part1(move_pairs: &[Vec<Move>]) -> i32 {
    let mut iter = move_pairs.iter();
    let mut score: i32 = 0;
    while let Some(pair) = iter.next() {
        match pair[1] {
            // R1,P2,S3 + L0,D3,W6
            Move::Rock => match pair[0] {
                Move::Rock => score += 1 + 3,
                Move::Paper => score += 1 + 0,
                Move::Scissors => score += 1 + 6,
            },
            Move::Paper => match pair[0] {
                Move::Rock => score += 2 + 6,
                Move::Paper => score += 2 + 3,
                Move::Scissors => score += 2 + 0,
            },
            Move::Scissors => match pair[0] {
                Move::Rock => score += 3 + 0,
                Move::Paper => score += 3 + 6,
                Move::Scissors => score += 3 + 3,
            },
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        //assert_eq!(part1(&[1, 2, 3]), 2);
    }
}
