use std::{fs::read_to_string, path::Path};
use std::collections::{HashMap, HashSet};

pub struct Cards;

impl Cards{
    fn card_vector(input: &str) -> Vec<i32>{
        let card_vec: Vec<i32> = input
            .lines()
            .map(|line| -> _ {
                let mut num_lists = line.split('|')
                    .map(|nums|{
                        nums.trim()
                            .split_whitespace()
                            .filter_map(|num|{ num.parse::<i32>().ok()})
                            .collect::<HashSet<_>>()
                    });
                let left_list: HashSet<i32> = num_lists.next().unwrap();
                let right_list: HashSet<i32> = num_lists.next().unwrap();
                left_list.intersection(&right_list).count() as i32
            }).collect();
        return card_vec;


    }

    fn part_1(card_values: &Vec<i32>)->i32{
        let winning_sum:i32 = card_values
            .iter()
            .map(|val|{
                match val{
                    0 => 0,
                    _ => 1<< (val -1),
                }
            }).sum();
                                        
    
        return winning_sum;
    }
    
    fn part_2(card_values: &Vec<i32>) -> i32{
        let mut card_map: HashMap<i32,i32> = HashMap::new();
        for card_num in 1 ..= card_values.len(){
            let mut to_increment:Vec<i32> = Vec::from([card_num as i32]);
            while let Some(current_card) = to_increment.pop(){
                *card_map.entry(current_card).or_insert(0) += 1;
                let curr_id = (current_card -1) as usize;
                let won_cards = card_values[curr_id];
                for i in (curr_id+1) ..= curr_id+(won_cards as usize){
                    if i < card_values.len(){
                        to_increment.push((i+1) as i32);
                    }
                }
            }
        }
        let card_count = card_map.iter().map(|(_key, val)|val).sum();
        return card_count;

    }
}




fn main() -> Result<(), std::io::Error>{
    let path = Path::new("src/cards.txt");
    let input = read_to_string(&path)?;
    let card_values:Vec<i32> = Cards::card_vector(input.as_str());
    let winning_count:i32 = Cards::part_1(&card_values);
    println!("Solution for part 1: {winning_count}");
    let card_count = Cards::part_2(&card_values);
    println!("Solution for part 2: {card_count}");
    return Ok(());
}



#[cfg(test)]
mod test{
    use super::*;
    const CARDLIST:&str= "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    #[test]
    fn _work_1(){
        let v:Vec<i32> = Cards::card_vector(&CARDLIST);
        assert_eq!(Cards::part_1(&v), 13) 
    }
    fn _work_2(){
        let v:Vec<i32> = Cards::card_vector(&CARDLIST);
        assert_eq!(Cards::part_2(&v), 30)
    }
}
