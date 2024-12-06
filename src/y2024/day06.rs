use core::fmt;
use std::collections::HashSet;
use crate::common::Day;

pub struct Day06;

fn _find_guard(line: (usize, &&[u8])) -> Option<(isize, isize)> {
    match line.1.iter().enumerate().find(|(_, p)| **p == '^' as u8) {
        Some((i, _)) => Some((line.0 as isize, i as isize)),
        None => None
    }
}
fn _find_guard_char(line: (usize, &[char])) -> Option<(isize, isize)> {
    match line.1.iter().enumerate().find(|(_, p)| **p == '^') {
        Some((i, _)) => Some((line.0 as isize, i as isize)),
        None => None
    }
}

fn get<'a>(map: &'a Vec<&'a [u8]>, i: isize, j: isize) -> Option<&'a u8> {
    // println!("Test: {:?} {:?} {:?} {:?} none? {}", i, j, map.len(), map[0].len(), ! (j >= 0 && j < map.len() as isize && i >= 0 && i < map[0].len() as isize));
    if ! (j >= 0 && j < map.len() as isize && i >= 0 && i < map[0].len() as isize) {
        return None
    }
    Some(&map[j as usize][i as usize])
}

fn get_char(map: &Vec<Vec<char>>, i: isize, j: isize) -> Option<char> {
    // println!("Test: {:?} {:?} {:?} {:?} none? {}", i, j, map.len(), map[0].len(), ! (j >= 0 && j < map.len() as isize && i >= 0 && i < map[0].len() as isize));
    if ! (j >= 0 && j < map.len() as isize && i >= 0 && i < map[0].len() as isize) {
        return None
    }
    Some(map[j as usize][i as usize])
}

impl Day for Day06 {
    fn solve_part1(&self, input: &str) -> String {
        let map: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
        format!("{}", solve_maze(&map))
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let orig_map = map.clone();
        // let mut map: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
        let h = map.len();
        let w = map[0].len();
        let mut res = 0;
        // let guard: (isize, isize) = map.iter().enumerate().filter_map(|(i, l)|_find_guard_char((i, &l))).next().unwrap();
        for i in 0..h {
            if i%20 == 0 {
                println!("Testing {} {}", i, res);
            }
            for j in 0..w {
                if map[i][j] == '#' || map[i][j] == '^' {
                    continue;
                }
                map[i][j] = '#';
                if solve_maze_char(&map) == End::Infinite {
                    res += 1;
                    map[i][j] = 'O';
                } else {
                    map[i][j] = '.';

                }
            }
        }
        for line in map {
            let a: String = line.into_iter().collect();
            println!("{}", a);
        }
        format!("{}", res)
        // 1563 too low
    }



    
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum End {
    Out(i32), // nb of steps to be out
    Infinite
}

impl fmt::Display for End {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            End::Out(nb) => write!(f, "End::Out({})", nb),
            End::Infinite => write!(f, "End::Infinite"),
        }
    }
}

fn solve_maze(map: &Vec<&[u8]>) -> End {
    let mut guard: (isize, isize) = map.iter().enumerate().filter_map(_find_guard).next().unwrap();
    // map[guard.1][guard.0] as char == '^'
    let mut res = 0;
    let dirs = [(-1,0), (0,1), (1,0), (0,-1)];
    let mut dir_idx = 0;
    let mut visited = Vec::new();
    let mut visited_with_dirs = Vec::new();
    while get(&map, guard.1, guard.0) != None {
        let guard_dirs = (guard.clone(), dirs[dir_idx].clone());
        if visited_with_dirs.contains(&guard_dirs){
            return End::Infinite;
        }
        visited_with_dirs.push(guard_dirs);
        if ! visited.contains(&guard) {
            res += 1;
            visited.push(guard.clone());
        }
        guard = (guard.0 + dirs[dir_idx].0, guard.1 + dirs[dir_idx].1);
        // check next and switch direction
        if let Some(c) = get(&map, guard.1 + dirs[dir_idx].1, guard.0 + dirs[dir_idx].0) {
            if *c == '#' as u8 {
                dir_idx = (dir_idx+1)%4;
            }
        }
    }
    End::Out(res)
}

fn solve_maze_char(map: &Vec<Vec<char>>) -> End {
    let mut guard: (isize, isize) = map.iter().enumerate().filter_map(|(i, l)|_find_guard_char((i, &l))).next().unwrap();
    // map[guard.1][guard.0] as char == '^'
    let mut res = 0;
    let dirs = [(-1,0), (0,1), (1,0), (0,-1)];
    let mut dir_idx = 0;
    let mut visited_with_dirs = HashSet::new();
    while get_char(&map, guard.1, guard.0) != None {
        let guard_dirs = (guard.clone(), dirs[dir_idx].clone());
        if visited_with_dirs.contains(&guard_dirs){
            return End::Infinite;
        }
        visited_with_dirs.insert(guard_dirs);

        let mut checked = false;
        while ! checked {
            match get_char(&map, guard.1 + dirs[dir_idx].1, guard.0 + dirs[dir_idx].0) {
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
    End::Out(res)
}
