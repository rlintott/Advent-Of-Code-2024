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

        let input_string = input.next().unwrap().unwrap();
        // this is ok because it's ascii
        //let disk_map: &[u8] = input_string.as_bytes_mut();
        let mut disk_map: Vec<u32> = input_string.bytes().map(|b| {
            (b as char).to_digit(10).unwrap()
        })
        .collect();

        let mut start_pointer: usize = 0; // want the index of the free free space
        let mut end_pointer: usize = disk_map.len() - 1;
        if disk_map.len() % 2 == 0 { // want the index of a file not free space
            end_pointer = disk_map.len() - 2;
        } 
        let mut files: Vec<File> = Vec::new();

        let mut completed_files: collections::HashSet<usize> = std::collections::HashSet::new();

        while start_pointer < end_pointer {

            let start_id = start_pointer / 2;
            let start_blocks = disk_map[start_pointer];

            if completed_files.contains(&start_id) == false {
                files.push(File { id: start_id, blocks: start_blocks });
                completed_files.insert(start_id);
            }

            let free_space: u32 = disk_map[start_pointer + 1];
            let end_id = end_pointer / 2;
            let end_blocks = disk_map[end_pointer];
            // push as much as possible of file blocks into the free space
            files.push(File { id: end_id, blocks: std::cmp::min(free_space, end_blocks) });

            // update the blocks left for end file
            if end_blocks >= free_space {
                disk_map[end_pointer] = end_blocks - free_space;
            }
            else {
                disk_map[end_pointer] = 0;
            }

            dbg!(start_pointer);
            dbg!(end_pointer);

            if free_space >= end_blocks { // finished compacting this file, move to next
                end_pointer -= 2;
            }
            else { // move to next free space
                start_pointer += 2;
            }

        }

        let mut checksum: u32 = 0;

        for (index, file) in files.iter().enumerate() {
            checksum += file.blocks * (file.id as u32);
        }

        dbg!(files);
        dbg!(checksum);

        format!("")
    }


    fn puzzle_2(input: io::Lines<io::BufReader<fs::File>>) -> String {
        format!("")
    }
}
