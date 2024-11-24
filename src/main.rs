mod common;
mod y2021;
use std::fs;

use common::Year;
use clap::Parser;


/// Advent of code implementation
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = None)]
    year: Option<usize>,

    #[arg(short, long, default_value = None)]
    day: Option<usize>,

    #[arg(short, long, default_value = None)]
    part: Option<usize>,

    #[arg(long)]
    all: bool

}


fn main() {
    let args = Args::parse();

    if args.all {
        match args.year {
            Some(year) => {
                run_all_days(year);
            },
            None => {
                for year in years() {
                    run_all_days(year);
                }
            }
        }
    } else {
        let year_nb = args.year.unwrap_or(years().pop().unwrap().to_owned());
        let year = get_year(year_nb).unwrap();
        let day_nb = args.day.unwrap_or(year.days().pop().unwrap().to_owned());
        let day = year.get_day(day_nb).unwrap();
        let input = fs::read_to_string(format!("input/y{year_nb}/d{}.txt", day_nb)).unwrap();
        println!("Running year {}, day {}", year_nb, day_nb);
        match args.part {
            Some(1) => println!("Result for part 1 : {}", day.solve_part1(&input)),
            Some(2) => println!("Result for part 2 : {}", day.solve_part2(&input)),
            Some(_) => panic!("Part can be 1 or 2, do not specify to run both."),
            None => day.run(&input),
        }
    }

}

fn run_all_days(year: usize) {
    println!("Running all challenges from year {}", year);
    let y = get_year(year).unwrap();
    for day in y.days() {
        println!("Day {}:", day);
        let input = fs::read_to_string(format!("input/y{}/d{}.txt", year, day)).unwrap();
        y.get_day(day).unwrap().run(&input);
    }
}

fn get_year(year: usize) -> Option<Box<dyn Year>> {
    match year {
        2021 => Some(Box::new(y2021::Year2021)),
        _ => None
    }
}
fn years() -> Vec<usize> {
    vec!(2021)
}