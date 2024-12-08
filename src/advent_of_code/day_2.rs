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
    //dbg!(jump);
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
                (check_safety(x.iter(), false, increasing).0 || 
                check_safety(x.iter(), false, decreasing).0) &&
                check_safety(x.iter(), false, jumps_with_range).0
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
                let mut safety: (bool, Option<usize>) = check_safety(x.iter(), true, increasing);
                if safety.0 == false {
                    safety = check_safety(x.iter(), true, decreasing);
                }      
                if safety.0 == false {
                    return false;
                }
                if let Some(skip_index) = safety.1 {
                    dbg!(x);
                    dbg!(skip_index);
                    let result = check_safety(x.iter().skip(skip_index), false, jumps_with_range).0;
                    dbg!(result);
                    result
                }
                else {
                    check_safety(x.iter(), true, jumps_with_range).0
                }
            })
            .count();

            println!("There are {safe_reports} safe reports! (allowing removal)");
    }
}


fn check_safety<'a, F>(input: impl Iterator<Item = &'a i32>, allow_removal: bool, condition: F) -> (bool, Option<usize>) where 
F: Fn(i32, i32) -> bool {
    //dbg!(input);
    let mut iter = input.enumerate().peekable();
    let mut prev = iter.next();
    let mut already_removed = false;
    let mut is_first_elem: bool = true;
    let mut index_removed: Option<usize> = None; 
    while let Some(mut curr) = iter.next() {
        //dbg!(prev.unwrap());
        //dbg!(curr);
        let prev_val: &i32 = prev.unwrap().1;
        let curr_val: &i32 = curr.1;

        if condition(*prev_val, *curr_val) == false {

            // about to fail
            if allow_removal 
                    && already_removed == false 
                    && iter.peek().is_some() 
                    && condition(*prev_val, *(iter.peek().unwrap()).1) {
                // if we havent already removed a level we remove this one
                already_removed = true;
                curr = iter.next().unwrap();
                index_removed = Some(curr.0);
            } else if allow_removal && is_first_elem {
                already_removed = true;
                curr = iter.next().unwrap();
                index_removed = Some(curr.0);
            } else if allow_removal && iter.peek().is_none() {
                index_removed = Some(curr.0);
                return (true, index_removed);
            } else {
                //dbg!(false);
                //dbg!(input);
                return (false, index_removed);
            }
        }
        prev = Some(curr);
        is_first_elem = false;
    }
    // 100, 3, 101, 102, 103
    // 100, 3, 4, 5, 7
    (true, index_removed)
}