use std::{collections::{HashMap, HashSet}, hash::Hash};
use ahash::RandomState;
use itertools::Itertools;

use crate::common::Day;

pub struct Day23;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Net3<'a>(&'a str, &'a str, &'a str);

impl Net3<'_> {
    fn new<'a>(c1: &'a str, c2: &'a str, c3: &'a str) -> Net3<'a> {
        let mut v = [c1, c2, c3];
        v.sort();
        Net3(v[0], v[1], v[2])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct NetN<'a> {
    v: Vec<&'a str>
}

impl<'a> NetN<'a> {
    fn new(comps: &[&'a str]) -> Self {
        let mut v = Vec::from(comps);
        v.sort();
        NetN { v }
    }

    fn add(&mut self, c: &'a str) {
        self.v.push(c);
        self.v.sort();
    }
}

// impl PartialEq for Net3<'_> {
//     fn eq(&self, other: &Self) -> bool {
//         (self.0 == other.0 || self.0 == other.1 || self.0 == other.2) 
//         && (self.1 == other.0 || self.1 == other.1 || self.1 == other.2) 
//         && (self.2 == other.0 || self.2 == other.1 || self.2 == other.2) 
//     }
// }

// impl Hash for Net3<'_> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         let mut v = [self.0, self.1, self.2];
//         v.sort();
//         v[0].hash(state);
//         v[1].hash(state);
//         v[2].hash(state);
//     }
// }

#[allow(unused)]
impl Day for Day23 {
    fn solve_part1(&self, input: &str) -> String {
        let mut computers: HashMap<&str, Vec<&str>, RandomState> = HashMap::with_capacity_and_hasher(200, ahash::RandomState::new());

        for line in input.lines() {
            let (c1, c2) = line.split('-').collect_tuple().unwrap();
            if computers.contains_key(c1) {
                computers.get_mut(c1).unwrap().push(c2);
            } else {
                computers.insert(c1, vec![c2]);
            }

            if computers.contains_key(c2) {
                computers.get_mut(c2).unwrap().push(c1);
            } else {
                computers.insert(c2, vec![c1]);
            }
        }

        let mut threeways: HashSet<Net3> = HashSet::with_capacity(200);
        for (c1, cons) in computers.iter() {
            for c2 in cons {
                let cons2 = &computers[c2];
                let network: Vec<&&str> = cons.iter().filter(|c3| cons2.contains(c3)).collect();
                for c3 in network {
                    threeways.insert(Net3::new(c1, c2, c3));
                }
            }
        }

        format!("{}", threeways.iter().filter(|n| n.0.starts_with('t') || n.1.starts_with('t') || n.2.starts_with('t')).count())
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut computers: HashMap<&str, HashSet<&str, RandomState>, RandomState> = HashMap::with_capacity_and_hasher(200, RandomState::new());
        for line in input.lines() {
            let (c1, c2) = line.split('-').collect_tuple().unwrap();
            if computers.contains_key(c1) {
                computers.get_mut(c1).unwrap().insert(c2);
            } else {
                let mut set = HashSet::with_hasher(RandomState::new());
                set.insert(c2);
                computers.insert(c1, set);
            }

            if computers.contains_key(c2) {
                computers.get_mut(c2).unwrap().insert(c1);
            } else {
                let mut set = HashSet::with_hasher(RandomState::new());
                set.insert(c1);
                computers.insert(c2, set);
            }
        }

        // idea : find all 3-comps networks, iterate on them to see which ones are actually 4-comps, then 5, etc until there is one group left
        let mut net3s: HashSet<NetN, RandomState> = HashSet::with_capacity_and_hasher(200, RandomState::new());
        for (c1, cons) in computers.iter() {
            for c2 in cons {
                let cons2 = &computers[c2];
                let network: Vec<&&str> = cons.iter().filter(|c3| cons2.contains(*c3)).collect();
                for c3 in network {
                    net3s.insert(NetN::new(&[c1, c2, c3]));
                }
            }
        }

        let mut networks = net3s.into_iter().collect_vec();
        let mut n_len = networks.len();
        let mut max_len = 3;
        let mut itertion = 0;
        while n_len > 1 {
            itertion+= 1;
            for (i, n) in networks.iter_mut().enumerate() {
                for (c, cons) in computers.iter() {
                    if n.v.iter().all(|cn| cons.contains(*cn)) {
                        n.add(c);
                        break;
                    }
                }
            }
            n_len = networks.len();
            for i in (0..n_len).rev() {
                if networks[i].v.len() <= max_len {
                    networks.swap_remove(i);
                }
            }
            networks.sort();
            networks.dedup();
            n_len = networks.len();
            max_len += 1;
        }
        format!("{} {}", networks[0].v.join(","), itertion)
    }
}
