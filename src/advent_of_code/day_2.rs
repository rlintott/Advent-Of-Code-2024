use std::io;
use std::fs;
use crate::advent_of_code;

pub struct Day2 {


 }

fn increasing(a: i32, b: i32) -> bool {
    a < b
}

fn decreasing(a: i32, b: i32) -> bool {
    a > b
}

fn jumps_with_range(a: i32, b: i32) -> bool {
    let jump = (b - a).abs();
    jump >= 1 && jump <= 3
}

impl advent_of_code::Day for Day2 {
    // https://adventofcode.com/2024/day/2

    fn puzzle_1(input: io::Lines<io::BufReader<fs::File>>) {
        let safe_reports = input.map(|line: Result<String, io::Error>| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(|number| { number.parse::<i32>().unwrap() })
                .collect()
            })
            .filter(|x: &Vec<i32>| { 
                (check_safety(x, false, increasing) || 
                check_safety(x, false, decreasing)) &&
                check_safety(x, false, jumps_with_range)
            })
            .count();

            println!("There are {safe_reports} safe reports!");
    }

    fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>) {
        let safe_reports = input.map(|line: Result<String, io::Error>| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(|number| { number.parse::<i32>().unwrap() })
                .collect()
            })
            .filter(|x: &Vec<i32>| { 
                (check_safety(x, true, increasing) || 
                check_safety(x, true, decreasing)) &&
                check_safety(x, true, jumps_with_range)
            })
            .count();

            println!("There are {safe_reports} safe reports! (allowing removal)");
    }
}

fn check_safety<F>(input: &[i32], allow_removal: bool, condition: F) -> bool where 
F: Fn(i32, i32) -> bool {
    let mut iter = input.iter().peekable();
    let mut prev = iter.next();
    let mut already_removed = false;
    dbg!(input);

    while let Some(mut curr) = iter.next() {

        if allow_removal && already_removed == false && iter.peek().is_none() {
            // edge case: last element
            return true
        }
        
        if prev.is_some() {
            if condition(*prev.unwrap(), *curr) == false {
                // about to fail
                if allow_removal 
                        && already_removed == false 
                        && iter.peek().is_some() 
                        && condition(*prev.unwrap(), **(iter.peek().unwrap())) {
                    // if we havent already removed a level we remove this one
                    already_removed = true;
                    curr = iter.next().unwrap();
                } else {
                    dbg!(false);
                    return false;
                }
            }
        } else if allow_removal 
                && prev.is_none() 
                && condition(*curr, **(iter.peek().unwrap())) == false {
            // edge case: if this is the first element and it is invalid, we need to skip it
            already_removed = true;
            curr = iter.next().unwrap();
        }
        prev = Some(curr);
    }
    // 100, 3, 101, 5, 7
    // 100, 3, 4, 5, 7

    dbg!(true);
    true
}