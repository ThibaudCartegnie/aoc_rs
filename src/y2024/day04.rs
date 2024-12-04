use itertools::Itertools;

use crate::common::Day;

pub struct Day04;

fn get_char_or_default<'a>(lines: &'a Vec<&'a [u8]>, i: isize, j: isize) -> &'a u8 {
    lines.get(j as usize).unwrap_or(&"".as_bytes()).get(i as usize).unwrap_or(&0)
}

#[allow(unused)]
impl Day for Day04 {
    fn solve_part1(&self, input: &str) -> String {
        let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
        let w = lines[0].len();
        let h = lines.len();
        let mut nb_xmas = 0;
        let dirs: Vec<(isize, isize)> = vec!((1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,1), (1,-1), (-1,-1));
        let mas = "MAS".as_bytes();
        for i in 0..w {
            for j in 0..h {
                if &lines[j][i] == &('X' as u8) {
                    nb_xmas += dirs.iter()
                        .filter(|(dx, dy)|
                                mas.iter().enumerate()
                                    .all(|(n, c)| c == get_char_or_default(&lines, i as isize + dy * (n as isize+1), j as isize + dx * (n as isize+1)))
                        ).count()
                }
            }
        }
        format!("{}", nb_xmas)
    }

    fn solve_part2(&self, input: &str) -> String {
        let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
        let w = lines[0].len() as isize;
        let h = lines.len() as isize;
        let mut nb_xmas = 0;
        let dirs: Vec<u8> = vec!('M' as u8, 'M'  as u8, 'S'  as u8, 'S'  as u8);
        for i in 0..w {
            for j in 0..h {
                if get_char_or_default(&lines, i , j) == &('A' as u8) {
                    for (a, b, c, d) in dirs.iter().circular_tuple_windows::<(_, _, _,_)>() {
                        if get_char_or_default(&lines, i-1 , j-1) == a
                            && get_char_or_default(&lines, i-1 , j+1) == b
                            && get_char_or_default(&lines, i+1 , j+1) == c
                            && get_char_or_default(&lines, i+1 , j-1) == d
                        {
                            nb_xmas += 1;
                            break;
                        }
                    }
                }
            }
        }
        format!("{}", nb_xmas)
    }
}
