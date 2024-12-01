use crate::common::Day;

pub struct Day01;

#[allow(unused)]
impl Day for Day01 {
    fn solve_part1(&self, input: &str) -> String {
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();

        for line in input.lines() {
            let mut nbs = line.split("   ");
            list1.push(i32::from_str_radix(nbs.next().unwrap(), 10).unwrap());
            list2.push(i32::from_str_radix(nbs.next().unwrap(), 10).unwrap());
        }

        list1.sort();
        list2.sort();

        let res = list1.iter().zip(list2.iter()).fold(0, |acc, (n1, n2)| acc + n1.abs_diff(*n2));

        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();

        for line in input.lines() {
            let mut nbs = line.split("   ");
            list1.push(i32::from_str_radix(nbs.next().unwrap(), 10).unwrap());
            list2.push(i32::from_str_radix(nbs.next().unwrap(), 10).unwrap());
        }
        let score = list1.iter().fold(0, |acc, nb| acc + nb * list2.iter().filter(|n2| nb == *n2).count() as i32 );
        format!("{}", score)
    }
}
