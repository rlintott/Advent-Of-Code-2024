use core::str;
use std::collections::HashSet;
use std::io;
use std::fs;
use std::usize;

use crate::advent_of_code::Day;


pub struct Day19 { }

impl Day for Day19 {

    fn puzzle_1(mut input: io::Lines<io::BufReader<fs::File>>) -> String {

        // step 1, put the patterns into a hashset, and find the longest pattern string length
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

        //dbg!(&patterns);

        input.next();
        // step 2, do the algorithm       
        let mut valid_designs: u64 = 0; 
        while let Some(design_result) = input.next() {

            let binding = design_result.unwrap_or_else(|_error| { panic!(); });
            let design = binding.as_bytes();

            let mut valid: Vec<bool> = vec![false; design.len() + 1];        
            valid[0] = true;   

            for i in 0..design.len() {
                //dbg!(i);
                if !valid[i]  {
                    //dbg!("continue");
                    continue;
                }

                for j in i..i + max_pattern_len {
                    if j >= design.len() {
                        //dbg!("break");
                        break;
                    }
                    let slice = str::from_utf8(&design[i..j + 1]).unwrap();
                    //dbg!(&slice);

                    if patterns.contains(slice) {
                        valid[j + 1] = true;
                    }
                    // else it's false                    
                }
                //dbg!(&valid);
            }
            //dbg!(str::from_utf8(design).unwrap());
            //dbg!(&valid);

            if valid[design.len()] {
                valid_designs += 1;
            }
        }

        dbg!(valid_designs);

        valid_designs.to_string()
    }

    fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>) -> String {
        "".to_string()
    }
}