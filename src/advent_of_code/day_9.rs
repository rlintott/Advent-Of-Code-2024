use std::fs;
use std::io;
use std::collections;

//use crate::advent_of_code;
use crate::advent_of_code::Day;

pub struct Day9 { }

#[derive(Debug)]
struct File {
    id: usize,
    blocks: u32
}

impl Day for Day9 {

    fn puzzle_1(mut input: io::Lines<io::BufReader<fs::File>>) -> String {

        let input_string: String = input.next().unwrap().unwrap();

        let mut disk_map: Vec<u32> = input_string.bytes().map(|b| {
            (b as char).to_digit(10).unwrap()
        }).collect();

        let mut left: usize = 0; 
        let mut right: usize = disk_map.len() - 1;
        if disk_map.len() % 2 == 0 { // files are only on even indices
            right = disk_map.len() - 2;
        } 

        let mut files: Vec<File> = Vec::new();
        let mut completed_files: collections::HashSet<usize> = std::collections::HashSet::new();

        while left < right {

            let left_id = left / 2;
            let left_blocks = disk_map[left];

            // check so that left files aren't duplicated in the list
            if completed_files.contains(&left_id) == false {
                files.push(File { id: left_id, blocks: left_blocks });
                completed_files.insert(left_id);
            }

            let left_free_space: u32 = disk_map[left + 1];
            let right_id = right / 2;
            let right_blocks = disk_map[right];

            if left_free_space == 0 {
                left += 2;
                if left == right && disk_map[right] > 0 {
                    // edge case, add the final right block to files list
                    files.push(File { id: right_id, blocks: disk_map[right] });
                }
                continue;
            }

            // push as much as possible of the right file's blocks into the free space
            files.push(File { id: right_id, blocks: std::cmp::min(left_free_space, right_blocks) });

            // update the blocks remaining and free space
            if right_blocks >= left_free_space {
                disk_map[right] = right_blocks - left_free_space;
                disk_map[left + 1] = 0;
            }
            else {
                disk_map[right] = 0;
                disk_map[left + 1] = left_free_space - right_blocks;
            }

            if disk_map[right] == 0 { // finished compacting file, advance right pointer left
                right -= 2;
            }
            else { // used up left pointer's free space, advance it right
                left += 2;
                if left == right && disk_map[right] > 0 {
                    // edge case, add the final right block to files list
                    files.push(File { id: right_id, blocks: disk_map[right] });
                }
            }

        }

        let mut checksum: u64 = 0;

        let mut pos: u32  = 0;
        for file in files {
            for _i in 0..file.blocks {
                checksum += (pos * (file.id as u32)) as u64;
                pos += 1;
            }
        }

        checksum.to_string()
    }


    fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>) -> String {
        /* 


        let input_string = input.next().unwrap().unwrap();

        let mut disk_map: Vec<u32> = input_string.bytes().map(|b| {
            (b as char).to_digit(10).unwrap()
        }).collect();

        let mut right: usize = disk_map.len() - 1;
        if disk_map.len() % 2 == 0 { // files are only on even indices
            right = disk_map.len() - 2;
        } 

        let mut files: Vec<File> = Vec::new();
        let mut completed_files: collections::HashSet<usize> = std::collections::HashSet::new();

        // TODO: this puzzle
                while right > 0 {

            for left in (0..right).step_by(2) {

                let left_id = left / 2;
                let left_blocks = disk_map[left];
    
                if completed_files.contains(&left_id) == false {
                    files.push(File { id: left_id, blocks: left_blocks });
                    completed_files.insert(left_id);
                }
    
                let left_free_space: u32 = disk_map[left + 1];
                let right_id = right / 2;
                let right_blocks = disk_map[right];
    
                if left_free_space == 0 {
                    /*
                    if left == right && disk_map[right] > 0 {
                        // edge case, add the final right block to files list
                        files.push(File { id: right_id, blocks: disk_map[right] });
                    }                    
                     */
                    continue;
                }
    
                //files.push(File { id: right_id, blocks: std::cmp::min(left_free_space, right_blocks) });
    
                if left_free_space >= right_blocks {
                    // the whole right block can fit in the empty space
                    disk_map[right] = 0;
                    disk_map[left + 1] = left_free_space - right_blocks;
                }
    
                if left_free_space >= right_blocks { // finished compacting file, advance left
                    right -= 2;
                }
                else { // no free space left, advance right
                    left += 2;
                    if left == right && disk_map[right] > 0 {
                        // edge case, add the final right block to files list
                        files.push(File { id: right_id, blocks: disk_map[right] });
                    }
                }
            }

            right -= 2;

        }
        
        
        */
        "".to_string()
    }
}
