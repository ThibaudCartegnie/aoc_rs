mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

use crate::common::{Day, Year};

pub struct Year2024;

impl Year for Year2024 {
    fn get_day(&self, day: usize) -> Option<Box<dyn Day>> {
        match day {
            1 => Some(Box::new(day01::Day01)),
            2 => Some(Box::new(day02::Day02)),
            3 => Some(Box::new(day03::Day03)),
            4 => Some(Box::new(day04::Day04)),
            5 => Some(Box::new(day05::Day05)),
            6 => Some(Box::new(day06::Day06)),
            7 => Some(Box::new(day07::Day07)),
            8 => Some(Box::new(day08::Day08)),
            9 => Some(Box::new(day09::Day09)),
            10 => Some(Box::new(day10::Day10)),
            11 => Some(Box::new(day11::Day11)),
            12 => Some(Box::new(day12::Day12)),
//             13 => Some(day13::Day13),
//             14 => Some(day14::Day14),
//             15 => Some(day15::Day15),
//             16 => Some(day16::Day16),
//             17 => Some(day17::Day17),
//             18 => Some(day18::Day18),
//             19 => Some(day19::Day19),
//             20 => Some(day20::Day20),
//             21 => Some(day21::Day21),
//             22 => Some(day22::Day22),
//             23 => Some(day23::Day23),
//             24 => Some(day24::Day24),
//             25 => Some(day25::Day25),
            _ => None,
        }
    }

    fn days(&self) -> Vec<usize> {
        vec!(
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
            // 13,
            // 14,
            // 15,
            // 16,
            // 17,
            // 18,
            // 19,
            // 20,
            // 21,
            // 22,
            // 23,
            // 24,
            // 25,
        )
    }
}
