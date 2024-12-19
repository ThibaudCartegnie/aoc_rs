use pathfinding::prelude::astar;

use crate::common::Day;

pub struct Day18;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i64, i64);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, bytes: &[(i64, i64)]) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        let dirs = [(0,1), (1,0), (0,-1), (-1,0)];

        let mut sucs = Vec::with_capacity(4);
        for (ndx, ndy) in dirs.iter().rev() {
            let nx = x + ndx;
            let ny = y + ndy;
            if nx >= 0 && nx < 71 && ny >= 0 && ny < 71 && !bytes.contains(&(x, y)) {
                sucs.push((Pos(nx, ny), 1));
            }
        }
        sucs
    }
}

#[allow(unused)]
impl Day for Day18 {
    fn solve_part1(&self, input: &str) -> String {
        let bytes: Vec<(i64, i64)> = input.lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();

        let start = Pos(0,0);
        let goal = Pos(70,70);

        let res = astar(&start, |p| p.successors(&bytes[..1024]), |p| p.distance(&goal), |p| *p == goal).unwrap();

        format!("{}", res.1)
    }

    fn solve_part2(&self, input: &str) -> String {
        let bytes: Vec<(i64, i64)> = input.lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();

        let start = Pos(0,0);
        let goal = Pos(70,70);


        // search by dichotomy solution
        let mut lower_bound = 1024; // from part 1
        let mut upper_bound = bytes.len(); // obviously

        while upper_bound - lower_bound > 1 {
            let i = (lower_bound + upper_bound)/2;
            // println!("Test {} low: {}, up: {}", i, lower_bound, upper_bound);
            match astar(&start, |p| p.successors(&bytes[..i]), |p| p.distance(&goal), |p| *p == goal) {
                Some(_) => {
                    lower_bound = i;
                },
                None => {
                    upper_bound = i;
                }
            }
        }

        // brute-force solution, the one i actually used

        // for i in 1024..bytes.len() {
        //     println!("{}", i);
        //     let res = 
        //     if res.is_none() {
        //         println!("None!");
        //         res_idx = i - 1;
        //         break;
        //     }
        // }


        format!("{:?}", bytes[lower_bound])
    }
}
