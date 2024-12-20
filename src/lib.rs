
pub mod advent_of_code {
    use std::fs;
    use std::io;
    
    pub mod day_1;
    pub mod day_2;
    pub mod day_9;
    pub mod day_11;
    pub mod day_19;

    pub trait Day 
    {
        // TODO: aybe pass in just a bufreader instead of lines because some inputs are just one massive line...
        fn dispatch(puzzle: u32, input: io::Lines<io::BufReader<fs::File>>) -> String {
            match puzzle {
                1 => { return Self::puzzle_1(input); },
                2 => { return Self::puzzle_2(input); },
                _ => { eprint!("there are only two puzzles!"); std::process::exit(1); }
            }
        }
    
        fn puzzle_1(input: io::Lines<io::BufReader<fs::File>>) -> String;
        fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>) -> String;
    }
}


