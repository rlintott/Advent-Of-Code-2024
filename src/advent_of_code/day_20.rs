use std::io;
use std::fs;
use std::path::PrefixComponent;

use crate::advent_of_code::Day;

// up down left right iterator! initialize with index 0
struct UDLRIterator {
    center: (i32, i32),
    dist_from_center: i32,
    index: u8
}

impl Iterator for UDLRIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.index {
            0 => Some((self.center.0, self.center.1 + self.dist_from_center)),
            1 => Some((self.center.0 + self.dist_from_center, self.center.1)),
            2 => Some((self.center.0, self.center.1 - self.dist_from_center)),
            3 => Some((self.center.0 - self.dist_from_center, self.center.1)),
            _ => None
        };      
        self.index += 1;
        next  
    }
}

struct SimpleMazeIterator<'a> {
    start_pos: (i32, i32),
    prev_pos: (i32, i32),
    maze: &'a Vec<Vec<i64>>
}

impl Iterator for SimpleMazeIterator<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {

        let y_len: usize = self.maze.len();
        let x_len: usize = self.maze[0].len();

        for pos in (UDLRIterator { center: self.start_pos, dist_from_center: 1, index: 0  }) {
            if pos.0 < 0 || pos.0 >= x_len as i32 || pos.1 < 0 || pos.1 >= y_len as i32 {
                continue;
            }
            if pos == self.prev_pos {
                continue;
            }
            match self.maze[pos.0 as usize][pos.1 as usize] {
                -1 => (),
                _ => { self.prev_pos = self.start_pos; self.start_pos = pos }
            }

        }
        if self.prev_pos != self.start_pos {
            Some(self.start_pos)
        }
        else { // reached end of maze
            None
        }
    }
}

fn generate_maze(input: &Vec<Result<String, io::Error>>) -> (Vec<Vec<i64>>, (i32, i32), (i32, i32)) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut start_pos: (i32, i32) = (0, 0);
    let mut end_pos: (i32, i32) = (0, 0);
    let mut maze: Vec<Vec<i64>> = Vec::new();
    for line_result in input {
        let mut maze_row: Vec<i64> = Vec::new();
        let line = line_result.as_ref().unwrap_or_else(|_err| { panic!(); });

        for char in line.chars() {
            match char {
                '#' => maze_row.push(-1),
                '.' => maze_row.push(0),
                'S' => { start_pos = (x as i32, y as i32); maze_row.push(0) }, 
                'E' => { end_pos = (x as i32, y as i32); maze_row.push(0) },
                _ => panic!() // not gonna happen! I trust advent of code!
            }
            x += 1;
        } 
        y += 1;
        maze.push(maze_row);
    }
    (maze, start_pos, end_pos)
}


pub struct Day20 { }

impl Day for Day20 {

    fn puzzle_1(input: io::Lines<io::BufReader<fs::File>>) -> String {
        
        let all_lines: Vec<Result<String, io::Error>> = input.collect();

        let maze_1 = generate_maze(&all_lines);
        let maze_2 = generate_maze(&all_lines);

        let start_pos = maze_1.1;
        let end_pos = maze_1.2;
        let iter_maze = maze_1.0;
        let mut mut_maze = maze_2.0;

        let maze_iter = SimpleMazeIterator { maze: &iter_maze, prev_pos: start_pos, start_pos: start_pos };

        let mut picoseconds_travelled: i64 = 0; 
        for pos in maze_iter {
            picoseconds_travelled += 1;
            mut_maze[pos.0 as usize][pos.1 as usize] = picoseconds_travelled;
            
        }

        "".to_string()
    }

    
    fn puzzle_2(mut input: io::Lines<io::BufReader<fs::File>>) -> String {

        "".to_string()
    }

}