use std::collections::HashMap;
use std::collections::VecDeque;
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

        let mut sequences_to_sum: HashMap<(i8, i8, i8, i8), (usize, u64)>  = HashMap::new();

        let mut window: VecDeque<i8> = VecDeque::new();
        let mut id: usize = 0;
        for line in input {
            let number = line.unwrap().parse::<u64>().unwrap();
            let mut curr = number;
            let mut prev: Option<u64>;

            for _i in 0..2000 {
                prev = Some(curr);
                curr = get_next_secret_number(curr);

                if let Some(prev) = prev {
                    let change = (curr % 10) as i8 - (prev % 10) as i8;
                    window.push_back(change);
                    if window.len()  == 4 {

                        let price =  curr % 10;
                        let entry = sequences_to_sum.entry((window[0], window[1], window[2], window[3])).or_insert((id, price));
                        if (*entry).0 != id { // overwrite, update curr and sum
                            *entry = (id,  price + (*entry).1);
                        }
                        window.pop_front();
                    }
                }
            }
            id += 1;
            window.clear();
        }

        let mut best_sequence: Option<(i8, i8, i8, i8)> = None;
        let mut best_sequence_bananas: u64 = 0;
        for entry in sequences_to_sum {
            let max_bananas = entry.1.1;
            if max_bananas > best_sequence_bananas {
                best_sequence = Some(entry.0);
                best_sequence_bananas = max_bananas;
            }
        }

        dbg!(best_sequence_bananas);
        dbg!(best_sequence);
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