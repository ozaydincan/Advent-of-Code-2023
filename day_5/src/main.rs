use itertools::Itertools;
use std::{fs::read_to_string, path::Path, str::Split};
struct Almanac;

impl Almanac {
    fn seed_map(input: &str) -> Vec<i64> {
        let seeds: Vec<i64> = input
            .strip_prefix("seeds: ")
            .unwrap()
            .split_ascii_whitespace()
            .filter_map(|seed| seed.parse::<i64>().ok())
            .collect();
        seeds
    }

    fn get_range_seed(first_map: &Vec<i64>) -> Vec<i64> {
        let temp: Vec<i64> = first_map
            .chunks(2)
            .flat_map(|seeds| {
                let start = seeds[0];
                let range = seeds[1];
                (start..start + range).collect::<Vec<i64>>()
            })
            .collect();
        temp
    }

    fn instructions_vec(instructions: Split<&str>) -> Vec<Vec<(i64, i64, i64)>> {
        let instrucion_list = instructions
            .map(|group| {
                group
                    .lines()
                    .skip(1)
                    .map(|line| {
                        line.split_ascii_whitespace()
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect::<Vec<(i64, i64, i64)>>()
            })
            .collect::<Vec<Vec<(i64, i64, i64)>>>();
        instrucion_list
    }

    fn update_maps(mappings_groups: &Vec<Vec<(i64, i64, i64)>>, seeds: &mut [i64]) {
        for mappings in mappings_groups.iter() {
            for value in seeds.iter_mut() {
                if let Some((new_start, old_start, _range)) =
                    mappings.iter().find(|&&(_, old_start, range)| {
                        *value >= old_start && *value < old_start + range
                    })
                {
                    *value = *value - old_start + new_start;
                }
            }
        }
    }
    fn get_min_location(seed_map: &Vec<i64>) -> i64 {
        let min_location = seed_map.iter().min().unwrap();
        *min_location
    }
}
const SEEDMAP: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("src/almanac.txt");
    let infile = read_to_string(path)?;
    let mut instructions = infile.split("\n\n");
    let mut seed_map: Vec<i64> = Almanac::seed_map(instructions.next().unwrap());
    let mut range_seeds: Vec<i64> = Almanac::get_range_seed(&seed_map);
    let instruction_list = Almanac::instructions_vec(instructions);
    Almanac::update_maps(&instruction_list, &mut seed_map);
    Almanac::update_maps(&instruction_list, &mut range_seeds);
    for seed in &seed_map {
        println!("{:?}", seed)
    }

    let min_loc = Almanac::get_min_location(&seed_map);
    let new_min_loc = Almanac::get_min_location(&range_seeds);
    println!("The lowest location: {min_loc}");
    println!("The part 2 lowest location: {new_min_loc}");
    Ok(())
}

#[cfg(test)]

mod test {

    use super::*;
    use crate::SEEDMAP;

    #[test]

    fn part_1() {
        let infile = SEEDMAP;
        let mut instructions = infile.split("\n\n");
        let mut seed_map: Vec<i64> = Almanac::seed_map(instructions.next().unwrap());
        let mut new_map = Almanac::get_range_seed(&seed_map);
        let ins_list = Almanac::instructions_vec(instructions);
        Almanac::update_maps(&ins_list, &mut seed_map);
        let min_location = Almanac::get_min_location(&seed_map);
        println!("Full range of seeds: {:?}", new_map);
        Almanac::update_maps(&ins_list, &mut new_map);
        println!("Updated maps: {:?}", new_map);
        let new_loc = Almanac::get_min_location(&new_map);
        assert_eq!(35, min_location);
        assert_eq!(46, new_loc)
    }
}
