use std::io::BufRead;
use aoc_2024::advent_of_code::Day;

use common::read_file;

extern crate aoc_2024;

mod common;

#[test]
fn sample_example_puzzle_1() {
    let buf = read_file(&"tests/day-9/sample-example.txt".to_string());
    let result = aoc_2024::advent_of_code::day_9::Day9::dispatch(1, buf.lines());
    assert_eq!("", "");
}
