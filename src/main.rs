use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use crate::advent_of_code::Day;
mod advent_of_code;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let mut day: u32 = 0;
    let mut puzzle: u32 = 0;
    let mut file_name: Option<String> = None;

    // get the command line arguments as an iterable
    let mut iter = args.iter();
    iter.next(); // skip the first arg because it's not command line input

    while let Some(arg) = iter.next() {
        let next_arg_iter: Option<&String> = match iter.next() {
            Some(x   ) => Some(x),
            None => None
        };

        fn handle_parse_int_error(string: &String) -> u32 {
            eprintln!("Failed to parse {string} into u32");
            std::process::exit(1);
        }
        
        let next_arg = next_arg_iter.unwrap();
        if arg == "day" && next_arg_iter.is_some() {
            day = next_arg.parse::<u32>()
                .unwrap_or_else(|_error| { handle_parse_int_error(&next_arg) });
        }
        else if arg == "puzzle" && next_arg_iter.is_some() {
            puzzle = next_arg.parse::<u32>()
                .unwrap_or_else(|_error| { handle_parse_int_error(&next_arg) });
        }
        else if arg == "input" && next_arg_iter.is_some() {
            file_name = Some(next_arg.to_string());
        }
    }

    if day == 0 || puzzle == 0 || file_name.is_none() {
        eprintln!("Invalid input");
        std::process::exit(1);    

    }

    let file: fs::File = fs::File::open(file_name.as_ref().unwrap())
                .unwrap_or_else(|_error: io::Error| { 
                    eprintln!("Failed to read file {name}", name = file_name.as_ref().unwrap());
                    std::process::exit(1);            
                });

    let input = io::BufReader::new(file).lines();

    match day {
        1 => { advent_of_code::day_1::Day1::dispatch(puzzle, input); }
        // 2 => { advent_of_code::day_2::Day2::dispatch(puzzle, &input); }
        // 3 => { Day3::dispatch(puzzle, &input); }
        // 4 => { Day4::dispatch(puzzle, &input); }
        // 5 => { Day5::dispatch(puzzle, &input); }
        // 6 => { Day6::dispatch(puzzle, &input); }
        // 7 => { Day7::dispatch(puzzle, &input); }
        // 8 => { Day8::dispatch(puzzle, &input); }
        // 9 => { Day9::dispatch(puzzle, &input); }
        // 10 => { Day10::dispatch(puzzle, &input); }
        // 11 => { Day11::dispatch(puzzle, &input); }
        // 12 => { Day12::dispatch(puzzle, &input); }
        // 13 => { Day13::dispatch(puzzle, &input); }
        // 14 => { Day14::dispatch(puzzle, &input); }
        // 15 => { Day15::dispatch(puzzle, &input); }
        // 16 => { Day16::dispatch(puzzle, &input); }
        // 17 => { Day17::dispatch(puzzle, &input); }
        // 18 => { Day18::dispatch(puzzle, &input); }
        // 19 => { Day19::dispatch(puzzle, &input); }
        // 20 => { Day20::dispatch(puzzle, &input); }
        // 21 => { Day21::dispatch(puzzle, &input); }
        // 22 => { Day22::dispatch(puzzle, &input); }
        // 23 => { Day23::dispatch(puzzle, &input); }
        // 24 => { Day24::dispatch(puzzle, &input); }
        // 25 => { Day25::dispatch(puzzle, &input); }
        _ => { }
    }     
}

