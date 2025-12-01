use std::ops::BitAnd;

use itertools::Itertools;

use crate::common::Day;

pub struct Day15;

#[allow(unused)]
impl Day for Day15 {
    fn solve_part1(&self, input: &str) -> String {
        let mut input_parts = input.split("\n\n");
        let map_input = input_parts.next().unwrap();
        let path_input = input_parts.next().unwrap();

        let mut map = map_input.lines().map(|l | l.chars().collect_vec()).collect_vec();
        let path = path_input.chars().filter(|c| "v<>^".contains(*c)).collect_vec();
        let mut pos: (i32, i32) = (0,0);
        'outer: for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == '@' {
                    pos = (x as i32, y as i32);
                    break 'outer;
                }
            }
        }
        // println!("Initial state");
        // print_map(&map);
        for (n_dir, dir) in path.iter().enumerate() {
            // println!("Iteration {} : {}", n_dir, dir);
            let dir: (i32, i32) = match dir {
                '^' => (0, -1),
                '>' => (1, 0),
                '<' => (-1, 0),
                'v' => (0, 1),
                _ => panic!("wrong direction :("),
            };

            match map[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] {
                '#' => {},
                '.' => {
                    map[pos.1 as usize][pos.0 as usize] = '.';
                    pos = (pos.0 + dir.0, pos.1 + dir.1);
                    map[pos.1 as usize][pos.0 as usize] = '@';
                },
                'O' => {

                    let mut npos = (pos.0 + 2*dir.0, pos.1 + 2*dir.1);
                    let mut olen = 1;
                    while map[npos.1 as usize][npos.0 as usize] == 'O' {
                        olen += 1;
                        npos = (npos.0 + dir.0, npos.1 + dir.1);
                    }
                    match map[npos.1 as usize][npos.0 as usize] {
                        '#' => {}, // do nothing, boxes are alreadystacked against the wall
                        '.' => { // move boxes
                            for i in 0..olen {
                                map[(npos.1 - i*dir.1) as usize][(npos.0  - i*dir.0) as usize] = 'O';
                            }
                            map[pos.1 as usize][pos.0 as usize] = '.';
                            pos = (pos.0 + dir.0, pos.1 + dir.1);
                            map[pos.1 as usize][pos.0 as usize] = '@';
                        },
                        any @ _ => panic!("Should be # or . but was {}", any)
                    }

                },
                any @ _ => panic!("Unknown state : {}", any)
            }
            // print_map(&map);
        }

        // println!("{:?}\n{:?}", &map, &path);

        let mut res = 0;

        for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == 'O' {
                    res += 100*y + x;
                }
            }
        }

        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut input_parts = input.split("\n\n");
        let map_input = input_parts.next().unwrap();
        let path_input = input_parts.next().unwrap();

        let mut map = map_input.lines().map(
            |l | l.chars().map(
                |c| match c {
                    'O' => vec!('[', ']'),
                    '@' => vec!('@', '.'),
                    o @ _ => vec!(o, o)
                }).flatten().collect_vec()
            ).collect_vec();
        // print_map(&map);
        let path = path_input.chars().filter(|c| "v<>^".contains(*c)).collect_vec();
        let mut pos: (i32, i32) = (0,0);
        'outer: for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == '@' {
                    pos = (x as i32, y as i32);
                    break 'outer;
                }
            }
        }
        // println!("Initial state");
        // print_map(&map);
        for (n_dir, dir) in path.iter().enumerate() {
            // println!("Iteration {} : {}", n_dir, dir);
            let dir: (i32, i32) = match dir {
                '^' => (0, -1),
                '>' => (1, 0),
                '<' => (-1, 0),
                'v' => (0, 1),
                _ => panic!("wrong direction :("),
            };

            match map[(pos.1 + dir.1) as usize][(pos.0 + dir.0) as usize] {
                '#' => {},
                '.' => {
                    map[pos.1 as usize][pos.0 as usize] = '.';
                    pos = (pos.0 + dir.0, pos.1 + dir.1);
                    map[pos.1 as usize][pos.0 as usize] = '@';
                },
                '[' | ']' => {
                    if dir.0 == 0 {
                        move_box_vertical(&mut map, &mut pos, dir.1);
                    } else {
                        move_box_horizontal(&mut map, &mut pos, dir.0);
                    }
                },
                any @ _ => panic!("Unknown state : {}", any)
            }
            // print_map(&map);
        }

        // println!("{:?}\n{:?}", &map, &path);

        let mut res = 0;

        for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == '[' {
                    res += 100*y + x;
                }
            }
        }

        format!("{}", res)
    }
}


