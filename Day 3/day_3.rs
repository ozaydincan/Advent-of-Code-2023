use std::error::Error;
use std::path::Path;
use std::fs;
use std::collections::{HashMap, HashSet};

struct SchematicDecoder;

impl SchematicDecoder{

    fn symbol_set(input: &String) -> HashSet<(i32,i32)>{
        let symbol_set = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)|{
                line.chars()
                    .enumerate()
                    .filter_map(move|(x, ch)|{
                        if !ch.is_digit(10) && ch != '.'{ 
                            Some((y as i32, x as i32))   
                        }else{
                            None
                        }
                    })
            }).collect();
        return symbol_set;
    }


    fn part_1(input: &String, symbol_positions: &HashSet<(i32,i32)>) -> i32{
        let part_sum: i32 = input 
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                let mut current_num = String::new();
                let mut results = Vec::new();

                for (x, c) in line.chars().enumerate() {
                    if c.is_digit(10) {
                        current_num.push(c);
                    } else if !current_num.is_empty() {
                        let num_value = current_num.parse::<i32>().unwrap();
                        results.push((y as i32, x as i32 - current_num.len() as i32, x as i32, num_value));
                        current_num.clear();
                    }
                }

                if !current_num.is_empty() {
                    let num_value = current_num.parse::<i32>().unwrap();
                    results.push((y as i32, line.len() as i32 - current_num.len() as i32, line.len() as i32, num_value));
                }

                results
            })
            .filter(|&(y, x0, x1, _)| {
                (y - 1..=y + 1)
                    .flat_map(move |ch_y| (x0 - 1..x1 + 1).map(move |ch_x| (ch_y, ch_x)))
                    .any(|p| symbol_positions.contains(&p))
            })
            .map(|(_, _, _, v)| v)
            .sum();
        return part_sum;

    }

    fn star_map(input: &str) -> HashMap<(i32, i32), Vec<i32>> {
        let star_map: HashMap<(i32, i32), Vec<i32>> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.match_indices('*')
                    .map(move |(x, _star)| ((y as i32, x as i32), Vec::new()))
            })
            .collect();
        return star_map;
    }

    fn part_2(input: &str, star_map: &mut HashMap<(i32, i32), Vec<i32>>) -> i32 {
        input.lines().enumerate().for_each(|(y, line)| {
            let y = y as i32;
            let mut current_num = String::new();
            let mut x_start = 0;

            line.chars().enumerate().for_each(|(x,c)| {
                if c.is_digit(10) {
                    if current_num.is_empty() {
                        x_start = x as i32;
                    }
                    current_num.push(c);
                } else if !current_num.is_empty() {
                    let num_value = current_num.parse::<i32>().unwrap();
                    let x_end = x as i32;
                    for p in (y - 1..=y + 1).flat_map(|ch_y| (x_start - 1..x_end + 1).map(move |ch_x| (ch_y, ch_x))) {
                        if let Some(vec) = star_map.get_mut(&p) {
                            vec.push(num_value);
                        }
                    }
                    current_num.clear();
                }
            });

            if !current_num.is_empty() {
                let num_value = current_num.parse::<i32>().unwrap();
                let x_end = line.len() as i32;
                for p in (y - 1..=y + 1).flat_map(|ch_y| (x_start - 1..x_end).map(move |ch_x| (ch_y, ch_x))) {
                    if let Some(vec) = star_map.get_mut(&p) {
                        vec.push(num_value);
                    }
                }
            }
        });

        let gear_sum: i32 = star_map
            .values()
            .filter(|vec| vec.len() == 2)
            .map(|vec| {vec.iter().copied().product::<i32>()})
            .sum();

        return gear_sum;


    }
}

    fn main() -> Result<(), Box <dyn Error>>{
        let path = Path::new("src/schematic.dat");
        let input:String = fs::read_to_string(&path)?;
        let symbol_idx = SchematicDecoder::symbol_set(&input);
        let part_sum = SchematicDecoder::part_1(&input, &symbol_idx);
        let mut star_idx = SchematicDecoder::star_map(&input); 
        let gear_sum = SchematicDecoder::part_2(&input, &mut star_idx);
        println!("Part 1:{part_sum}");
        println!("Part 2: {gear_sum}");
        Ok(())
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn works_1() {
            let schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string();
            let symb_id: HashSet<(i32, i32)> = SchematicDecoder::symbol_set(&schematic);
            assert_eq!(SchematicDecoder::part_1(&schematic, &symb_id), 4361);
                    }

    #[test]
    fn works_2(){
         let schematic = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string();

    let mut star_idx: HashMap<(i32,i32),Vec<i32>> =  SchematicDecoder::star_map(&schematic);
    let gear_sum:i32 = SchematicDecoder::part_2(&schematic, &mut star_idx);
    assert_eq!(467835, gear_sum);
}
    }
