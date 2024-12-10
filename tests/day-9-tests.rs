use std::io::BufRead;
use aoc_2024::advent_of_code::Day;

use common::read_file;

extern crate aoc_2024;

mod common;

#[test]
fn test_1_puzzle_1() {
    let buf = read_file(&"tests/day-9/1.txt".to_string());
    let result = aoc_2024::advent_of_code::day_9::Day9::dispatch(1, buf.lines());
    assert_eq!(result, "60");
}

#[test]
fn test_2_puzzle_1() {
    let buf = read_file(&"tests/day-9/2.txt".to_string());
    let result = aoc_2024::advent_of_code::day_9::Day9::dispatch(1, buf.lines());
    assert_eq!(result, "1928");
}

#[test]
fn test_3_puzzle_1() {
    let buf = read_file(&"inputs/day-3.txt".to_string());
    let result = aoc_2024::advent_of_code::day_9::Day9::dispatch(1, buf.lines());
    assert_eq!(result, "6299243228569");
}

