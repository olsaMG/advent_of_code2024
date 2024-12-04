use crate::day1::day1;
use crate::day2::day2;
use std::fs;

mod day1;
mod day2;

fn main() {
    // let input_day1 = fs::read_to_string("./input/day1_input").unwrap();
    // day1(input_day1);

    let input_day2 = fs::read_to_string("./input/day2_input").unwrap();
    day2(input_day2);
}
