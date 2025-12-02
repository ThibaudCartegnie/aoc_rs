use crate::common::Day;
use itertools::Itertools;

pub struct Day02;

#[allow(unused)]
impl Day for Day02 {
    fn solve_part1(&self, input: &str) -> String {
        let mut invalids = Vec::new();
        for (l, h) in input.trim().split(',').map(|r| r.split('-').map(|n| {i64::from_str_radix(n, 10).unwrap()}).collect_tuple().unwrap()) {
            for n in l..=h {
                let size = n.ilog10() + 1;
                let base = 10i64.pow(size/2);
                if size % 2 == 0 && n/base == n%base {
                    invalids.push(n);
                }
            }
        }
        // 863 too low <= forgot to add anwsers lol
        format!("{}", invalids.iter().sum::<i64>())
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut invalids = Vec::new();
        for (l, h) in input.trim().split(',').map(|r| r.split('-').map(|n| {i64::from_str_radix(n, 10).unwrap()}).collect_tuple().unwrap()) {
            for n in l..=h {
                if is_id_invalid_p2(n){
                    invalids.push(n);
                }
            }
        }
        
        // 58963527544 too high
        // 58962153807 too high :(
        format!("{}", invalids.iter().sum::<i64>())
    }
}

fn is_id_invalid_p2(n: i64) -> bool {
    let size = n.ilog10() + 1;
    for i in 1..=size/2 {
        let base = 10i64.pow(i);
        let rightmost = n%base;
        let mut comparison = rightmost;
        let mut comparison_base = base;
        // true_size_comp is used to handle cases where the repeating pattern starts with 0
        //i.e. 101 is valid but with this method, it compares it to 0101 (which would be invalid if we counted beforehand zeros)
        let mut true_size_comp = i;
        while comparison == n%comparison_base {
            comparison += rightmost*comparison_base;
            true_size_comp += i;
            // println!("i {}, n {},size {}, comp size {}, comp {}, base {}, rightmost {}, comp base {}", i, n, size, true_size_comp, comparison, base, rightmost, comparison_base);
            if comparison == n && size == true_size_comp {
                return true;
            }
            comparison_base *= base;
            // println!("i {}, n {}, comp {}, check {}, base {}, rightmost {}, comp base {}", i, n, comparison, comparison == n%comparison_base , base, rightmost, comparison_base);
        }
    }
    false
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_p2() {
        assert_eq!(is_id_invalid_p2(20202), false);
        assert_eq!(is_id_invalid_p2(1001001), false);
        assert_eq!(is_id_invalid_p2(101010), true);
    }
}
