use std::io::BufRead;
use aoc_2024::advent_of_code::Day;

use common::read_file;

extern crate aoc_2024;

mod common;

#[test]
fn test_1_puzzle_1() {
    let buf = read_file(&"tests/day-11/1.txt".to_string());
    let result = aoc_2024::advent_of_code::day_11::Day11::dispatch(1, buf.lines());
    assert_eq!(result, "55312");
}



