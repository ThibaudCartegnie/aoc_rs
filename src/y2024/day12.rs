use std::collections::HashSet;

use crate::common::Day;

pub struct Day12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}


#[derive(Debug, Clone, Default)]
struct Shape {
    val: u8,
    perimeter: usize,
    fences: Vec<(isize, isize, Dir)>,
    points: Vec<(isize, isize)>
}

impl Shape {
    fn new(val: u8) -> Shape {
        Shape {
            val,
            perimeter: 0,
            fences: Vec::with_capacity(10),
            points: Vec::with_capacity(10)
        }
    }
    fn add_to_shape(&mut self, x: usize, y: usize, map: &Vec<Vec<u8>>, visited: &mut HashSet<(usize, usize)>) {
        self.points.push((x as isize, y as isize));
        visited.insert((x, y));
        let dirs = [(0,1, Dir::Up), (1,0, Dir::Right), (0,-1, Dir::Down), (-1,0, Dir::Left)];
        let h = map.len() as isize;
        let w = map[0].len() as isize;
        let ix = x as isize;
        let iy = y as isize;
        for (dx, dy, dir) in dirs {
            let new_x = ix+dx;
            let new_y = iy+dy;
            if self.points.contains(&(new_x, new_y)) {
                continue;
            }
            if (0..h).contains(&new_y) && (0..w).contains(&new_x) && map[new_y as usize][new_x as usize] == self.val {
                self.add_to_shape(new_x as usize, new_y as usize, map, visited);
            } else {
                self.fences.push((ix, iy, dir));
                self.perimeter += 1;
            }
        }
    }

    fn area(&self) -> usize {
        self.points.len()
    }

    fn faces(&self) -> usize {
        let mut faces = Vec::with_capacity(self.fences.len());
        for (fx, fy, dir) in self.fences.iter() {
            match dir {
                //                              start, end, reference, direction
                Dir::Up | Dir::Down => faces.push((*fx, *fx, *fy, *dir)),
                Dir::Left | Dir:: Right => faces.push((*fy, *fy, *fx, *dir))
            }
        }
        let mut nbchanges = 1;

        while nbchanges > 0 {
            // merge fences between and iterate multiple times until there are no more changes
            let n = faces.len();
            let mut to_delete = Vec::with_capacity(n);
            for i in 0..n {
                let (s, e, r, dir) = faces[i];
                for j in 0..n {
                    let (s2, e2, r2, d2) = faces[j];
                    if dir == d2 && r == r2 {
                        if s2 == e + 1 {
                            faces[j] = (s, e2, r2, d2);
                            to_delete.push(i);
                            break;
                        }
                        if e2 + 1 == s {
                            faces[j] = (s2, e, r2, d2);
                            to_delete.push(i);
                            break;
                        }
                    }
                }
            }
            nbchanges = to_delete.len();
            for i in to_delete.iter().rev() {
                faces.swap_remove(*i);
            }
        }

        // let v = self.val as char;
        // println!("{} faces : {} fences: {:?}, faces: {:?}", v, faces.len(), &self.fences, &faces);
        faces.len()
    }
}

#[allow(unused)]
impl Day for Day12 {
    fn solve_part1(&self, input: &str) -> String {
        let map: Vec<Vec<u8>> = input.lines().map(|l|l.bytes().collect()).collect();
        let h = map.len();
        let w = map[0].len();
        let mut shapes = Vec::with_capacity(512);
        let mut visited = HashSet::with_capacity(h*w);

        for x in 0..w {
            for y in 0..h {
                let pos = (x,y);
                if visited.contains(&pos) {
                    continue;
                }
                let mut shape = Shape::new(map[y][x]);
                shape.add_to_shape(x, y, &map, &mut visited);
                shapes.push(shape);
            }
        }

        let res: usize = shapes.iter().map(|s| s.perimeter * s.area()).sum();

        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let map: Vec<Vec<u8>> = input.lines().map(|l|l.bytes().collect()).collect();
        let h = map.len();
        let w = map[0].len();
        let mut shapes = Vec::with_capacity(512);
        let mut visited = HashSet::with_capacity(h*w);

        for x in 0..w {
            for y in 0..h {
                let pos = (x,y);
                if visited.contains(&pos) {
                    continue;
                }
                let mut shape = Shape::new(map[y][x]);
                shape.add_to_shape(x, y, &map, &mut visited);
                shapes.push(shape);
            }
        }

        let res: usize = shapes.iter().map(|s| s.faces() * s.area()).sum();
        format!("{}", res)
    }
}
