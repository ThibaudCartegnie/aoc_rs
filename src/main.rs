mod common;
mod y2021;
use std::fs;

use common::{Day, Year};
use clap::Parser;


/// Advent of code implementation
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = None)]
    year: Option<usize>,

    #[arg(short, long, default_value = None)]
    day: Option<usize>,

    #[arg(short, long, default_value = None)]
    part: Option<usize>

}


fn main() {
    let args = Args::parse();

    let year = match args.year {
        Some(y) => {
            get_year(y).unwrap()
        },
        None => {
            get_year(years().pop().unwrap().to_owned()).unwrap()
        }
    };

    let day_nb = args.day.unwrap_or(year.days().pop().unwrap().to_owned());
    let day = year.get_day(day_nb).unwrap();

    let input = fs::read_to_string(format!("input/day{:02}.txt", day_nb)).unwrap();

    match args.part {
        Some(1) => println!("Result for part 1 : {}", day.solve_part1(&input)),
        Some(2) => println!("Result for part 2 : {}", day.solve_part2(&input)),
        Some(_) => panic!("Part can be 1 or 2, do not specify to run both."),
        None => day.run(&input),
    }
}

fn get_year(year: usize) -> Option<impl Year> {
    match year {
        2021 => Some(y2021::Year2021),
        _ => None
    }
}
fn years() -> Vec<usize> {
    vec!(2021)
}