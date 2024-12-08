use std::collections::HashMap;

use itertools::Itertools;

use crate::common::Day;

pub struct Day08;

#[derive(Debug, Clone)]
struct Point {
    antenna: Option<char>,
    antinode: bool
}

impl Point {
    fn new(value: char) -> Point {
        let antenna = match value {
            '.' => None,
            c => Some(c)
        };
        Point{
            antenna,
            antinode: false
        }
    }
}

fn create_point(x: usize, y: usize, value: char, antennas: &mut HashMap<char, Vec<(usize, usize)>>) -> Point {
    let p = Point::new(value);
    if let Some(c) = p.antenna {
        if let Some(v) = antennas.get_mut(&c) {
            v.push((x, y));
        } else {
            antennas.insert(c, vec![(x, y)]);
        }
    }
    p
}

impl Day for Day08 {
    fn solve_part1(&self, input: &str) -> String {
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        let mut map: Vec<Vec<Point>> = input.lines().enumerate().map(|(y, l) | l.char_indices().map(|(x, value)| create_point(x, y, value, &mut antennas)).collect()).collect();
        let h = map.len() as isize;
        let w = map[0].len() as isize;

        for freq_antennas in antennas.values() {
            for antenna_vec in freq_antennas.iter().combinations(2) {
                let a0 = antenna_vec[0];
                let a0 = (a0.0 as isize, a0.1 as isize);
                let a1 = antenna_vec[1];
                let a1 = (a1.0 as isize, a1.1 as isize);

                let an1 = (2*a0.0-a1.0, 2*a0.1-a1.1);
                let an2 = (2*a1.0-a0.0, 2*a1.1-a0.1);
                if an1.0 >= 0 && an1.0 < w && an1.1 >= 0 && an1.1 < h {
                    map[an1.1 as usize][an1.0 as usize].antinode = true;
                }

                if an2.0 >= 0 && an2.0 < w && an2.1 >= 0 && an2.1 < h {
                    map[an2.1 as usize][an2.0 as usize].antinode = true;
                }
            }
        }

        let res: usize = map.iter().map(|l| l.iter().filter(|p| p.antinode).count()).sum();

        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        let mut map: Vec<Vec<Point>> = input.lines().enumerate().map(|(y, l) | l.char_indices().map(|(x, value)| create_point(x, y, value, &mut antennas)).collect()).collect();
        let h = map.len() as isize;
        let w = map[0].len() as isize;

        for freq_antennas in antennas.values() {
            for antenna_vec in freq_antennas.iter().combinations(2) {
                let a0 = antenna_vec[0];
                map[a0.1][a0.0].antinode = true;
                let a0 = (a0.0 as isize, a0.1 as isize);
                let a1 = antenna_vec[1];
                map[a1.1][a1.0].antinode = true;
                let a1 = (a1.0 as isize, a1.1 as isize);

                let (dx, dy) = (a0.0 - a1.0, a0.1 - a1.1);
                let mut an1 = (a0.0+dx, a0.1 + dy);
                while an1.0 >= 0 && an1.0 < w && an1.1 >= 0 && an1.1 < h {
                    map[an1.1 as usize][an1.0 as usize].antinode = true;
                    an1 = (an1.0 + dx, an1.1 + dy);
                }

                let (dx, dy) = (a1.0 - a0.0, a1.1 - a0.1);
                let mut an1 = (a1.0+dx, a1.1 + dy);
                while an1.0 >= 0 && an1.0 < w && an1.1 >= 0 && an1.1 < h {
                    map[an1.1 as usize][an1.0 as usize].antinode = true;
                    an1 = (an1.0 + dx, an1.1 + dy);
                }
            }
        }

        let res: usize = map.iter().map(|l| l.iter().filter(|p| p.antinode).count()).sum();
        // 954 too low
        format!("{}", res)
    }
}
