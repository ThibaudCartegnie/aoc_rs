use crate::common::Day;
use regex::Regex;

pub struct Day03;

#[allow(unused)]
impl Day for Day03 {
    fn solve_part1(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        // let mut s = 0;
        // for (_, [n1, n2]) in re.captures_iter(input).map(|c| c.extract()) {
        //     s += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
        // }
        let s: i32 = re.captures_iter(input).map(|c| c.extract()).map(|(_, [n1, n2])| n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap()).sum();
        format!("{}", s)
    }

    fn solve_part2(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let re2 = Regex::new(r"(do\(\)|don't\(\))").unwrap();
        let dos: Vec<(usize, bool)> = re2.find_iter(input).map(|m| (m.start(), m.as_str() == "do()")).collect();
        let mut s = 0;
        for (idx, (_, [n1, n2])) in re.captures_iter(input).map(|c| (c.get(0).unwrap().start(), c.extract())) {
            if dos.iter().filter(|(idx_do, _)| *idx_do < idx).last().unwrap_or(&(0 as usize, true)).1 {
                s += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
            }
        }
        format!("{}", s)
    }
}
