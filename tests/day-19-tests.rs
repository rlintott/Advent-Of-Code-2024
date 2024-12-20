use std::io::BufRead;
use aoc_2024::advent_of_code::Day;

use common::read_file;

extern crate aoc_2024;

mod common;

#[test]
fn test_1_puzzle_1() {
    let buf = read_file(&"inputs/day-19.txt".to_string());
    let result = aoc_2024::advent_of_code::day_19::Day19::dispatch(1, buf.lines());
    assert_eq!(result, "216");
}


#[test]
fn test_2_puzzle_2() {
    let buf = read_file(&"inputs/day-19.txt".to_string());
    let result = aoc_2024::advent_of_code::day_19::Day19::dispatch(2, buf.lines());
    assert_eq!(result, "603191454138773");
}

