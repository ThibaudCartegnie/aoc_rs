use crate::common::Day;
use itertools::Itertools;

pub struct Day02;

fn is_safe(reports: &Vec<i32>) -> bool {
    let inc = reports.iter().tuple_windows().all(|(a, b)| a > b);
    let dec = reports.iter().tuple_windows().all(|(a, b)| a < b);
    let differ = reports.iter().tuple_windows().map(|(a, b)| a.abs_diff(*b)).any(|d| d < 1 || d > 3);
    !differ && (inc || dec)
}

impl Day for Day02 {
    fn solve_part1(&self, input: &str) -> String {
        let mut res = 0;
        for line in input.lines() {
            let nbs: Vec<i32> = line.split(' ').map(|n| i32::from_str_radix(n, 10).unwrap()).collect();
            let safe = is_safe(&nbs);
            res += if safe {
                1
            } else {
                0
            };

        }
        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut res = 0;
        for line in input.lines() {
            let nbs: Vec<i32> = line.split(' ').map(|n| i32::from_str_radix(n, 10).unwrap()).collect();
            let mut safe = is_safe(&nbs);
            if !safe {
                let l = nbs.len();
                for i in 0..l {
                    let mut reports = nbs.clone();
                    reports.remove(i);
                    if is_safe(&reports) {
                        safe = true;
                        break;
                    }
                }
            }
            res += if safe {
                1
            } else {
                0
            };

        }
        format!("{}", res)
    }
}
