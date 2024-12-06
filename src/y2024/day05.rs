use itertools::Itertools;

use crate::common::Day;

pub struct Day05;

#[derive(Debug, Clone, Copy)]
enum Respected {
    Yes,
    No(usize, usize)
}

impl Into<bool> for Respected {
    fn into(self) -> bool {
        match self {
            Self::Yes => true,
            Self::No(_,_) => false
        }
    }
}

fn is_contraint_respected(update: &Vec<i32>, constraint: &(i32, i32)) -> Respected {
    let mut idx = (None, None);
    for (i, val) in update.iter().enumerate() {
        if val == &constraint.0 {
            idx.0 = Some(i);
        }
        if val == &constraint.1 {
            idx.1 = Some(i);
        }
    }
    match idx {
        (None,_) => Respected::Yes,
        (_, None) => Respected::Yes,
        (Some(a), Some(b)) => {if a < b  { Respected::Yes  } else { Respected::No(a, b)}},
    }
    // let c = idx.0.unwrap_or(0) < idx.1.unwrap_or(usize::MAX);
    // dbg!(update);
    // dbg!(constraint);
    // dbg!(idx);
    // dbg!(c);
    // c
}

#[allow(unused)]
impl Day for Day05 {
    fn solve_part1(&self, input: &str) -> String {
        let mut switch = false;
        let mut updates: Vec<Vec<i32>> = Vec::new();
        let mut constraints: Vec<(i32, i32)> = Vec::new();

        for line in input.lines() {
            if line.len() == 0 {
                switch = true;
                continue;
            }
            if switch {
                updates.push(line.split(',').map(|n|n.parse().unwrap()).collect());
            } else {
                constraints.push(line.split('|').map(|n|n.parse::<i32>().unwrap()).next_tuple().unwrap());
            }
        }
        let mut res = 0;
        let mut n=0;
        for update in updates.iter() {
            if constraints.iter().all(|c| is_contraint_respected(update, c).into()) {
                // println!("{}: {}: {:?}", (update.len()/2) +1, update.len(), update);
                res += update[(update.len()/2)];
                n += 1;
            }
        }
        format!("{}/{}, {}", n, updates.len(), res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut switch = false;
        let mut updates: Vec<Vec<i32>> = Vec::new();
        let mut constraints: Vec<(i32, i32)> = Vec::new();

        for line in input.lines() {
            if line.len() == 0 {
                switch = true;
                continue;
            }
            if switch {
                updates.push(line.split(',').map(|n|n.parse().unwrap()).collect());
            } else {
                constraints.push(line.split('|').map(|n|n.parse::<i32>().unwrap()).next_tuple().unwrap());
            }
        }
        let mut res = 0;
        let mut n=0;
        for update in updates.iter() {
            if ! constraints.iter().all(|c| is_contraint_respected(update, c).into()) {
                n += 1;
                // println!("Update order to be changed: {:?}", update);
                let mut new_update = update.clone();
                let nb_contraints = constraints.len();
                while ! constraints.iter().all(|c| is_contraint_respected(&new_update, c).into()) {
                    let mut idx_c = 0;
                    let mut change = false;
                    while ! change && idx_c < nb_contraints {
                        if let Respected::No(a,b) = is_contraint_respected(&new_update, &constraints[idx_c]) {
                            new_update.swap(a, b);
                            change = true;
                        }
                        idx_c += 1;
                    }
                }
                res += new_update[(new_update.len()/2)]
            }
        }
        format!("{}/{}, {}", n, updates.len(), res)
    }
}
