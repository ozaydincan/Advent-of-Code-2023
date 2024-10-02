// src/cards.rs
use std::collections::{HashMap, HashSet};

pub struct Cards;

impl Cards {
    pub fn card_vector(input: &str) -> Vec<i32> {
        let card_vec: Vec<i32> = input
            .lines()
            .map(|line| -> _ {
                let mut num_lists = line.split('|').map(|nums| {
                    nums.split_whitespace()
                        .filter_map(|num| num.parse::<i32>().ok())
                        .collect::<HashSet<_>>()
                });
                let left_list: HashSet<i32> = num_lists.next().unwrap();
                let right_list: HashSet<i32> = num_lists.next().unwrap();
                left_list.intersection(&right_list).count() as i32
            })
            .collect();
        card_vec
    }

    pub fn part_1(card_values: &[i32]) -> i32 {
        let winning_sum: i32 = card_values
            .iter()
            .map(|val| match val {
                0 => 0,
                _ => 1 << (val - 1),
            })
            .sum();

        winning_sum
    }

    pub fn part_2(card_values: &[i32]) -> i32 {
        let mut card_map: HashMap<i32, i32> = HashMap::new();
        for card_num in 1..=card_values.len() {
            let mut to_increment: Vec<i32> = Vec::from([card_num as i32]);
            while let Some(current_card) = to_increment.pop() {
                *card_map.entry(current_card).or_insert(0) += 1;
                let curr_id = (current_card - 1) as usize;
                let won_cards = card_values[curr_id];
                for i in (curr_id + 1)..=curr_id + (won_cards as usize) {
                    if i < card_values.len() {
                        to_increment.push((i + 1) as i32);
                    }
                }
            }
        }
        let card_count = card_map.values().sum();
        card_count
    }
}
