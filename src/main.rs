use crate::day1::day1;
use std::fs;

mod day1;

fn main() {
    let input_day1 = fs::read_to_string("./input/day1_input").unwrap();
    day1(input_day1);
}
