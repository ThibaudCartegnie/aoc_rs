use std::i32;

use crate::common::Day;

pub struct Day01;

impl Day for Day01 {
    fn solve_part1(&self, input: &str) -> String {
        let lines = input.split('\n');
        let mut last_val = i32::MAX;
        let mut nb_increase = 0;
        for line in lines {
            let maybeval = i32::from_str_radix(line, 10);
            if let Ok(val) = maybeval {
                if val > last_val {
                    nb_increase += 1;
                }
                last_val = val
            }
        }
        format!("Found {} increases!", nb_increase)
    }

    fn solve_part2(&self, input: &str) -> String {
        let lines: Vec<&str> = input.split('\n').collect();
        let mut accs: Vec<i32> = vec![0; lines.len()-2];
        for (i, line) in lines.iter().enumerate() {
            let maybeval = i32::from_str_radix(line, 10);
            if let Ok(val) = maybeval {
                if (i as isize)-2 >= 0 && i-2 < accs.len() {
                    accs[i-2] += val;
                }
                if (i as isize)-1 >= 0 && i-1 < accs.len() {
                    accs[i-1] += val;
                }
                if i < accs.len() {
                    accs[i] += val;
                }
            }
        }

        let mut last_val = i32::MAX;
        let mut nb_increase = 0;

        for val in accs {
            if val > last_val {
                nb_increase += 1;
            }
            last_val = val
        }
        
        format!("Found {} increases!", nb_increase)
    }
}
