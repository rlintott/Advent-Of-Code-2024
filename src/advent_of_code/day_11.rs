use std::fs;
use std::io;
use std::collections::HashMap;

//use crate::advent_of_code;
use crate::advent_of_code::Day;

pub struct Day11 { }

struct DigitIterator {
    value: u64
}

impl Iterator for DigitIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value <= 0 {
            return None;
        }
        let digit = self.value % 10;
        self.value /= 10;
        Some(digit)
    }

}

fn get_stones_count(parent_stone: u64, blinks: u64, memo: &mut HashMap<u64, Vec<u64>>) {
            
    if blinks <= 0 {
        return;
    }
    if memo.contains_key(&parent_stone) && memo[&parent_stone].len() >= (blinks as usize) {
        //dbg!("hit cache 1!");
        return;
    }

    // there can only be 2 children stones
    let child_1: Option<u64>;
    let mut child_2: Option<u64> = None;

    let digits: usize = (DigitIterator { value: parent_stone }).count();
    if parent_stone == 0 {
        child_1 = Some(1);
    } else if digits % 2 == 0 {
        // even number of digits
        let mut digit_first_half: u64 = 0;
        let mut digit_second_half: u64 = 0;
        let mut mult: u64 = 1;
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

    fn update_memo_stones(stone: u64, blinks: u64, memo: &mut HashMap<u64, Vec<u64>>) {
        if !memo.contains_key(&stone) || memo[&stone].len() < (blinks as usize) {
            get_stones_count(stone, blinks - 1, memo);
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

    // parent stones after x itr = (child1 stones after x-1 itr) + (child2 stones after x-1 itr)
    let mut result: Vec<u64> = vec![0; blinks as usize];
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
}


impl Day for Day11 {
    fn puzzle_1(mut input: io::Lines<io::BufReader<fs::File>>) -> String {

        let starting_stones: Vec<u64> = input.next().unwrap().unwrap()
                                            .split_ascii_whitespace()
                                            .map(|s| { s.parse::<u64>().unwrap() })
                                            .collect();
        let stone_memo_table: &mut HashMap<u64, Vec<u64>> = &mut HashMap::new(); 

        let mut result: u64 = 0;
        for stone in starting_stones {
            // TODO: there's an off by one error requiring us to pass in blinks + 1
            get_stones_count(stone, 26,  stone_memo_table);
            result += stone_memo_table[&stone].last().unwrap();
        }
        dbg!(result);
        result.to_string()
    }


    // exact same code for part 2, yay for memoization!
    fn puzzle_2(mut input: io::Lines<io::BufReader<fs::File>>) -> String {
        let starting_stones: Vec<u64> = input.next().unwrap().unwrap()
                                            .split_ascii_whitespace()
                                            .map(|s| { s.parse::<u64>().unwrap() })
                                            .collect();
        let stone_memo_table: &mut HashMap<u64, Vec<u64>> = &mut HashMap::new(); 

        let mut result: u64 = 0;
        for stone in starting_stones {
            // TODO: there's an off by one error requiring us to pass in blinks + 1
            get_stones_count(stone, 76,  stone_memo_table);
            result += stone_memo_table[&stone].last().unwrap();
        }
        dbg!(result);
        result.to_string()    }


}