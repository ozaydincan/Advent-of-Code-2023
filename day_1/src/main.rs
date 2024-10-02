use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
struct Decoder;

impl Decoder {
    fn part_1(infile: &String) -> Result<i32, std::io::Error> {
        let sum = infile
            .lines()
            .filter_map(|line| {
                let first_digit = line.chars().find(|c| c.is_digit(10))?;
                let last_digit = line.chars().rev().find(|c| c.is_digit(10))?;
                let first_digit = first_digit.to_digit(10)? as i32;
                let last_digit = last_digit.to_digit(10)? as i32;
                Some(first_digit * 10 + last_digit)
            })
            .sum();
        return Ok(sum);
    }
    fn part_2(infile: &String) -> Result<i32, std::io::Error> {
        let digit_map: HashMap<&str, usize> = [
            ("0", 0),
            ("zero", 0),
            ("1", 1),
            ("one", 1),
            ("2", 2),
            ("two", 2),
            ("3", 3),
            ("three", 3),
            ("4", 4),
            ("four", 4),
            ("5", 5),
            ("five", 5),
            ("6", 6),
            ("six", 6),
            ("7", 7),
            ("seven", 7),
            ("8", 8),
            ("eight", 8),
            ("9", 9),
            ("nine", 9),
        ]
        .into_iter()
        .collect();
        let sum: usize = infile
            .lines()
            .map(|line| {
                let (a, b) = digit_map
                    .iter()
                    .flat_map(|(key, &value)| {
                        line.match_indices(key).map(move |(idx, _)| (idx, value))
                    })
                    .minmax()
                    .into_option()
                    .unwrap();
                a.1 * 10 + b.1
            })
            .sum();
        return Ok(sum as i32);
    }
}

fn main() -> Result<(), std::io::Error> {
    let infile = read_to_string("src/input.txt")?;
    let sums_1 = Decoder::part_1(&infile)?;
    println!("Solution for part 1: {}", sums_1);
    let sums_2 = Decoder::part_2(&infile)?;
    println!("Solution for part 2: {}", sums_2);
    Ok(())
}
