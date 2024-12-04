pub fn day2(input: String) -> i32 {
    let reports_split: Vec<&str> = input.lines().collect();
    let mut reports = vec![];

    reports_split.into_iter().for_each(|line| {
        let report: Vec<&str> = line.split(" ").collect();
        let mapped: Vec<u8> = report
            .clone()
            .into_iter()
            .map(|str| str.parse::<u8>().unwrap())
            .collect();

        // dbg!(&mapped);
        reports.push(mapped);
    });
    let mut valid_reports = 0;

    reports.iter().for_each(|report| {
        if check_report(report) {
            valid_reports += 1;
        }
    });

    dbg!(valid_reports);

    valid_reports
}

fn check_report(report: &[u8]) -> bool {
    let mut safe = true;
    if report[0] < report[1] {
        dbg!("in accenting");
        report.iter().enumerate().for_each(|(idx, level)| {
            let next = report.get(idx + 1);

            if safe {
                if let Some(next_idx) = next {
                    dbg!(level, next);
                    if level < next_idx {
                        let diff = next_idx - level;
                        dbg!(diff);
                        if (1..=3).contains(&diff) {
                            safe = true;
                            dbg!("Safe");
                        } else {
                            dbg!("Unsafe");
                            safe = false;
                        }
                    } else {
                        safe = false;
                    }
                }
            }
        });
    } else {
        dbg!("in decending");
        report.iter().enumerate().for_each(|(idx, level)| {
            let next = report.get(idx + 1);

            if safe {
                if let Some(next_idx) = next {
                    dbg!(level, next);
                    if level > next_idx {
                        let diff = level - next_idx;
                        dbg!(diff);
                        if (1..=3).contains(&diff) {
                            safe = true;
                            dbg!("Safe");
                        } else {
                            dbg!("Unsafe");
                            safe = false;
                        }
                    } else {
                        safe = false;
                    }
                }
            }
        });
    }

    dbg!("end of check", safe);
    safe
}

#[cfg(test)]
mod day2_tests {
    use std::fs;

    use crate::day2::{check_report, day2};

    #[test]
    fn day2_test() {
        let input = fs::read_to_string("./input/day2_test").unwrap();

        assert_eq!(day2(input), 2);
    }

    #[test]
    fn check_report_test() {
        //Safe
        let report1 = [1, 2, 3, 4, 5];
        let report2 = [5, 4, 3, 2, 1];
        let report7 = [50, 48, 47, 45, 42, 40];
        let report8 = [29, 28, 26, 25, 22, 19, 16, 15];
        //Unsafe
        let report3 = [1, 2, 2, 4, 5];
        let report4 = [1, 2, 1, 4, 5];
        let report5 = [1, 2, 7, 4, 5];
        let report6 = [5, 4, 5, 2, 1];

        assert!(check_report(&report1));
        assert!(check_report(&report2));
        assert!(check_report(&report7));
        assert!(check_report(&report8));
        assert!(!check_report(&report3));
        assert!(!check_report(&report4));
        assert!(!check_report(&report5));
        assert!(!check_report(&report6));
    }
}
