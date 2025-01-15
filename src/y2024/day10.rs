use crate::common::Day;

pub struct Day10;

fn parse_and_put_zeros_aside(c: &str, pos: (usize, usize), zeros: &mut Vec<(usize, usize)>) -> u8 {
    let n = c.parse().unwrap();
    if n == 0 {
        zeros.push(pos);
    }
    n
}

fn walk_until_9(pos: (usize, usize), map: &Vec<Vec<u8>>, visited: &mut Option<&mut Vec<(usize, usize)>>) -> i32 {
    if map[pos.1][pos.0] == 9 {
        match visited {
            None => return 1,
            Some(visited) => {
                if visited.contains(&pos) {
                    return 0; // already seen on this trailhead
                } else {
                    visited.push(pos);
                    return 1;
                }
            }
        }
    }

    let h = map.len() as isize;
    let w = map[0].len() as isize;

    let dirs = [(0,1), (1,0), (0,-1), (-1,0)];
    let mut res = 0;
    for (dx, dy) in dirs.iter() {
        let new_pos = (pos.0 as isize + dx, pos.1 as isize + dy);
        if new_pos.0 >= 0 && new_pos.0 < w && new_pos.1 >= 0 && new_pos.1 < h {
            if map[new_pos.1 as usize][new_pos.0 as usize] == map[pos.1][pos.0] + 1 {
                res += walk_until_9((new_pos.0 as usize, new_pos.1 as usize), map, visited);
            }
        }
    }
    res
}


#[allow(unused)]
impl Day for Day10 {
    fn solve_part1(&self, input: &str) -> String {
        let mut zeros = Vec::with_capacity(512);
        let lines: Vec<Vec<u8>> = input.lines()
                .enumerate()
                .map(
                    |(y, l)| l.split("")
                        .filter(|c| c.len() != 0)
                        .enumerate()
                        .map(|(x, n)| parse_and_put_zeros_aside(n, (x, y), &mut zeros))
                        .collect())
                .collect();
        let mut res = 0;
        for (x, y) in zeros.iter() {
            let mut visited = Vec::with_capacity(256);
            res += walk_until_9((*x, *y), &lines, &mut Some(&mut visited));
        }
        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut zeros = Vec::with_capacity(512);
        let lines: Vec<Vec<u8>> = input.lines()
                .enumerate()
                .map(
                    |(y, l)| l.split("")
                        .filter(|c| c.len() != 0)
                        .enumerate()
                        .map(|(x, n)| parse_and_put_zeros_aside(n, (x, y), &mut zeros))
                        .collect())
                .collect();
        // println!("bru");
        let mut res = 0;
        for (x, y) in zeros.iter() {
            res += walk_until_9((*x, *y), &lines, &mut None);
        }
        format!("{}", res)
    }
}
