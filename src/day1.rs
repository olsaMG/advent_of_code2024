pub fn day1(input: String) -> u32 {
    let mut left = vec![];
    let mut right = vec![];
    let split_input: Vec<&str> = input.lines().collect();
    dbg!(&split_input);

    split_input.into_iter().for_each(|line| {
        let split_line: Vec<&str> = line.split("   ").collect();
        left.push(split_line[0]);
        right.push(split_line[1]);
    });

    left.sort();
    right.sort();

    let mut sum = 0;

    for (idx, val) in left.into_iter().enumerate() {
        let left_val: u32 = val.parse().unwrap();
        let right_val: u32 = right[idx].parse().unwrap();

        sum += diff(left_val, right_val);
    }

    dbg!(sum);

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

    use crate::day1::day1;

    use super::diff;

    #[test]
    fn day1_test() {
        let input = fs::read_to_string("./input/day1_test").unwrap();

        assert_eq!(day1(input), 11)
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
}
