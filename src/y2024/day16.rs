use itertools::Itertools;

use crate::common::Day;

pub struct Day16;

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

        let map = Map {
            m: map,
            h: h,
            w: w,
            start: start,
            end: end
        };

        let mut open: Vec<Node> = Vec::with_capacity(512);
        let mut close: Vec<Node> = Vec::with_capacity(512);

        let dirs = [(0,1), (1,0), (0,-1), (-1,0)];
        open.push(Node::new(&map, map.start.0, map.start.1, None, (1, 0)));
        while open.len() != 0 {
            let (idx, cur) = open.iter().enumerate().min_by(|(_, a), (_, b) | a.f.cmp(&b.f)).unwrap();
            if cur.x == end.0 && cur.y == end.1 {
                println!("Found the end! {:?}", &cur);
                break;
            }
            let cur = open.swap_remove(idx);

            'dirs: for (dx, dy) in dirs.iter().rev() {
                let nx = cur.x as i64 + dx;
                let ny = cur.y as i64 + dy;
                if (*dx != 0 && *dx == cur.dir.0 *-1) || (*dy != 0 && *dy == cur.dir.1*-1) {
                    continue;
                }
                if nx >= 0 && nx < map.w && ny >= 0 && ny < map.h && map.m[ny as usize][nx as usize] != '#' {
                    let node = Node::new(&map, nx as usize, ny as usize, Some(&cur), (*dx, *dy));
                    for (i, n) in open.iter().enumerate() {
                        if n.x == node.x && n.y == node.y {
                            if n.f < node.f {
                                // skipping this node
                                continue 'dirs;
                            }
                            break;
                        }
                    }

                    for (i, n) in close.iter().enumerate() {
                        if n.x == node.x && n.y == node.y {
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

        format!("")
    }

    fn solve_part2(&self, input: &str) -> String {
        format!("")
    }
}

#[derive(Debug)]
struct Map {
    m: Vec<Vec<char>>,
    h: i64,
    w: i64,
    start: (usize, usize),
    end: (usize, usize)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    dir: (i64, i64),
    f: i64,
    g: i64,
    h: i64,
}

impl Node {
    fn new(map: &Map, x: usize, y: usize, parent: Option<&Node>, dir: (i64, i64)) -> Node {
        let g = match parent {
            Some(p) => {
                if dir == p.dir {
                    p.g + 1
                } else {
                    p.g + 1 + 1000
                }
            },
            None => 0
        };
        let h = distance((x, y), map.end) as i64;
        Node { x: x, y: y, dir: dir, f: g+h, g: g, h: h }
    }
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}
