
pub mod advent_of_code {
    use std::fs;
    use std::io;
    
    pub mod day_1;
    pub mod day_2;
    pub mod day_9;
    pub mod day_11;
    pub mod day_19;
    pub mod day_20;
    pub mod day_21;
    pub mod day_22;
    pub mod day_23;

    // up down left right iterator! initialize with index 0. Skips positions that are out of bounds
    struct UDLRIterator {
        center: (i32, i32),
        dist_from_center: i32,
        index: u8,
        x_bound: i32,
        y_bound: i32
    }

    #[derive(Debug)]
    enum Direction {
        UP,
        DOWN,
        LEFT,
        RIGHT
    }

    impl Iterator for UDLRIterator {
        type Item = ((i32, i32), Direction);

        fn next(&mut self) -> Option<Self::Item> {
            let mut next: Option<((i32, i32), Direction)>;
            loop {
                next = match self.index {
                    0 => Some(((self.center.0, self.center.1 + self.dist_from_center), Direction::UP)),
                    1 => Some(((self.center.0 + self.dist_from_center, self.center.1), Direction::RIGHT)),
                    2 => Some(((self.center.0, self.center.1 - self.dist_from_center), Direction::DOWN)),
                    3 => Some(((self.center.0 - self.dist_from_center, self.center.1), Direction::LEFT)),
                    _ => None
                }; 
                let mut in_bounds = true;
                if let Some(ref pos_data) = next {
                    let pos = pos_data.0;
                    in_bounds = !(pos.0 < 0 || pos.0 >= self.x_bound || pos.1 < 0 || pos.1 >= self.y_bound);
                }

                if self.index > 3 || in_bounds {
                    self.index += 1;
                    break;
                }
                self.index += 1;
            } 
            next  
        }
    }


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


