use itertools::Itertools;
use pathfinding::prelude::{astar, astar_bag_collect};

use crate::common::Day;

pub struct Day16;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32, i32, i32);
// x, y, dx, dy

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }
  
    fn successors(&self, map: &Map) -> Vec<(Pos, u32)> {
        let &Pos(x, y, dx, dy) = self;
        let dirs = [(0,1), (1,0), (0,-1), (-1,0)];

        let mut sucs = Vec::with_capacity(4);
        for (ndx, ndy) in dirs.iter().rev() {
            let nx = x + ndx;
            let ny = y + ndy;
            if nx >= 0 && nx < map.w as i32 && ny >= 0 && ny < map.h as i32 && map.m[ny as usize][nx as usize] != '#' {
                let cost = if dx == *ndx && dy == *ndy {
                    1
                } else {
                    1001
                };
                sucs.push((Pos(nx, ny, *ndx, *ndy), cost));
            }
        }
        sucs
    }
  }

#[allow(unused)]
impl Day for Day16 {
    fn solve_part1(&self, input: &str) -> String {
        let map = input.lines().map(|l | l.chars().collect_vec()).collect_vec();
        let mut start = (0, 0);
        let mut end = (0, 0);
        'outer: for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == 'S' {
                    start = (x, y);
                } else if *c == 'E' {
                    end = (x, y);
                }

                if start.0 != 0 && end.0 != 0 {
                    break 'outer;
                }
            }
        }
        let h = map.len() as i64;
        let w = map[0].len() as i64;

        let mut map = Map {
            m: map,
            h: h,
            w: w,
            start: start,
            end: end
        };

        let mut map2 = map.clone();

        // Ca marche masi c'est long (10s), j'optimeserai ça un jour peut etre
        // let res1 = astar_custom(&map);
        // mark_path(&mut map, &res1.as_ref().unwrap());


        let goal = Pos(map.end.0 as i32, map.end.1 as i32, 0, 0);
        let start = Pos(map.start.0 as i32, map.start.1 as i32, 1, 0);
        let result = astar(&start, |p| p.successors(&map2), |p| p.distance(&goal),
                   |p| p.0 == goal.0 && p.1 == goal.1);

        mark_path_pos(&mut map2, &result.as_ref().unwrap().0);
        // _print_map(&map.m);

        for (l1, l2) in map.m.iter().zip(map2.m.iter()) {
            for (c1, c2) in l1.iter().zip(l2.iter()) {
                if c1 == c2 {
                    print!("{}", c1);
                } else {
                    print!("\x1b[31m{}\x1b[0m", c1);
                }
            }
            println!("");
        }

        format!("{} = 85396, custom = {:?}", result.unwrap().1, "Commented because flemme of optimizing sorry")
    }

    fn solve_part2(&self, input: &str) -> String {

        let map = input.lines().map(|l | l.chars().collect_vec()).collect_vec();
        let mut start = (0, 0);
        let mut end = (0, 0);
        'outer: for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == 'S' {
                    start = (x, y);
                } else if *c == 'E' {
                    end = (x, y);
                }

                if start.0 != 0 && end.0 != 0 {
                    break 'outer;
                }
            }
        }
        let h = map.len() as i64;
        let w = map[0].len() as i64;

        let mut map = Map {
            m: map,
            h: h,
            w: w,
            start: start,
            end: end
        };

        let goal = Pos(map.end.0 as i32, map.end.1 as i32, 0, 0);
        let start = Pos(map.start.0 as i32, map.start.1 as i32, 1, 0);
        let result = astar_bag_collect(&start, |p| p.successors(&map), |p| p.distance(&goal),
                   |p| p.0 == goal.0 && p.1 == goal.1).unwrap();
        
        let mut a = result.0.iter().flatten().collect_vec();
        a.sort();
        a.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);


        format!("{}", a.len())
    }
}

