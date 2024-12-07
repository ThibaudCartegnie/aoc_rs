use core::fmt;
use std::collections::HashSet;
use crate::common::Day;

pub struct Day06;

fn _find_guard(line: (usize, &[char])) -> Option<(isize, isize)> {
    match line.1.iter().enumerate().find(|(_, p)| **p == '^') {
        Some((i, _)) => Some((line.0 as isize, i as isize)),
        None => None
    }
}

fn get(map: &Vec<Vec<char>>, i: isize, j: isize) -> Option<char> {
    // println!("Test: {:?} {:?} {:?} {:?} none? {}", i, j, map.len(), map[0].len(), ! (j >= 0 && j < map.len() as isize && i >= 0 && i < map[0].len() as isize));
    if ! (j >= 0 && j < map.len() as isize && i >= 0 && i < map[0].len() as isize) {
        return None
    }
    Some(map[j as usize][i as usize])
}

impl Day for Day06 {
    fn solve_part1(&self, input: &str) -> String {
        let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        println!("Should be 4602");
        let res = if let End::Out(n, _) = solve_maze(&map) {
            n
        } else {
            panic!("Bugged input :'(");
        };
        assert!(4602 == res);
        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut res = 0;
        let normal_path =  if let End::Out(_, v) = solve_maze(&map) {
            v
        } else {
            panic!("Bugged input :'(");
        };
        for (i, j) in normal_path {
            let i = i as usize;
            let j = j as usize;
            if map[i][j] == '#' || map[i][j] == '^' {
                continue;
            }
            map[i][j] = '#';
            if solve_maze(&map) == End::Infinite {
                res += 1;
            }
            map[i][j] = '.';

        }
        println!("Should be 1703");
        assert!(1703 == res);
        format!("{}", res)
        // 1563 too low
    }



    
}

#[derive(Debug, Clone, PartialEq)]
enum End {
    Out(i32, HashSet<(isize, isize)>), // nb of steps to be out
    Infinite
}

impl fmt::Display for End {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            End::Out(nb, _) => write!(f, "End::Out({})", nb),
            End::Infinite => write!(f, "End::Infinite"),
        }
    }
}

fn solve_maze(map: &Vec<Vec<char>>) -> End {
    let mut guard: (isize, isize) = map.iter().enumerate().filter_map(|(i, l)|_find_guard((i, &l))).next().unwrap();
    // map[guard.1][guard.0] as char == '^'
    let dirs = [(-1,0), (0,1), (1,0), (0,-1)];
    let mut dir_idx = 0;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut visited_with_dirs = HashSet::new();
    while get(&map, guard.1, guard.0) != None {
        let guard_dirs = (guard.clone(), dirs[dir_idx].clone());
        if visited_with_dirs.contains(&guard_dirs){
            return End::Infinite;
        }
        visited_with_dirs.insert(guard_dirs);
        if ! visited.contains(&guard) {
            visited.insert(guard.clone());
        }

        let mut checked = false;
        while ! checked {
            match get(&map, guard.1 + dirs[dir_idx].1, guard.0 + dirs[dir_idx].0) {
                Some(c) => {
                    if c == '#' {
                        dir_idx = (dir_idx+1)%4;
                    } else {
                        checked = true;
                    }
                },
                None => checked = true,
            };
        }
        guard = (guard.0 + dirs[dir_idx].0, guard.1 + dirs[dir_idx].1);
        // check next and switch direction
    }
    End::Out(visited.len() as i32, visited)
}
