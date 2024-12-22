use core::str;
use std::collections::HashSet;
use std::io;
use std::fs;
use std::usize;

use crate::advent_of_code::Day;


pub struct Day19 { }

impl Day for Day19 {

    fn puzzle_1(mut input: io::Lines<io::BufReader<fs::File>>) -> String {

        // step 1, put the patterns into a hashset, and find the maximum pattern length
        let binding = input.next().unwrap().unwrap();
        let patterns_iter: std::str::Split<'_, &str> = binding.split(", ");

        let mut patterns: HashSet<&str> = HashSet::new();
        let mut max_pattern_len: usize = 0;
        for pattern in patterns_iter {
            if pattern.len() > max_pattern_len {
                max_pattern_len = pattern.len(); // can assume string is ascii
            }
            patterns.insert(pattern);
        }

        input.next();

        // step 2, do the algorithm
        // is_valid[i + n] = is_valid[i - 1] and substring [i..n] is valid pattern      
        let mut valid_designs: u64 = 0; 
        while let Some(design_result) = input.next() {

            let binding = design_result.unwrap_or_else(|_error| { panic!(); });
            let design = binding.as_bytes();

            let mut valid: Vec<bool> = vec![false; design.len() + 1];  
            // first element in array is base condition, an empty string is valid      
            // NOTE: because there's one extra element for empty string valid[i] is actually valid[i+1] 
            valid[0] = true;   

            for i in 0..design.len() {
                if !valid[i]  {
                    continue;
                }

                for j in i..i + max_pattern_len {
                    if j >= design.len() {
                        break;
                    }
                    let slice = str::from_utf8(&design[i..j + 1]).unwrap();
                    if patterns.contains(slice) {
                        valid[j + 1] = true;
                    }
                }
            }

            if valid[design.len()] {
                valid_designs += 1;
            }
        }

        dbg!(valid_designs);
        valid_designs.to_string()
    }

    fn puzzle_2(mut input: io::Lines<io::BufReader<fs::File>>) -> String {

        // step 1, put the patterns into a hashset, and find the maximum pattern length
        let binding = input.next().unwrap().unwrap();
        let patterns_iter: std::str::Split<'_, &str> = binding.split(", ");

        let mut patterns: HashSet<&str> = HashSet::new();
        let mut max_pattern_len: usize = 0;
        for pattern in patterns_iter {
            if pattern.len() > max_pattern_len {
                max_pattern_len = pattern.len(); // can assume string is ascii
            }
            patterns.insert(pattern);
        }
        
        input.next();

        // step 2: algorithm
        // num_ways[i] = num_ways[i-1] if substring i..i-1 exists in patterns + .. + num_ways[i-n] if substring i..i-n exists in patterns
        let mut total_ways: u64 = 0;
        while let Some(design_result) = input.next() {

            let binding = design_result.unwrap_or_else(|_error| { panic!(); });
            let design = binding.as_bytes();

            let mut num_ways: Vec<u64> = vec![0; design.len() + 1];        
            // first element in array is base condition, there's 1 way to make an empty string      
            // NOTE: because there's one extra element for empty string valid[i] is actually valid[i+1]
            num_ways[0] = 1;

            for i in 0..design.len() {
                let mut num_ways_i: u64 = 0;

                let lower_bound = match (i + 1).checked_sub(max_pattern_len) {
                    Some(x) => x,
                    None => 0
                };

                for j in (lower_bound..i + 1).rev() {
                    let slice = str::from_utf8(&design[j..i + 1]).unwrap();
                    if patterns.contains(slice) {
                        num_ways_i += num_ways[j];
                    }
                }
                num_ways[i + 1] = num_ways_i;
            }

            total_ways += num_ways[design.len()];
        }

        total_ways.to_string()
    }
}