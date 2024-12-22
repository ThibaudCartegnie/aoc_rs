use itertools::Itertools;
use memoize::memoize;

use crate::common::Day;

pub struct Day22;


const fn _div_round(dividend: i64, divisor: i64) -> i64 {
    if dividend ^ divisor >= 0 {
        (dividend + (divisor / 2)) / divisor
    } else {
        (dividend - (divisor / 2)) / divisor
    }
}

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}


fn prune(a: i64) -> i64 {
    a % 16777216
}

fn next_secret(n: i64) -> i64 {
    let s1 = prune(mix(n, n*64));
    let s2 = prune(mix(s1, s1/32));//div_round(s1, 32)));
    prune(mix(s2, 2048*s2))
}

#[allow(unused)]
impl Day for Day22 {
    fn solve_part1(&self, input: &str) -> String {
        let mut res = 0;
        for line in input.lines() {
            let mut secret = line.parse().unwrap();
            for _ in 0..2000 {
                secret = next_secret(secret);
            }
            res += secret;
        }
        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let secrets: Vec<i64> = input.lines().map(|l| l.parse().unwrap()).collect();
        let mut secret_changes = Vec::with_capacity(secrets.len());
        let mut prices: Vec<Vec<i64>> = Vec::with_capacity(secrets.len());

        for secret in secrets.iter() {
            let mut diffs = Vec::with_capacity(2000);
            let mut banana_prices = Vec::with_capacity(2000);
            banana_prices.push(secret%10);
            let mut secret = *secret;
            let mut new_secret;
            for _ in 0..2000 {
                new_secret = next_secret(secret);
                diffs.push((new_secret%10 - secret%10));
                banana_prices.push(new_secret%10);
                secret = new_secret;
            }
            secret_changes.push(diffs);
            prices.push(banana_prices);
        }
        let mut bananas = Vec::with_capacity(secrets.len());

        println!("This next part takes about 240s on my machine (~4 minutes)");

        for monkey in secret_changes.iter() {
            for (a, b, c, d) in monkey.iter().tuple_windows::<(&i64, &i64, &i64, &i64)>() {
                let p = (*a, *b, *c, *d);
                bananas.push(test_possibility(p, &prices, &secret_changes));
            }
        }

        format!("{}", bananas.iter().max().unwrap())
    }
}

#[memoize(Ignore: prices, Ignore: diffs)]
fn test_possibility(p: (i64, i64, i64, i64), prices: &Vec<Vec<i64>>, diffs: &Vec<Vec<i64>>) -> i64 {
    let mut n_bananas = 0;

    for n in 0..prices.len() {
        for i in 3..diffs[0].len() {
            if diffs[n][i-3] == p.0 && diffs[n][i-2] == p.1 && diffs[n][i-1] == p.2 && diffs[n][i] == p.3 {
                n_bananas += prices[n][i+1];
                break;
            }
        }
    }
    n_bananas
}