#[derive(Debug, Clone)]
struct Map {
    m: Vec<Vec<char>>,
    h: i64,
    w: i64,
    start: (usize, usize),
    end: (usize, usize)
}
#[allow(unused)]
#[derive(Debug, Clone)]
struct Node {
    x: usize,
    y: usize,
    dir: (i64, i64),
    f: i64,
    g: i64,
    h: i64,
    parents: Vec<Node>
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.dir == other.dir
    }
}

#[allow(unused)]
impl Node {
    fn cp_no_parents(&self) -> Node {
        Node { x: self.x, y: self.y, dir: self.dir, f: self.f, g: self.g, h: self.h, parents: Vec::new() }
    }
    fn new(map: &Map, x: usize, y: usize, parent: Option<&Node>, dir: (i64, i64)) -> Node {
        // let mut parents = String::new();
        let mut parents;
        let g = match parent {
            Some(p) => {
                parents = p.parents.clone();
                parents.push(p.cp_no_parents());
                if dir == p.dir {
                    p.g + 1
                } else {
                    p.g + 1 + 1000
                }
            },
            None => {
                parents = Vec::with_capacity(50);
                0
            }
        };
        let h = distance((x, y), map.end) as i64;
        Node { x, y, dir, f: g+h, g, h , parents}
    }
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}
#[allow(unused)]
fn astar_custom(map: &Map) -> Option<Node> {
    let mut open: Vec<Node> = Vec::with_capacity(512);
    let mut close: Vec<Node> = Vec::with_capacity(512);

    let dirs = [(0,1), (1,0), (0,-1), (-1,0)];
    open.push(Node::new(&map, map.start.0, map.start.1, None, (1, 0)));
    while open.len() != 0 {
        let (idx, cur) = open.iter().enumerate().min_by(|(_, a), (_, b) | a.f.cmp(&b.f)).unwrap();
        if cur.x == map.end.0 && cur.y == map.end.1 {
            println!("Found the end! {:?}", &cur);
            // mark_path(&mut map, cur);
            // _print_map(&map.m);
            return Some(cur.clone());
        }
        let cur = open.swap_remove(idx);

        'dirs: for (dx, dy) in dirs.iter().rev() {
            let nx = cur.x as i64 + dx;
            let ny = cur.y as i64 + dy;
            if nx >= 0 && nx < map.w && ny >= 0 && ny < map.h && map.m[ny as usize][nx as usize] != '#' {
                let node = Node::new(&map, nx as usize, ny as usize, Some(&cur), (*dx, *dy));
                for (_, n) in open.iter().enumerate() {
                    if n.x == node.x && n.y == node.y && n.dir == node.dir {
                    // if n == &node {
                        if n.f < node.f {
                            // skipping this node
                            continue 'dirs;
                        }
                        break;
                    }
                }

                for (_, n) in close.iter().enumerate() {
                    if n.x == node.x && n.y == node.y && n.dir == node.dir {
                    // if n == &node {
                        if n.f < node.f {
                            // skipping this node
                            continue 'dirs;
                        }
                        break;
                    }
                }

                open.push(node);

            }
        }

        close.push(cur);
    }
    return None;
}

#[allow(unused)]
fn mark_path(map: &mut Map, node: &Node) {
    for n in &node.parents {
        let dir = match n.dir {
            (0, -1) => '^',
            (1, 0) => '>',
            (-1, 0) => '<',
            (0, 1) => 'v',
            _ => panic!("wrong direction :("),
        };
        map.m[n.y as usize][n.x as usize] = dir;
    }
}

#[allow(unused)]
fn mark_path_pos(map: &mut Map, nodes: &Vec<Pos>) {
    for Pos(x, y, dx, dy) in nodes.iter() {
        let dir = match (dx, dy) {
            (0, -1) => '^',
            (1, 0) => '>',
            (-1, 0) => '<',
            (0, 1) => 'v',
            _ => panic!("wrong direction :("),
        };
        map.m[*y as usize][*x as usize] = dir;
    }
}

fn _print_map(map: &Vec<Vec<char>>) {
    for line in map {
        let s: String = line.iter().collect();
        println!("{}", s);
    }
}