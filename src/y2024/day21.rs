use std::{collections::HashMap, usize};

use itertools::Itertools;

use crate::common::Day;

#[derive(Debug, Clone, Copy)]
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
            let mut min_path_size = 0;
            let mut full_path = Vec::new();
            let mut previous = 'A';
            for c in line.chars() {
                let mut path = get_min_path(previous, c, &ctx);
                min_path_size += path.len();
                full_path.append(&mut path);
                // println!("{}->{}: {:?}", previous, c, get_paths_from_one_lut(previous, c, &ctx.num_lut));
                previous = c;
            }
            println!("{} : {}*{} (or {})={}, Path : {}", line, code, min_path_size, full_path.len(), code*min_path_size, full_path.iter().join(""));
            res += code*min_path_size;
        }

        format!("158224 too low: {}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        format!("")
    }
}

#[memoize::memoize(Ignore: ctx)]
fn expand_paths(paths: Vec<Path>, ctx: &Context) -> Vec<Path> {
    let mut out = Vec::new();
    for path in paths {
        let mut expendend = expand_path(path, ctx);
        out.append(&mut expendend);
    }
    out
}

#[memoize::memoize(Ignore: ctx)]
fn expand_path(path: Path, ctx: &Context) -> Vec<Path> {
    let mut out = Vec::new();
    let mut previous = 'A';
    for c in path {
        let mut dir_kb_1_paths = get_paths_from_one_lut(previous, c, false, &ctx.dir_lut);
        if out.is_empty() {
            out.append(&mut dir_kb_1_paths);
        } else {
            let mut new_paths = Vec::with_capacity(out.len()*dir_kb_1_paths.len());
            for path in out.iter_mut() {
                for append_path in dir_kb_1_paths.iter() {
                    let mut np = path.clone();
                    np.extend_from_slice(append_path.as_slice());
                    new_paths.push(np)
                }
            }
            out = new_paths;
        }
        previous = c;
    }
    out
}

#[memoize::memoize(Ignore: ctx)]
fn get_min_path(from: char, to: char, ctx: &Context) -> Path {

    let num_kb_paths = get_paths_from_one_lut(from, to, true, &ctx.num_lut);
    let paths1 = expand_paths(num_kb_paths, ctx);
    let paths2 = expand_paths(paths1, ctx);
    // let paths3 = expand_paths(paths2, ctx);

    
    let path = paths2.iter().min_by(|a,b| a.len().cmp(&b.len()));
    let min_path_len = path.unwrap().len();
    
    let n_min_paths = paths2.iter().filter(|x| x.len() <= min_path_len).count();
    println!("{} -> {} : {} paths, {} min", from, to, paths2.len(), n_min_paths);

    let path = paths2.into_iter().min_by(|a,b| a.len().cmp(&b.len()));
    path.unwrap()
}

#[memoize::memoize(Ignore: lut)]
fn get_paths_from_one_lut(from: char, to: char, is_num: bool, lut: &Lut) -> Vec<Path> {
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
    let mut hor = vec![xdir.encode(); dx.abs() as usize];
    let ver = vec![ydir.encode(); dy.abs() as usize];
    hor.extend(ver.iter());
    let l = hor.len();
    let mut out = hor.into_iter().permutations(l).collect_vec();
    out.sort();
    out.dedup();
    for p in out.iter_mut() {
        p.push('A');
    }
    out
}