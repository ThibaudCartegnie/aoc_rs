use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::prelude::{bfs, dijkstra};

use crate::common::Day;

pub struct Day20;


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i64, i64);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, map: &Vec<Vec<char>>) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        let dirs = [(0,1), (1,0), (0,-1), (-1,0)];
        let w = map[0].len() as i64;
        let h = map.len() as i64;

        let mut sucs = Vec::with_capacity(4);
        for (ndx, ndy) in dirs.iter().rev() {
            let nx = x + ndx;
            let ny = y + ndy;
            if nx >= 0 && nx < w && ny >= 0 && ny < h && map[ny as usize][nx as usize] != '#' {
                sucs.push((Pos(nx, ny), 1));
            }
        }
        sucs
    }

    fn successors_with_cheat(&self, map: &Vec<Vec<char>>, cheat_length: i64) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        let dirs = [(0,1), (1,0), (0,-1), (-1,0)];
        let w = map[0].len() as i64;
        let h = map.len() as i64;
        let mut sucs = Vec::with_capacity(4);


        let top = 0.max(y - cheat_length);
        let bottom = (h-1).min(y + cheat_length);
        let left = 0.max(x - cheat_length);
        let right = (w-1).min(x + cheat_length);

        for ny in top..=bottom {
            for nx in left..=right {
                if nx >= 0 && nx < w && ny >= 0 && ny < h && map[ny as usize][nx as usize] != '#' {
                    let np = Pos(nx, ny);
                    if self.distance(&np) <= cheat_length as u32 {
                        sucs.push((Pos(nx, ny), 1));
                    }
                }
            }
        }

        // for (ndx, ndy) in dirs.iter().rev() {
        //     for cx in 1..=cheat_length {
        //         for cy in 1..=cheat_length {
        //             let nx = x + ndx*cx;
        //             let ny = y + ndy*cy;
        //             if nx >= 0 && nx < w && ny >= 0 && ny < h && map[ny as usize][nx as usize] != '#' {
        //                 let np = Pos(nx, ny);
        //                 if self.distance(&np) <= cheat_length as u32 {
        //                     sucs.push((Pos(nx, ny), 1));
        //                 }
        //             }
        //         }
        //     }
        // }
        sucs
    }
}

#[allow(unused)]
impl Day for Day20 {
    fn solve_part1(&self, input: &str) -> String {
        let map = input.lines().map(|l | l.chars().collect_vec()).collect_vec();
        let mut start = Pos(0, 0);
        let mut end = Pos(0, 0);
        'outer: for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == 'S' {
                    start = Pos(x as i64, y as i64);
                } else if *c == 'E' {
                    end = Pos(x as i64, y as i64);
                }

                if start.0 != 0 && end.0 != 0 {
                    break 'outer;
                }
            }
        }

        let start = Pos(start.0, start.1);
        let end = Pos(end.0, end.1);

        let path = dijkstra(&start, |p| p.successors(&map), |p| *p == end).unwrap();
        println!("{} {}", path.0.len(), path.1);

        let mut path_map = HashMap::with_capacity(path.0.len());
        for (i, p) in path.0.iter().enumerate() {
            path_map.insert(p.clone(), i);
        }

        let path_len = path.1;

        let mut res = 0;
        for (d1, pos) in path.0.iter().enumerate() {
            let sucs = pos.successors_with_cheat(&map, 2);
            for (p2, _) in sucs {
                if let Some(d2) = path_map.get(&p2) {
                    if *d2 as i64 - d1 as i64 > 100 {
                        res += 1;
                    }
                }
            }
        }


        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let map = input.lines().map(|l | l.chars().collect_vec()).collect_vec();
        let mut start = Pos(0, 0);
        let mut end = Pos(0, 0);
        'outer: for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == 'S' {
                    start = Pos(x as i64, y as i64);
                } else if *c == 'E' {
                    end = Pos(x as i64, y as i64);
                }

                if start.0 != 0 && end.0 != 0 {
                    break 'outer;
                }
            }
        }

        let start = Pos(start.0, start.1);
        let end = Pos(end.0, end.1);

        let path = dijkstra(&start, |p| p.successors(&map), |p| *p == end).unwrap();
        println!("{} {}", path.0.len(), path.1);

        let mut path_map = HashMap::with_capacity(path.0.len());
        for (i, p) in path.0.iter().enumerate() {
            path_map.insert(p.clone(), i);
        }

        let path_len = path.1;

        let mut res = 0;
        let mut saved_times = HashMap::new();
        for (d1, pos) in path.0.iter().enumerate() {
            let sucs = pos.successors_with_cheat(&map, 20);
            for (p2, _) in sucs {
                if let Some(d2) = path_map.get(&p2) {
                    let d = *d2 as i64 - d1 as i64;
                    let db = *d2 - (d1 + pos.distance(&p2) as usize);
                    if *d2 > d1 + 2 && db >= 100 {
                        if saved_times.contains_key(&db) {
                            *saved_times.get_mut(&db).unwrap() += 1;
                        } else {
                            saved_times.insert(db, 1);
                        }
                        res += 1;
                    }
                }
            }
        }

        // let mut v = saved_times.iter().map(|(k,v)| (*k, *v)).collect_vec();

        // v.sort();
        // for (d, nb) in v {
        //     println!("{} cheats that save {} ps", nb, d);
        // }
        // println!("{:?}", &saved_times);

        format!("110228 too low, 2204560 too high {}", res)
    }
}
