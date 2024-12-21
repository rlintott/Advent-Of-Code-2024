use std::collections::HashMap;
use std::io;
use std::fs;

use crate::advent_of_code::Day;

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

struct SimpleMazeIterator<'a> {
    start_pos: (i32, i32),
    prev_pos: (i32, i32),
    end_pos: (i32, i32),
    maze: &'a Vec<Vec<i64>>
}

impl Iterator for SimpleMazeIterator<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {

        if self.start_pos == self.end_pos {
            return None;
        }

        let y_len: usize = self.maze.len();
        let x_len: usize = self.maze[0].len();

        for pos_data in (UDLRIterator { center: self.start_pos, 
                                                                    dist_from_center: 1, 
                                                                    index: 0, 
                                                                    x_bound: x_len as i32, 
                                                                    y_bound: y_len as i32 }) {                                                                
            let pos = pos_data.0;
            if pos == self.prev_pos {
                continue;
            }
            match self.maze[pos.1 as usize][pos.0 as usize] {
                -1 => (),
                _ => { self.prev_pos = self.start_pos; self.start_pos = pos; break }
            }

        }
        Some(self.start_pos)
    }
}

fn create_maze(input: &Vec<Result<String, io::Error>>) -> (Vec<Vec<i64>>, (i32, i32), (i32, i32)) {
    let mut x: i32;
    let mut y: i32 = 0;
    let mut start_pos: (i32, i32) = (0, 0);
    let mut end_pos: (i32, i32) = (0, 0);
    let mut maze: Vec<Vec<i64>> = Vec::new();

    for line_result in input {
        let mut maze_row: Vec<i64> = Vec::new();
        let line = line_result.as_ref().unwrap_or_else(|_err| { panic!(); });
        x = 0;
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

        let maze_1 = create_maze(&all_lines);
        let maze_2 = create_maze(&all_lines);

        let start_pos = maze_1.1;
        let end_pos = maze_1.2;
        let iter_maze = maze_1.0;
        let mut mut_maze = maze_2.0;

        let maze_iter = SimpleMazeIterator { maze: &iter_maze, prev_pos: start_pos, start_pos: start_pos, end_pos: end_pos };

        let mut picoseconds_travelled: i64 = 0; 
        let y_bound = iter_maze.len() as i32;
        let x_bound = iter_maze[0].len() as i32;
        let mut good_cheats: u64 = 0;

        for pos in maze_iter {
            picoseconds_travelled += 1;
            // TODO: have to index maze like [y][x], is confusing, find better way 
            mut_maze[pos.1 as usize][pos.0 as usize] = picoseconds_travelled;

            for peek_pos_data in (UDLRIterator { center: pos, 
                                                                            dist_from_center: 2, 
                                                                            index: 0, 
                                                                            x_bound: x_bound, 
                                                                            y_bound: y_bound }) {
                let peek_pos = peek_pos_data.0;
                let wall_check_pos = match peek_pos_data.1 {
                    Direction::UP => (peek_pos.0, peek_pos.1 - 1),
                    Direction::RIGHT => (peek_pos.0 - 1, peek_pos.1),
                    Direction::DOWN => (peek_pos.0, peek_pos.1 + 1),
                    Direction::LEFT => (peek_pos.0 + 1, peek_pos.1),
                };

                if (mut_maze[peek_pos.1 as usize][peek_pos.0 as usize] > 0 || peek_pos == start_pos) &&
                    mut_maze[wall_check_pos.1 as usize][wall_check_pos.0 as usize] == -1 {
                    // peek_pos is behind a wall and already visited, so is a valid cheat
                    let time_saved = picoseconds_travelled - mut_maze[peek_pos.1 as usize][peek_pos.0 as usize] - 2;
                    if time_saved >= 100 {
                        good_cheats += 1;
                    }
                }
            }

        }

        good_cheats.to_string()
    }


    
    
    fn puzzle_2(mut input: io::Lines<io::BufReader<fs::File>>) -> String {

        "".to_string()
    }

}


struct ManhattanDistanceIterator {
    center: (i32, i32),
    radius: i32,
    index: i32,
    sub_index: i32,
    x_bound: i32,
    y_bound: i32
}

impl Iterator for ManhattanDistanceIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: Option<(i32, i32)>;

        loop {
            if self.sub_index > 3 {
                self.sub_index = 0;
                self.index += 1;
                if self.index >= self.radius {
                    self.index = 0;
                    self.radius -= 1;
                }
            }
            if self.radius < 1 {
                return None;
            }    

            let offset = self.index;
            let inverse_offset = self.radius - self.index;

            // stole this elegant algorithm: https://stackoverflow.com/questions/75128474/how-to-generate-all-of-the-coordinates-that-are-within-a-manhattan-distance-r-of
            next = match self.index {
                0 => Some((self.center.0 + offset, self.center.1 + inverse_offset)),
                1 => Some((self.center.0 + inverse_offset, self.center.1 - offset)),
                2 => Some((self.center.0 - offset, self.center.1 - inverse_offset)),
                3 => Some((self.center.0 - inverse_offset, self.center.1 + offset)),
                _ => panic!() // not gonna happen!
            }; 

            self.sub_index += 1;
            if let Some(pos) = next {
                let in_bounds = !(pos.0 < 0 || pos.0 >= self.x_bound || pos.1 < 0 || pos.1 >= self.y_bound);
                if in_bounds {
                    break;
                }    
            }
        }

        next
    }
}
