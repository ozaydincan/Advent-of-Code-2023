struct Cubes;
use std::{collections::HashMap, fs::read_to_string};

impl Cubes{
    fn part_1(cube_map: &Vec<HashMap<&str, u32>>)-> Result<u32, std::io::Error>{
    
    const RED_LIMIT:u32 = 12;
    const GREEN_LIMIT:u32 = 13;
    const BLUE_LIMIT:u32 = 14;
    let sum_of_possible_game_ids: u32 = cube_map
        .iter()
        .enumerate()
        .filter_map(|(i, map)| {
            let is_possible = map.iter().all(|(color, &num)| {
                match *color {
                    "red" => num <= RED_LIMIT,
                    "green" => num <= GREEN_LIMIT,
                    "blue" => num <= BLUE_LIMIT,
                    _ => false,
                }
            });

            if is_possible {
                Some(i as u32 + 1) // Game IDs are 1-based
            } else {
                None
            }
        })
        .sum();

    return Ok(sum_of_possible_game_ids);
    }
    fn part_2 (cube_map: &Vec<HashMap<&str, u32>>) -> Result<u64, std::io::Error>{
        let sum:u32 = cube_map.iter().map(|row|{
        match (row.get("red"), row.get("blue"), row.get("green")) {
            (Some(&red), Some(&green), Some(&blue)) => red*green*blue,
            _ => 0,
        }
    }).sum();

        return Ok(sum as u64);
    }
}


fn main() -> Result<(), std::io::Error>{
    let infile = read_to_string("src/cubes.txt")?;
    let cube_map: Vec<HashMap<&str, u32>> = infile
    .lines()
    .map(|line| {
        let (_, entries) = line.split_once(':').expect("Failure parsing by ':' separator");
        entries
            .trim()
            .split(';')
            .flat_map(|entry| entry.trim().split(','))
            .map(|pair| {
                let (num, color) = pair.trim().split_once(' ').expect("Failure parsing by ';' separator");
                (color, num.parse::<u32>().unwrap())
            })
            .fold(HashMap::new(), |mut map, (color, num)| {
                map.entry(color)
                    .and_modify(|e: &mut u32| *e = (*e).max(num))
                    .or_insert(num);
                map
            })
    })
    .collect();
    let cube_sum:u32 = Cubes::part_1(&cube_map)?;
    println!("Sum of the cubes are: {cube_sum}");
    let power_sum = Cubes::part_2(&cube_map)?;
    println!("The power sum of the games are: {power_sum}");
    Ok(())
}
