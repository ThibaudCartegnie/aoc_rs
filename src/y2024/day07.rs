use itertools::{repeat_n, Itertools};

use crate::common::Day;

pub struct Day07;

#[derive(Debug, Clone)]
struct Line {
    result: i64,
    operands: Vec<i64>
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let mut parts = value.split(':').into_iter();
        let res = parts.next().unwrap().parse().unwrap();

        let operands = parts.next().unwrap().split(' ').filter_map(|l|l.parse().ok()).collect();
        Self {
            result: res,
            operands: operands
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Times,
    Concat
}

fn concat(a: i64, b: i64) -> i64 {
    a * 10i64.pow(b.ilog10() + 1) + b
}

impl Line {
    fn possible(&self, operators: &[Operator]) -> bool {
        let n_operators = self.operands.len() - 1;
        
        for current_ops in repeat_n(operators.iter(), n_operators).multi_cartesian_product() {
            let r = current_ops.iter().enumerate().fold(self.operands[0], 
                |acc,(i, o)| match o {
                    Operator::Plus => acc + self.operands[i+1],
                    Operator::Times => acc * self.operands[i+1],
                    Operator::Concat => concat(acc, self.operands[i+1])
                }
            );
            if r == self.result {
                return true;
            }
        }
        false
    }
}

impl Day for Day07 {
    fn solve_part1(&self, input: &str) -> String {
        let lines: Vec<Line> = input.lines().map(|l| Line::from(l)).collect();
        let ops = [Operator::Plus, Operator::Times];
        let res = lines.iter().fold(0, |acc, l| if l.possible(&ops) { acc + l.result } else { acc });
        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let lines: Vec<Line> = input.lines().map(|l| Line::from(l)).collect();
        let ops = [Operator::Plus, Operator::Times, Operator::Concat];
        let res = lines.iter().fold(0, |acc, l| if l.possible(&ops) { acc + l.result } else { acc });
        format!("{}", res)
    }
}