fn move_box_horizontal(map: &mut Vec<Vec<char>>, pos: &mut (i32, i32), dir_x: i32) {
    let mut npos = (pos.0 + 2*dir_x, pos.1);
    let mut olen = 1;

    while map[npos.1 as usize][npos.0 as usize] == '[' || map[npos.1 as usize][npos.0 as usize] == ']' {
        olen += 1;
        npos = (npos.0 + dir_x, npos.1);
    }
    match map[npos.1 as usize][npos.0 as usize] {
        '#' => {}, // do nothing, boxes are alreadystacked against the wall
        '.' => { // move boxes
            for i in 0..olen {
                let c = if (i%2 == 0) ^ (dir_x == 1) {
                    '['
                } else {
                    ']'
                };
                map[npos.1 as usize][(npos.0  - i*dir_x) as usize] = c;
            }
            map[pos.1 as usize][pos.0 as usize] = '.';
            pos.0 += dir_x;
            map[pos.1 as usize][pos.0 as usize] = '@';
        },
        any @ _ => panic!("Should be # or . but was {}", any)
    }
}

fn move_box_vertical(map: &mut Vec<Vec<char>>, pos: &mut (i32, i32), dir_y: i32) {
    let mut boxes = vec![];
    let movable = match map[(pos.1 + dir_y)as usize][pos.0 as usize] {
        '[' => is_box_movible(map, (pos.0, pos.1 + dir_y), dir_y, &mut boxes) && is_box_movible(map, (pos.0+1, pos.1 + dir_y), dir_y, &mut boxes),
        ']' => is_box_movible(map, (pos.0, pos.1 + dir_y), dir_y, &mut boxes) && is_box_movible(map, (pos.0-1, pos.1 + dir_y), dir_y, &mut boxes),
        any @ _ => panic!("should not be there : {}", any)
    };

    if movable {
        // println!("{:?}", &boxes);
        for (x, y, _) in boxes.iter() {
            map[*y as usize][*x as usize] = '.';
        }
        for (x, y, c) in boxes.iter() {
            map[(y + dir_y) as usize][*x as usize] = *c;
        }

        map[pos.1 as usize][pos.0 as usize] = '.';
        pos.1 += dir_y;
        map[pos.1 as usize][pos.0 as usize] = '@';
    }
    
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum Movable {
    Yes(Vec<(i32, i32)>),
    No
}

impl BitAnd for Movable {
    type Output = Movable;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::No, _) => Self::No,
            (_, Self::No) => Self::No,
            (Self::Yes(mut a), Self::Yes(mut b)) => { 
                a.append(&mut b);
                Self::Yes(a)
            },
        }
    }
}

fn is_box_movible(map: &Vec<Vec<char>>, pos: (i32, i32), dir_y: i32, boxes: &mut Vec<(i32, i32, char)>) -> bool {
    boxes.push((pos.0, pos.1, map[(pos.1)as usize][pos.0 as usize] ));
    match map[(pos.1 + dir_y)as usize][pos.0 as usize] {
        '[' => is_box_movible(map, (pos.0, pos.1 + dir_y), dir_y, boxes) && is_box_movible(map, (pos.0+1, pos.1 + dir_y), dir_y, boxes),
        ']' => is_box_movible(map, (pos.0, pos.1 + dir_y), dir_y, boxes) && is_box_movible(map, (pos.0-1, pos.1 + dir_y), dir_y, boxes),
        '.' => true,
        _ => false
    }
}

fn _print_map(map: &Vec<Vec<char>>) {
    for line in map {
        let s: String = line.iter().collect();
        println!("{}", s);
    }
}