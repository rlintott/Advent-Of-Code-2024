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
                //dbg!("increasing");
                let mut safety: (bool, Option<usize>) = check_safety(x.iter(), true, increasing);
                //dbg!("decreasing");
                //let safety2: (bool, Option<usize>) = check_safety(x.iter(), true, decreasing);
                dbg!(x);
                /* 
                if safety.0 && safety2.0 {
                    dbg!("WHAT THE FUCK!");
                    dbg!(safety.1);
                    dbg!(safety2.1);

                    dbg!(x);
                    panic!();
                }
                */
                //dbg!(x);
                if safety.0 == false {
                    safety = check_safety(x.iter(), true, decreasing);
                }      
                if safety.0 == false {
                    dbg!(false);
                    dbg!("uh wut");
                    return false;
                }
                if let Some(skip_index) = safety.1 {
                    dbg!(x);
                    dbg!(skip_index);
                    let skipped = x.iter().enumerate().filter(|&(i, _)| i != skip_index).map(|(_, v)| v);
                    let result = check_safety(skipped, false, jumps_with_range).0;
                    dbg!(result);
                    dbg!("uh wut");
                    result
                }
                else {
                    let result = check_safety(x.iter(), true, jumps_with_range).0;
                    dbg!(result);
                    dbg!("uh wut");
                    result
                }
            })
            .count();

            println!("There are {safe_reports} safe reports! (allowing removal)");
    }
}

// TODO; get rid of condition parameter this only works with transitive stuff or whatever
fn check_safety<'a, F>(input: impl Iterator<Item = &'a i32>, allow_removal: bool, condition: F) -> (bool, Option<usize>) where 
F: Fn(i32, i32) -> bool {
    dbg!("Starting");
    let mut iter = input.enumerate().peekable();
    let mut prev = iter.next();
    let mut already_removed = false;
    let mut is_first_elem: bool = true;
    let mut index_removed: Option<usize> = None; 
    while let Some(mut curr) = iter.next() {
        dbg!(prev.unwrap());
        dbg!(curr);
        let prev_val: &i32 = prev.unwrap().1;
        let curr_val: &i32 = curr.1;
        if allow_removal == false && condition(*prev_val, *curr_val) == false {
            return (false, index_removed);
        }

        /*
            a b  c       
            7 10 8 10 11
         */
        //let skip_elem_invalid = iter.peek().is_some() && condition(*prev_val, *(iter.peek().unwrap()).1) == false;
        let ab_valid = condition(*prev_val, *curr_val);
        let ac_valid = iter.peek().is_none() || condition(*prev_val,  *(iter.peek().unwrap()).1);
        let bc_valid = iter.peek().is_none() || condition(*curr_val,  *(iter.peek().unwrap()).1);

        if bc_valid == false {
            if ac_valid {
                // have to delete b
                already_removed = true;
                index_removed = Some(curr.0);
                curr = iter.next().unwrap();
                continue;
            } else if ab_valid {
                // delete c which will happen in the next iteration
            }
        } 

        if ab_valid == false {
            if ac_valid {
                // delete b then
                already_removed = true;
                index_removed = Some(curr.0);
                curr = iter.next().unwrap();
            } else if bc_valid {
                // have to delete a 
                already_removed = true;
                index_removed = Some(prev.unwrap().0);
            }

        }
    
        if ac_valid == false {
            // c has to be deletedwhich will happen in next iteration
        }
        

        if condition(*prev_val, *curr_val) == false {

            //if skip_elem_invalid && 
            
            // about to fail
            if allow_removal 
                    && already_removed == false 
                    && iter.peek().is_some() 
                    && condition(*prev_val, *(iter.peek().unwrap()).1) {
                // if we havent already removed a level we remove this one
                already_removed = true;
                index_removed = Some(curr.0);
                curr = iter.next().unwrap();
                dbg!("removing");
            
            } else if allow_removal && is_first_elem {
                already_removed = true;
                index_removed = Some(prev.unwrap().0);
                dbg!(index_removed);
                dbg!("removing first elem");
            } else if allow_removal && already_removed == false && iter.peek().is_none() {
                dbg!("here");
                index_removed = Some(curr.0);
                return (true, index_removed);
            } else {
                dbg!("here");
                //dbg!(input);
                return (false, index_removed);
            }
        }
        prev = Some(curr);
        is_first_elem = false;
    }
    // 100, 3, 101, 102, 103
    // 100, 3, 4, 5, 7
    //dbg!("win");
    (true, index_removed)
}