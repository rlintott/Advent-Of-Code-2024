use std::fs;
use std::io;

pub mod day_1;
mod util;

pub trait Day 
{
    fn dispatch(puzzle: u32, input: io::Lines<io::BufReader<fs::File>>) {
        match puzzle {
            1 => { Self::puzzle_1(input); },
            2 => { Self::puzzle_2(input); },
            _ => { eprint!("there are only two puzzles!"); std::process::exit(1); }
        }
    }

    fn puzzle_1(input: io::Lines<io::BufReader<fs::File>>);
    fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>);
}

