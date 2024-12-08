use std::io;
use std::fs;
use std::usize;
use crate::advent_of_code;
use std::collections::HashMap;

pub struct Day1 { }

impl advent_of_code::Day for Day1 {
    // https://adventofcode.com/2024/day/1

    fn puzzle_1(input: io::Lines<io::BufReader<fs::File>>) {

        let mut list_1: Vec<i32> = Vec::new();
        let mut list_2: Vec<i32> = Vec::new();
        for line in input {

            let line: String = line.unwrap();
            let mut numbers = line.split_ascii_whitespace();
            
            // insert first number into list 1
            if let Ok(number) = numbers.next().unwrap().parse::<i32>() {
                let insertion_idx: usize;
                match list_1.binary_search(&number) {
                    Ok(x) => insertion_idx = x,
                    Err(x) => insertion_idx = x
                }
                list_1.insert(insertion_idx, number);    
            } 

            // insert second number into list 2
            if let Ok(number) = numbers.next().unwrap().parse::<i32>() {
                let insertion_idx: usize;
                match list_2.binary_search(&number) {
                    Ok(x) => insertion_idx = x,
                    Err(x) => insertion_idx = x
                }
                list_2.insert(insertion_idx, number);    
            } 
        }

        let zipped = list_1.iter().zip(list_2.iter());

        // add up the distances
        let mut distance: i32 = 0;
        for (a, b) in zipped {
            distance += (a - b).abs();
        }

        println!("Distance is: {distance}");
    }


    fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>) {

        let mut col_1_counter: HashMap<i32, i32> = HashMap::new();
        let mut col_2_counter: HashMap<i32, i32> = HashMap::new();

        for line in input {

            let line: String = line.unwrap();
            let mut numbers = line.split_ascii_whitespace();

            if let Ok(number) = numbers.next().unwrap().parse::<i32>() {
                let freq = col_1_counter.entry(number).or_insert(0);
                *freq += 1;
            }

            if let Ok(number) = numbers.next().unwrap().parse::<i32>() {
                let freq = col_2_counter.entry(number).or_insert(0);
                *freq += 1;
            }
        }

        let mut total_similarity_score: i32 = 0;
        for (key, value) in col_1_counter {
            let mut similarity_score: i32 = 0;
            if col_2_counter.contains_key(&key) {
                similarity_score = col_2_counter[&key] * key;
            }
            total_similarity_score += similarity_score * value;
        }

        println!("Similary score is: {total_similarity_score}");
    }
}
