use memoize::memoize;

use crate::common::Day;

pub struct Day11;

#[derive(Debug, Clone)]
enum StoneBlinked {
    One(i64),
    Two(i64, i64)
}

fn blink(n: i64) -> StoneBlinked {
    if n == 0 {
        StoneBlinked::One(1)
    } else {
        let n_digits = n.ilog10() +1;
        if n_digits % 2 == 0 {
            let first = n/10_i64.pow(n_digits/2);
            StoneBlinked::Two(first, n-first*10_i64.pow(n_digits/2))
        } else {
            StoneBlinked::One(n*2024)
        }
    }
}

impl Day for Day11 {
    fn solve_part1(&self, input: &str) -> String {
        let mut nbs: Vec<i64> = input.split(' ').filter_map(|n| n.trim().parse().ok()).collect();
        // dbg!(nbs);

        // println!("Blink {} : {:?}", 0, &nbs);
        for _ in 0..25 {
            let nlen = nbs.len();
            for i in 0..nlen {
                match blink(nbs[i]) {
                    StoneBlinked::One(n) => nbs[i] = n,
                    StoneBlinked::Two(a, b) => {
                        nbs[i] = a;
                        nbs.push(b);
                    }
                }
            }
            // if i < 10 {
            //     println!("Blink {} : {:?}", i+1, &nbs);
            // }
        }
        // 205104 too low
        format!("{}", nbs.len())
    }

    fn solve_part2(&self, input: &str) -> String {
        let nbs: Vec<i64> = input.split(' ').filter_map(|n| n.trim().parse().ok()).collect();
        let res: i64 = nbs.iter().map(|n| nb_rocks(*n, 0, 75)).sum();
        format!("{}", res)
    }
}

#[memoize(Capacity:20000)] // Works well starting from 500 but 20 000 makes it run under 100ms
fn nb_rocks(n: i64, step: i32, max_step: i32) -> i64 {
    if step == max_step {
        1
    } else {
        match blink(n) {
            StoneBlinked::One(n) => nb_rocks(n, step+1, max_step),
            StoneBlinked::Two(a, b) => nb_rocks(a, step+1, max_step) + nb_rocks(b, step+1, max_step)
        }
    }
}
