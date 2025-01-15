use std::{collections::HashMap, usize};

use itertools::Itertools;
use memoize::memoize;

use crate::common::Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

impl Dir {
    fn encode(&self) -> char {
        match self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }
}

pub struct Day21;

fn get_lut(pad: &[[char; 3]; 4]) -> Lut {
    let mut lut = HashMap::with_capacity_and_hasher(12, ahash::RandomState::new());
    for (y, line) in pad.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != ' ' {
                lut.insert(*c, (x as i32, y as i32));
            }
        }
    }
    lut
}

type Lut = HashMap<char, (i32, i32), ahash::RandomState>;
type Path = Vec<char>;

#[derive(Debug)]
struct Context {
    num_pad: [[char; 3]; 4],
    num_lut: Lut,
    dir_pad: [[char; 3]; 4],
    dir_lut: Lut
}

#[allow(unused)]
impl Day for Day21 {
    fn solve_part1(&self, input: &str) -> String {
        let num_pad = [
            ['7', '8', '9'],
            ['4', '5', '6'],
            ['1', '2', '3'],
            [' ', '0', 'A'],
        ];
        let dir_pad = [
            [' ', ' ', ' '],
            [' ', ' ', ' '],
            [' ', '^', 'A'],
            ['<', 'v', '>'],
        ];
        let num_lut = get_lut(&num_pad);
        let dir_lut = get_lut(&dir_pad);
        let ctx = Context {
            num_pad,
            num_lut,
            dir_pad,
            dir_lut
        };

        let mut res = 0;

        for line in input.lines() {
            let code: usize = line[..line.len()-1].parse().unwrap();
            let path = line.chars().collect_vec();
            let min_path = shortest(path, 3, true, &ctx);
            // println!("{} : {}*{}={}, Path: {}", line, code, min_path.len(), code*min_path.len(), min_path.iter().join(""));
            res += code*min_path.len();
        }

        format!("158224 too low: {}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let num_pad = [
            ['7', '8', '9'],
            ['4', '5', '6'],
            ['1', '2', '3'],
            [' ', '0', 'A'],
        ];
        let dir_pad = [
            [' ', ' ', ' '],
            [' ', ' ', ' '],
            [' ', '^', 'A'],
            ['<', 'v', '>'],
        ];
        let num_lut = get_lut(&num_pad);
        let dir_lut = get_lut(&dir_pad);
        let ctx = Context {
            num_pad,
            num_lut,
            dir_pad,
            dir_lut
        };

        let mut res = 0;

        for line in input.lines() {
            let code: i128 = line[..line.len()-1].parse().unwrap();
            let path = line.chars().collect_vec();
            let min_path = shortest_only_size(path, 26, true, &ctx);
            // println!("{} : {}*{}={}", line, code, min_path, code*min_path);
            res += code*min_path;
        }

        format!("{}", res)
    }
}

// #[memoize::memoize(Ignore: ctx)]
// fn expand_paths(paths: Vec<Path>, ctx: &Context) -> Vec<Path> {
//     let mut out = Vec::new();
//     for path in paths {
//         let mut expendend = expand_path(path, ctx);
//         out.append(&mut expendend);
//     }
//     out
// }

// #[memoize(Ignore: ctx)]
// fn expand_path(path: Path, ctx: &Context) -> Vec<Path> {
//     let mut out = Vec::new();
//     let mut previous = 'A';
//     for c in path {
//         let mut dir_kb_1_paths = get_paths_from_one_lut(previous, c, false, &ctx.dir_lut);
//         if out.is_empty() {
//             out.append(&mut dir_kb_1_paths);
//         } else {
//             let mut new_paths = Vec::with_capacity(out.len()*dir_kb_1_paths.len());
//             for path in out.iter_mut() {
//                 for append_path in dir_kb_1_paths.iter() {
//                     let mut np = path.clone();
//                     np.extend_from_slice(append_path.as_slice());
//                     new_paths.push(np)
//                 }
//             }
//             out = new_paths;
//         }
//         previous = c;
//     }
//     out
// }

// #[memoize::memoize(Ignore: ctx)]
// fn get_min_path(from: char, to: char, ctx: &Context) -> Path {

//     let num_kb_paths = get_paths_from_one_lut(from, to, true, &ctx.num_lut);
//     let paths1 = expand_paths(num_kb_paths, ctx);
//     let paths2 = expand_paths(paths1, ctx);
//     // let paths3 = expand_paths(paths2, ctx);

    
//     let path = paths2.iter().min_by(|a,b| a.len().cmp(&b.len()));
//     let min_path_len = path.unwrap().len();
    
//     let n_min_paths = paths2.iter().filter(|x| x.len() <= min_path_len).count();
//     println!("{} -> {} : {} paths, {} min", from, to, paths2.len(), n_min_paths);

//     let path = paths2.into_iter().min_by(|a,b| a.len().cmp(&b.len()));
//     path.unwrap()
// }

#[memoize(Ignore: ctx)]
fn get_paths_from_one_lut(from: char, to: char, is_num: bool, ctx: &Context) -> Vec<Path> {
    let lut = if is_num { &ctx.num_lut } else { &ctx.dir_lut };
    let pad = if is_num { &ctx.num_pad } else { &ctx.dir_pad };
    let p0 = lut[&from];
    let p1 = lut[&to];
    let dx = p1.0 - p0.0;
    let dy = p1.1 - p0.1;
    let xdir = if dx >= 0 {
        Dir::Right
    } else {
        Dir::Left
    };
    let ydir = if dy >= 0 {
        Dir::Down
    } else {
        Dir::Up
    };
    let mut hor = vec![xdir; dx.abs() as usize];
    let ver = vec![ydir; dy.abs() as usize];
    hor.extend(ver.iter());
    let l = hor.len();
    let mut out = hor.into_iter().permutations(l).collect_vec();
    out.sort();
    out.dedup();
    
    let out = out.into_iter().filter_map(move | m| {
        let (mut x, mut y) = p0;
        let mut res = Vec::new();
        for d in m.iter() {
            let (dx, dy) = d.delta();
            x += dx;
            y += dy;
            let k = pad[y as usize][x as usize];
            if k == ' ' {
                return None;
            }
            res.push(d.encode())
        }
        res.push('A');
        Some(res)
    }).collect_vec();

    out
}

#[memoize(Ignore: ctx)]
fn shortest(path: Path, depth: i32, num_kb: bool, ctx: &Context) -> Path {
    if depth == 0 {
        return path;
    }

    let mut previous = 'A';
    let mut full_path = Vec::new();
    let mut short_path = Vec::new();
    for c in path {
        let mut size = usize::MAX;
        let np = get_paths_from_one_lut(previous, c, num_kb, ctx);
        for new_path in np {
            let expended_path = shortest(new_path, depth-1, false, ctx);
            if expended_path.len() < size {
                size = expended_path.len() ;
                short_path = expended_path;
            }
        }
        full_path.append(&mut short_path);
        previous = c;
    }
    full_path
}

#[memoize(Ignore: ctx)]
fn shortest_only_size(path: Path, depth: i32, num_kb: bool, ctx: &Context) -> i128 {
    if depth == 0 {
        return path.len() as i128;
    }

    let mut previous = 'A';
    let mut res = 0;
    for c in path {
        let mut size = i128::MAX;
        let np = get_paths_from_one_lut(previous, c, num_kb, ctx);
        for new_path in np {
            let new_size = shortest_only_size(new_path, depth-1, false, ctx);
            if new_size < size {
                size = new_size ;
            }
        }
        res += size;
        previous = c;
    }
    res
}