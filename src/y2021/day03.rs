use crate::common::Day;

pub struct Day03;

#[derive(Clone, Copy, Debug, Default)]
struct Stat {
    pub n0: i32,
    pub n1: i32
}

impl Stat {
    fn get_most(&self) -> i32 {
        if self.n0 > self.n1 {
            0
        } else {
            1
        }
    }
    fn get_least(&self) -> i32 {
        if self.n0 > self.n1 {
            1
        } else {
            0
        }
    }
    fn get_most_c(&self) -> char {
        if self.n0 > self.n1 {
            '0'
        } else {
            '1'
        }
    }
    fn get_least_c(&self) -> char {
        if self.n0 > self.n1 {
            '1'
        } else {
            '0'
        }
    }
}

impl Day for Day03 {
    fn solve_part1(&self, input: &str) -> String {
        let lines = input.lines();

        let first_line = lines.clone().next().unwrap();
        let n_bits = first_line.len();
        let mut stats = vec![Stat{n0:0, n1:0}; n_bits];

        for line in lines {
            if line.len() == 0 {
                continue;
            }
            for (i, c) in line.char_indices() {
                if c == '0' {
                    stats[i].n0 += 1;
                } else {
                    stats[i].n1 += 1;
                }
            }
        }

        let gamma = stats.iter()
            .rev()
            .map(Stat::get_most)
            .enumerate()
            .fold(0, |acc, (i, val)| acc + i32::pow(2, i.try_into().unwrap())*val);
        let epsilon = stats.iter().rev().map(Stat::get_least).enumerate().fold(0, |acc, (i, val)| acc + i32::pow(2, i.try_into().unwrap())*val);
        format!("{}", gamma*epsilon)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut mosts: Vec<Vec<char>> = input.lines().filter(|l| l.len() != 0).map(|l| l.chars().collect()).collect();
        let mut leasts = mosts.clone();
        let w = mosts[0].len();
        for j in 0..w {
            if mosts.len() != 1 {
                handle_row(&mut mosts, j, Stat::get_most_c);
            }
            if leasts.len() != 1 {
                handle_row(&mut leasts, j, Stat::get_least_c);
            }
        }
        let generator_s: String = mosts[0].iter().collect();
        let scrubber_s: String = leasts[0].iter().collect();

        let generator = i32::from_str_radix(&generator_s, 2).unwrap();
        let scrubber = i32::from_str_radix(&scrubber_s, 2).unwrap();

        format!("{}", generator*scrubber)
    }
}

fn handle_row<T: Fn(&Stat) -> char>(lines: &mut Vec<Vec<char>>, row: usize, func: T) {
    let mut stat = Stat::default();
    for i in 0..lines.len() {
        if lines[i][row] == '0' {
            stat.n0 += 1;
        } else {
            stat.n1 += 1;
        }
    }

    let extremum = func(&stat);

    for i in (0..lines.len()).rev() {
        if lines[i][row] != extremum {
            lines.remove(i);
        }
    }
}