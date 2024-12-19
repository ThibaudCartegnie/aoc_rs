use memoize::memoize;

use crate::common::Day;

pub struct Day19;

#[memoize(Ignore: patterns)]
fn is_design_possible(design: &'static str, patterns: &Vec<&str>) -> bool {
    let is_a_pattern = patterns.contains(&design);
    if is_a_pattern {
        return true;
    }
    if design.len() == 1 {
        return is_a_pattern;
    } else {
        for i in 0..design.len() {
            if is_design_possible(&design[..i], patterns) && is_design_possible(&design[i..], patterns) {
                return true;
            }
        }
        false
    }
} 
#[memoize(Ignore: patterns)]
fn count_possible_designs(design: &'static str, patterns: &Vec<&str>) -> i64 {
    let is_a_pattern = patterns.contains(&design);
    if design.len() == 1 {
        match is_a_pattern {
            true => 1,
            false => 0,
        }
    } else {
        let mut possibilities = 0;
        for i in 0..design.len() {
            possibilities += count_possible_designs(&design[..i], patterns) * count_possible_designs(&design[i..], patterns);
        }
        possibilities
    }
}

#[allow(unused)]
impl Day for Day19 {
    fn solve_part1(&self, input: &str) -> String {
        let input: &'static mut str = input.to_string().leak();
        let mut towels = Vec::new();
        let mut designs = Vec::new();
        for (i, l) in input.lines().enumerate() {
            if i == 0 {
                towels = l.split(", ").collect();
            } else if l.len() != 0 {
                designs.push(l);
            }
        }

        let mut res = 0;
        for d in designs {
            if is_design_possible(d, &towels) {
                res += 1;
            }
        }

        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let input: &'static mut str = input.to_string().leak();
        let mut towels = Vec::new();
        let mut designs = Vec::new();
        for (i, l) in input.lines().enumerate() {
            if i == 0 {
                towels = l.split(", ").collect();
            } else if l.len() != 0 {
                designs.push(l);
            }
        }

        let mut res = 0;
        for d in designs {
            if is_design_possible(d, &towels) {
                res += count_possible_designs(d, &towels)
            }
        }

        format!("{}", res)
    }
}
