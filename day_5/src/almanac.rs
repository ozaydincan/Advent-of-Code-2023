use itertools::Itertools;
use std::str::Split;

pub struct Almanac;

impl Almanac {
    pub fn seed_map(input: &str) -> Vec<i64> {
        let seeds: Vec<i64> = input
            .strip_prefix("seeds: ")
            .unwrap()
            .split_ascii_whitespace()
            .filter_map(|seed| seed.parse::<i64>().ok())
            .collect();
        seeds
    }

    pub fn get_range_seed(first_map: &Vec<i64>) -> Vec<i64> {
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

    pub fn instructions_vec(instructions: Split<&str>) -> Vec<Vec<(i64, i64, i64)>> {
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

    pub fn update_maps(mappings_groups: &Vec<Vec<(i64, i64, i64)>>, seeds: &mut [i64]) {
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
    pub fn get_min_location(seed_map: &Vec<i64>) -> i64 {
        let min_location = seed_map.iter().min().unwrap();
        *min_location
    }
}
