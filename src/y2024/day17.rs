// use std::thread;

use itertools::Itertools;

use crate::common::Day;

pub struct Day17;

#[allow(unused)]
impl Day for Day17 {
    fn solve_part1(&self, input: &str) -> String {
        let mut registers = (0, 0, 0);
        let mut instructions: Vec<i64> = Vec::new();
        for line in input.lines() {
            if line.starts_with("Register A: ") {
                registers.0 = i64::from_str_radix(&line[12..], 10).unwrap();
            }
            if line.starts_with("Register B: ") {
                registers.1 = i64::from_str_radix(&line[12..], 10).unwrap();
            }
            if line.starts_with("Register C: ") {
                registers.2 = i64::from_str_radix(&line[12..], 10).unwrap();
            }
            if line.starts_with("Program: ") {
                instructions = line[9..].split(',').map(|op| i64::from_str_radix(op, 10).unwrap()).collect_vec();
            }
        }

        let output = run_program(&instructions, &mut registers);

        
        format!("{:?}, {}", &output, output.iter().map(i64::to_string).join(","))
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut registers = (0, 0, 0);
        let mut instructions: Vec<i64> = Vec::new();
        for line in input.lines() {
            if line.starts_with("Program: ") {
                instructions = line[9..].split(',').map(|op| i64::from_str_radix(op, 10).unwrap()).collect_vec();
            }
        }

        // let mut output = Vec::new();
        // let mut registers;
        // let mut a = -1;
        // let goal = vec![2,4,1];
        // while output != goal {
        //     a += 1 as i64;
        //     registers = (a, 0, 0);
        //     println!("Test {:?}", &registers);
        //     output = run_program(&instructions, &mut registers);
        // }
        // println!("Found! {}", a);

        let mut solution: Vec<i64> = Vec::new();
        
        let mut letsgong = 0;
        for n_i in (0..instructions.len()).rev() {
            for i in 0..8 {
                let mut input = i;
                for (pow, val) in solution.iter().rev().enumerate() {
                    input += val* 8i64.pow(pow as u32 + 1);
                }
                registers = (input, 0, 0);

                let output = run_program(&instructions, &mut registers);
                if output == instructions[n_i..] {
                    println!("Ok with register : {}", input);
                    letsgong = input;
                    solution.push(i);
                    break;
                }
            }
            println!("instruction n{}, sol: {:?}", n_i, solution);
        }

        // loop {
        //     let mut reg = String::new();
        //     std::io::stdin().read_line(&mut reg).unwrap();
        //     let reg: i64 = reg.trim().parse().unwrap();
        //     registers = (reg, 0, 0);
        //     let output = run_program(&instructions, &mut registers);
        //     println!("{:?}", output);
        // }
        // return "".into();

        // let start: i64 = 51000000000;
        // let n_thread = 1; //thread::available_parallelism().unwrap().get();

        // let mut threads = Vec::new();
        // for i in 0..n_thread {
        //     let instructions = instructions.clone();
        //     threads.push(thread::spawn(move || {
        //         let mut output = Vec::new();
        //         let mut registers;
        //         let mut a = i as i64 - n_thread as i64;
        //         while output != instructions {
        //             a += n_thread as i64;
        //             if a % 1_000_000_000 == 0 {
        //                 println!("Attempt {}", a);
        //             }
        //             registers = (a, 0, 0);
        //             output = run_program(&instructions, &mut registers);
        //         }
        //         println!("FOUND IT {}", a);
        //         println!("FOUND IT {}", a);
        //         println!("FOUND IT {}", a);
        //         println!("FOUND IT {}", a);
        //         println!("FOUND IT {}", a);
        //         println!("FOUND IT {}", a);
        //         println!("FOUND IT {}", a);
        //     }));
        // }

        // for t in threads {
        //     t.join();
        // }

        format!("LET GO LA TEAM EN FAIT {}", letsgong)
    }
}


fn combo(n: i64, registers: &(i64, i64, i64)) -> i64 {
    match n {
        0..=3 => n,
        4 => registers.0,
        5 => registers.1,
        6 => registers.2,
        7 => 7,
        _ => panic!("Unknown operand {}", n)
    }
}

fn run_program(instructions: &Vec<i64>, registers: &mut (i64, i64, i64)) -> Vec<i64> {
    let mut ip = 0;
    let mut output: Vec<i64> = Vec::with_capacity(100);

    loop {
        let op = instructions.get(ip);
        let op = match op {
            Some(op) => *op,
            None => break
        };
        // println!("ip={}, op {},{} or {}, reg=({:o}, {:o}, {:o})", ip, op, instructions[ip+1], combo(instructions[ip+1], registers), registers.0, registers.1, registers.2);
        match op {
            0 => { // adv
                registers.0 = registers.0 / 2i64.pow(combo(instructions[ip+1], &registers) as u32);
            },
            1 => { // bxl
                registers.1 = registers.1 ^ instructions[ip+1];
            },
            2 => { // bst
                registers.1 = combo(instructions[ip+1], &registers) % 8;
            },
            3 => { // jnz
                if registers.0 != 0 {
                    ip = instructions[ip+1] as usize;
                    continue;
                }
            },
            4 => { // bxc
                registers.1 = registers.1 ^ registers.2;
            },
            5 => { // out
                output.push(combo(instructions[ip+1], &registers) %8);
                // println!("Output {}", output.last().unwrap());
            },
            6 => { // bdv
                registers.1 = registers.0 / 2i64.pow(combo(instructions[ip+1], &registers) as u32);
            },
            7 => { // cdv
                registers.2 = registers.0 / 2i64.pow(combo(instructions[ip+1], &registers) as u32);
            },
            _ => {
                panic!("Wrong instruction! ip={}, op={}", ip, op);
            },
        }
        ip += 2;
    }
    // println!("{:?} : {:?}", registers, instructions);
    output
}