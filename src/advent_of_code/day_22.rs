use std::io;
use std::fs;
use std::ops::BitXor;
use std::usize;

use crate::advent_of_code::Day;

pub struct Day22 { }

impl Day for Day22 {

    fn puzzle_1(input: io::Lines<io::BufReader<fs::File>>) -> String {
        let mut final_sum: u64 = 0;
        for line in input {
            // brute force baby!
            let number = line.unwrap().parse::<u64>().unwrap();
            let mut curr = number;
            for _i in 0..2000 {
                curr = get_next_secret_number(curr);
            }
            final_sum += curr;
        }
        dbg!(final_sum);
        final_sum.to_string()
    }
    

    fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>) -> String {
        // plus 19 to get it 
        let mut sequences_to_sum: Vec<(u32, u32)> = vec![(u32::max_value(), 0); 19usize.pow(4)];
        let mut hash: u32 = 0;
        let mut id: u32 = 0;

        for line in input {
            let number = line.unwrap().parse::<u64>().unwrap();
            let mut curr: u64 = number;
            let mut prev: u64;
            let mut shifts: usize = 0;
            for _i in 0..2000 {
                prev = curr;
                curr = get_next_secret_number(curr);
                
                let price =  curr % 10;                        
                let change: i8 = (price as i8 - (prev % 10) as i8) + 9;
                hash = (hash << 8) | (change as u32 & 0xff);
                shifts += 1;
                if shifts > 3 {
                    let index = ((hash & 0xFF000000) >> 24) * 19 * 19 * 19 +
                                        ((hash & 0x00FF0000) >> 16) * 19 * 19 +
                                        ((hash & 0x0000FF00) >> 8) * 19 +
                                        ((hash & 0x000000FF));                        

                    let entry = sequences_to_sum[index as usize];
                    if entry.0 != id { // overwrite, update curr and sum
                        sequences_to_sum[index as usize] = (id, price as u32 + entry.1);
                    }    
                }
            }
            id += 1;
            hash = 0;
        }

        let mut best_sequence_bananas: u32 = 0;
        for entry in sequences_to_sum {
            let max_bananas = entry.1;
            if max_bananas > best_sequence_bananas {
                //best_sequence = Some(entry.0);
                best_sequence_bananas = max_bananas;
            }
        }

        dbg!(best_sequence_bananas);
        best_sequence_bananas.to_string()
    }

}


fn mix(secret_number: u64, value: u64) -> u64 {
    secret_number.bitxor(value)
}

fn prune(secret_number: u64) -> u64 {
    secret_number % 16777216
}

fn get_next_secret_number(secret_number: u64) -> u64 {
    let mut result: u64 = prune(mix(secret_number, secret_number * 64));
    result = prune(mix(result, result / 32));
    result = prune(mix(result, result * 2048));
    result
}
