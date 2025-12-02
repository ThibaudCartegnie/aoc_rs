mod common;
mod y2021;
mod y2024;
mod y2025;
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
    all: bool,

    #[arg(short, long)]
    example_if_any: bool

}


fn main() {
    let start = std::time::Instant::now();
    let args = Args::parse();

    if args.all {
        match args.year {
            Some(year) => {
                run_year(year, args.example_if_any);
            },
            None => {
                for year in years() {
                    run_year(year, args.example_if_any);
                }
            }
        }
    } else {
        let year_nb = args.year.unwrap_or(years().pop().unwrap().to_owned());
        let year = get_year(year_nb).unwrap();
        let day_nb = args.day.unwrap_or(year.days().pop().unwrap().to_owned());
        run_day(year_nb, day_nb, args.example_if_any, args.part);
    }

    let us = start.elapsed().as_micros();
    let ms = us/1000;
    let us = us - 1000*ms;

    println!("Program ran in {}ms and {}µs", ms, us);

}

fn run_day(year_nb: usize, day_nb: usize, example_if_any: bool, part: Option<usize>) {
    let year = get_year(year_nb).unwrap();
    let day = year.get_day(day_nb).unwrap();
    let input = get_input(year_nb, day_nb, example_if_any);
    println!("Running year {}, day {}", year_nb, day_nb);
    let start = std::time::Instant::now();
    match part {
        Some(1) => println!("Result for part 1 : {}", day.solve_part1(&input)),
        Some(2) => println!("Result for part 2 : {}", day.solve_part2(&input)),
        Some(_) => panic!("Part can be 1 or 2, do not specify to run both."),
        None => day.run(&input),
    }
    let us = start.elapsed().as_micros();
    let ms = us/1000;
    let us = us - 1000*ms;
    println!("Day {} from year {} ran in {}ms and {}µs",day_nb, year_nb, ms, us);
}

fn get_input(year: usize, day: usize, example_if_any: bool) -> String {
    let normal_path = format!("input/y{}/d{}.txt", year, day);
    let example_path = format!("input/y{}/d{}_ex.txt", year, day);
    let path = if example_if_any && fs::exists(&example_path).unwrap_or(false) {
        println!("Using example!");
        &example_path
    } else if fs::exists(&normal_path).unwrap_or(false) {
        &normal_path
    } else {
        download_input(year, day);
        &normal_path
    };
    fs::read_to_string(path).unwrap()
}

fn download_input(year: usize, day: usize) {
    let session_cookie = std::env::var("AOC_SESSION_COOKIE").unwrap_or_else(|e| panic!("You need to setup AOC_SESSION_COOKIE env variable to download the inputs : {}", e));
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let req = client.get(format!("https://adventofcode.com/{}/day/{}/input", year, day))
        .header("Cookie", format!("session={}", session_cookie))
        .build().unwrap();
    let resp = client.execute(req).unwrap().text().unwrap();
    if resp.starts_with("Puzzle inputs differ by user.") {
        panic!("The downloaded input is incorrect, please set a correct AOC_SESSION_COOKIE env variable");
    }

    let path = format!("input/y{}/d{}.txt", year, day);
    let _ = fs::create_dir_all(format!("input/y{}/", year));
    fs::write(path, resp).unwrap();
}

fn run_year(year: usize, example_if_any: bool) {
    println!("Running all challenges from year {}", year);
    let y = get_year(year).unwrap();
    let start = std::time::Instant::now();
    for day in y.days() {
        run_day(year, day, example_if_any, None);
    }
    let us = start.elapsed().as_micros();
    let ms = us/1000;
    let us = us - 1000*ms;
    println!("Year {} ran in {}ms and {}µs", year, ms, us);
}

fn get_year(year: usize) -> Option<Box<dyn Year>> {
    match year {
        2021 => Some(Box::new(y2021::Year2021)),
        2024 => Some(Box::new(y2024::Year2024)),
        2025 => Some(Box::new(y2025::Year2025)),
        _ => None
    }
}
fn years() -> Vec<usize> {
    vec!(2021, 2024, 2025)
}