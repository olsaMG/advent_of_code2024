pub fn day1(input: String) -> (u32, u32) {
    let mut left = vec![];
    let mut right = vec![];
    let split_input: Vec<&str> = input.lines().collect();

    split_input.into_iter().for_each(|line| {
        let split_line: Vec<&str> = line.split("   ").collect();
        left.push(split_line[0]);
        right.push(split_line[1]);
    });

    left.sort();
    right.sort();

    let mut sum = 0;

    for (idx, val) in left.iter().enumerate() {
        let left_val: u32 = val.parse().unwrap();
        let right_val: u32 = right[idx].parse().unwrap();

        sum += diff(left_val, right_val);
    }
    let mut similarity = 0;
    left.into_iter().for_each(|left_val| {
        let count = count_appearance(left_val, &right);

        let parsed_left: u32 = left_val.parse().unwrap();

        similarity += parsed_left * count
    });

    dbg!(sum);
    dbg!(similarity);

    (sum, similarity)
}

fn count_appearance(left_val: &str, right: &[&str]) -> u32 {
    let mut sum = 0;
    right.iter().copied().for_each(|right_val| {
        if *left_val == *right_val {
            sum += 1
        }
    });

    sum
}

fn diff(a: u32, b: u32) -> u32 {
    if a >= b {
        return a - b;
    }
    b - a
}

#[cfg(test)]

mod day1_tests {
    use std::fs;

    use crate::day1::{count_appearance, day1};

    use super::diff;

    #[test]
    fn day1_part1_test() {
        let input = fs::read_to_string("./input/day1_test").unwrap();
        let result = day1(input);

        assert_eq!(result.0, 11)
    }

    #[test]
    fn day1_part2_test() {
        let input = fs::read_to_string("./input/day1_test").unwrap();
        let result = day1(input);

        assert_eq!(result.1, 31)
    }

    #[test]
    fn diff_test() {
        let poss = diff(3, 1);
        let neg = diff(1, 3);
        let same = diff(3, 3);

        assert_eq!(poss, 2);
        assert_eq!(neg, 2);
        assert_eq!(same, 0);
    }

    #[test]
    fn count() {
        let one = "1";
        let two = "2";
        let tree = "3";
        let right = ["1", "1", "2", "3", "3", "3"];

        assert_eq!(count_appearance(one, &right), 2);
        assert_eq!(count_appearance(two, &right), 1);
        assert_eq!(count_appearance(tree, &right), 3);
    }
}
