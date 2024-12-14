use std::collections::HashSet;
use std::fs;
use std::io;
use std::collections::HashMap;

//use crate::advent_of_code;
use crate::advent_of_code::Day;

pub struct Day11 { }

struct DigitIterator {
    value: u32
}

impl Iterator for DigitIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value <= 0 {
            return None;
        }
        let digit = self.value % 10;
        self.value /= 10;
        Some(digit)
    }

}

impl Day for Day11 {

    fn puzzle_1(mut input: io::Lines<io::BufReader<fs::File>>) -> String {

        fn get_stones_count(parent_stone: u32, blinks: u32, memo: &mut HashMap<u32, Vec<u32>>) {
            
            if blinks <= 0 {
                return;
            }
            if memo.contains_key(&parent_stone) && memo[&parent_stone].len() >= (blinks as usize) {
                //dbg!("hit cache 1!");
                return;
            }

            
            // there can only be 2 children stones
            let child_1: Option<u32>;
            let mut child_2: Option<u32> = None;

            let digits: usize = (DigitIterator { value: parent_stone }).count();
            if parent_stone == 0 {
                child_1 = Some(1);
            } else if digits % 2 == 0 {
                // even number of digits
                let mut digit_first_half: u32 = 0;
                let mut digit_second_half: u32 = 0;
                let mut mult: u32 = 1;
                for digit in (DigitIterator { value: parent_stone }).take(digits / 2) {
                    // starting with the second half since it iteratates right to left
                    digit_second_half += digit * mult;
                    mult *= 10;
                }
                mult = 1;
                for digit in (DigitIterator { value: parent_stone }).skip(digits / 2).take(digits / 2) {
                    digit_first_half += digit * mult;
                    mult *= 10;
                }
                child_1 = Some(digit_first_half);
                child_2 = Some(digit_second_half);
            } else {
                child_1 = Some(parent_stone * 2024);
            }

            //get_stones_count(parent_stone, blinks - 1, memo);
            fn update_memo_stones(stone: u32, blinks: u32, memo: &mut HashMap<u32, Vec<u32>>) {
                if !memo.contains_key(&stone) || memo[&stone].len() < (blinks as usize) {
                    get_stones_count(stone, blinks - 1, memo);
                    /*
                    
                    memo.entry(stone)
                        .and_modify(|memo_stones| {
                            for i in memo_stones.len()..stones.len() {
                                memo_stones.push(stones[i]);
                            }
                    });                          
                    
                     */
  
                } else {
                    //dbg!("hit cache 2!");
                }
            }

            //dbg!(blinks);
            //dbg!(child_1);
            //dbg!(child_2);

            if let Some(stone) = child_1 {
                update_memo_stones(stone, blinks, memo);
            }
            if let Some(stone) = child_2 {
                update_memo_stones(stone, blinks, memo);
            }

            // [ 1, 1, 2, 4, 4, 8]
            // the stones made after x itr = (child1 after x-1 itr) + (child2 after x-1 itr)
            let mut result: Vec<u32> = vec![0; blinks as usize];
            //dbg!(blinks);
            //dbg!(&memo[&child_1.unwrap()]);
            result[0] = 1;
            for i in 1..blinks {
                //println!("um here");
                //dbg!(i);
                //dbg!(result.len());
                //dbg!(memo[&child_1.unwrap()].len());
                result[i as usize] += memo[&child_1.unwrap()][(i - 1) as usize]
            }
            if let Some(child_2) = child_2 {
                for i in 1..blinks {
                    result[i as usize] += memo[&child_2][(i - 1) as usize]
                }    
            }
            (*memo).insert(parent_stone, result);
            //println!("returning");
            //result
        }

        let starting_stones: Vec<u32> = input.next().unwrap().unwrap()
                                            .split_ascii_whitespace()
                                            .map(|s| { s.parse::<u32>().unwrap() })
                                            .collect();
        let stone_memo_table: &mut HashMap<u32, Vec<u32>> = &mut HashMap::new(); 

        
        get_stones_count(starting_stones[0], 26,  stone_memo_table);
        get_stones_count(starting_stones[1], 26,  stone_memo_table);
        dbg!(&stone_memo_table[&125]);
        dbg!(&stone_memo_table[&17]);
        "".to_string()
    }



    fn puzzle_2(mut input: io::Lines<io::BufReader<fs::File>>) -> String {
        "".to_string()
    }


}