use crate::common::Day;

pub struct Day02;

impl Day for Day02 {
    fn solve_part1(&self, input: &str) -> String {
        let lines = input.lines();
        let mut horizontal_pos = 0;
        let mut depth = 0;
        for line in lines {
            if line.len() == 0 {
                continue;
            }
            let nb_str = line.split(" ").last().unwrap();
            let nb = i32::from_str_radix(nb_str, 10).unwrap();
            if line.starts_with("forward ") {
                horizontal_pos += nb;
            }

            if line.starts_with("down ") {
                depth += nb;
            }

            if line.starts_with("up ") {
                depth -= nb;
            }

        }

        format!("{}", depth*horizontal_pos)

    }

    fn solve_part2(&self, input: &str) -> String {
        let lines = input.lines();
        let mut horizontal_pos = 0;
        let mut depth = 0;
        let mut aim = 0;
        for line in lines {
            if line.len() == 0 {
                continue;
            }
            let nb_str = line.split(" ").last().unwrap();
            let nb = i32::from_str_radix(nb_str, 10).unwrap();
            if line.starts_with("forward ") {
                horizontal_pos += nb;
                depth += aim*nb;
            }

            if line.starts_with("down ") {
                aim += nb;
            }

            if line.starts_with("up ") {
                aim -= nb;
            }

        }

        format!("{}", depth*horizontal_pos)
    }
}
