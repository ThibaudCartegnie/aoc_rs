use std::str::FromStr;

use crate::common::Day;

pub struct Day14;

#[derive(Debug, Clone)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64)
}

impl FromStr for Robot {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let pos_str = parts.next().unwrap().strip_prefix("p=").unwrap();
        let mut pos_parts = pos_str.split(',');
        let x = pos_parts.next().unwrap().parse().unwrap();
        let y = pos_parts.next().unwrap().parse().unwrap();
        
        let vel_str = parts.next().unwrap().strip_prefix("v=").unwrap();
        let mut vel_parts = vel_str.split(',');
        let vx = vel_parts.next().unwrap().parse().unwrap();
        let vy = vel_parts.next().unwrap().parse().unwrap();


        Ok(Robot {
            pos: (x, y),
            vel: (vx, vy)
        })
    }
}

impl Robot {
    fn steps(&mut self, n: i64, h: i64, w: i64) {
        let mut x = (self.pos.0 + self.vel.0*n) % w;
        let mut y = (self.pos.1 + self.vel.1*n) % h;
        if x < 0 {
            x += w;
        }
        if y < 0 {
            y += h;
        }
        self.pos = (x, y);
    }
}

fn count_robot(mut acc: (i64, i64, i64, i64), robot: &Robot, h: i64, w: i64) -> (i64, i64, i64, i64) {
    let x_mid = (w-1)/2;
    let y_mid = (h-1)/2;
    if robot.pos.0 < x_mid && robot.pos.1 < y_mid {
        acc.0 += 1;
    }

    if robot.pos.0 < x_mid && robot.pos.1 > y_mid {
        acc.1 += 1;
    }

    if robot.pos.0 > x_mid && robot.pos.1 < y_mid {
        acc.2 += 1;
    }

    if robot.pos.0 > x_mid && robot.pos.1 > y_mid {
        acc.3 += 1;
    }

    acc
}

#[allow(unused)]
impl Day for Day14 {
    fn solve_part1(&self, input: &str) -> String {
        let mut robots: Vec<Robot> = input.lines().map(|l| Robot::from_str(l).unwrap()).collect();

        let h = 103;
        let w = 101;
        // Example 
        // let h = 11;
        // let w = 7;
        for robot in robots.iter_mut() {
            robot.steps(100, h, w);
        }

        // println!("{:?}", &robots);

        // print_robots(&robots, h, w);

        let res = robots.iter().fold((0,0,0,0), |acc, robot| count_robot(acc, robot, h, w));
        format!("{}", res.0*res.1*res.2*res.3)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut robots: Vec<Robot> = input.lines().map(|l| Robot::from_str(l).unwrap()).collect();

        let h = 103;
        let w = 101;
        // Example 
        // let h = 11;
        // let w = 7;
        // for robot in robots.iter_mut() {
        //     robot.steps(100, h, w);
        // }

        // println!("{:?}", &robots);

        let mut original_robots = robots.clone();


        let mut results = Vec::with_capacity(10000);
        for i in 0..10000 {
            for robot in robots.iter_mut() {
                robot.steps(1, h, w);
            }

            let res = robots.iter().fold((0,0,0,0), |acc, robot| count_robot(acc, robot, h, w));

            results.push(res.0*res.1*res.2*res.3);
            // println!("{} seconds !", i+1);
            // print_robots(&robots, h, w, true);
        }

        let min_score = results.iter().enumerate().min_by(|a, b| a.1.cmp(b.1)).unwrap();

        for robot in original_robots.iter_mut() {
            robot.steps(min_score.0 as i64 + 1, h, w);
        }

        // _print_robots(&original_robots, h, w, false);
        format!("{:?}", min_score.0 as i64 + 1)
    }
}


fn _print_robots(robots: &Vec<Robot>, h: i64, w: i64, print_on_condition: bool) {
    let mut grid = vec![Vec::<char>::with_capacity(w as usize); h as usize];
    for line in grid.iter_mut() {
        for _ in 0..w {
            line.push('.');
        }
    }

    for robot in robots.iter() {
        let x = robot.pos.0 as usize;
        let y = robot.pos.1 as usize;
        match grid[y][x] {
            '.' => grid[y][x] = '1',
            _ => grid[y][x] = char::from_digit(grid[y][x].to_digit(10).unwrap() + 1, 10).unwrap()
        }
    }

    if print_on_condition && grid[(h as usize -1)/2][(w as usize-1)/2] == '.' {
        return;
    }

    for line in grid.iter() {
        let l: String = line.iter().collect();
        println!("{}", l);
    }

}