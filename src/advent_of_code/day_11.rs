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

        let memoization: HashMap<u32, Vec<u32>> = HashMap::new();
        //let inputs = Vec<u32>
        fn get_stones(input: u32, blinks: u32, memo: &mut HashMap<u32, Vec<u32>>) -> Vec<u32> {
            // there can only be 2 stones
            let mut stone_1: Option<u32> = None;
            let mut stone_2: Option<u32> = None;

            let digits: usize = (DigitIterator { value: input }).count();
            if input == 0 {
                stone_1 = Some(1);
            } else if digits % 2 == 0 {
                // even number of digits
                let mut first_half: u32 = 0;
                let mut second_half: u32 = 0;
                let mut mult: u32 = 1;
                for digit in (DigitIterator { value: input }).take(digits / 2) {
                    // starting with the second half since it iteratates right to left
                    second_half += digit * mult;
                    mult *= 10;
                }
                mult = 1;
                for digit in (DigitIterator { value: input }).skip(digits / 2).take(digits / 2) {
                    first_half += digit * mult;
                    mult *= 10;
                }
                stone_1 = Some(first_half);
                stone_2 = Some(second_half);
                //let stones_1: Vec<u32> = get_stones(first_half, blinks - 1, memo);
                //let stones_2: Vec<u32> = get_stones(second_half, blinks - 1, memo);
            } else {
                stone_1 = Some(input * 2024);
            }


            fn update_memo_stones(input: u32, blinks: u32, memo: &mut HashMap<u32, Vec<u32>>) {
                if !memo.contains_key(&input) || memo[&input].len() < (blinks as usize) {
                    let stones: Vec<u32> = get_stones(input, blinks - 1, memo);
                    memo.entry(input)
                        .and_modify(|memo_stones| {
                            for i in memo_stones.len()..stones.len() {
                                memo_stones.push(stones[i]);
                            }
                    });        
                }
            }

            if let Some(stone_1) = stone_1 {
                update_memo_stones(stone_1, blinks, memo);
            }
            if let Some(stone_2) = stone_2 {
                update_memo_stones(stone_2, blinks, memo);
            }

            if !memo.contains_key(&input) {
                // amalgamate the results
            }  

            Vec::new()
        }

        "".to_string()
    }



    fn puzzle_2(mut input: io::Lines<io::BufReader<fs::File>>) -> String {
        "".to_string()
    }


}