use std::collections::HashMap;

use itertools::Itertools;

use crate::common::Day;

pub struct Day24;

#[derive(Debug, Clone, PartialEq, Eq)]
enum WireState {
    Z,
    Low,
    High,
}

impl WireState {
    fn or(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Z, _) => panic!("State should not be Z to be evaluated, be better"),
            (_, Self::Z) => panic!("State should not be Z to be evaluated, be better"),
            (Self::Low, Self::Low) => Self::Low,
            (_, _) => Self::High,
        }
    }

    fn and(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Z, _) => panic!("State should not be Z to be evaluated, be better"),
            (_, Self::Z) => panic!("State should not be Z to be evaluated, be better"),
            (Self::High, Self::High) => Self::High,
            (_, _) => Self::Low
        }
    }

    fn xor(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Z, _) => panic!("State should not be Z to be evaluated, be better"),
            (_, Self::Z) => panic!("State should not be Z to be evaluated, be better"),
            (Self::High, Self::Low) => Self::High,
            (Self::Low, Self::High) => Self::High,
            (_, _) => Self::Low
        }
    }
}

#[derive(Debug, Clone)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    in1: &'a str,
    in2: &'a str,
    out: &'a str,
    op: GateType
}

#[allow(unused)]
impl Day for Day24 {
    fn solve_part1(&self, input: &str) -> String {
        let mut wires = HashMap::with_capacity(256);
        let mut gates = Vec::with_capacity(256);
        for line in input.lines() {
            if line.contains(':') {
                let (wire_name, state) = line.split(": ").collect_tuple().unwrap();
                let state = match state {
                    "0" => WireState::Low,
                    "1" => WireState::High,
                    other @ _=> panic!("Unknown state {}", other)
                };
                wires.insert(wire_name, state);
            } else if line.len() != 0 {
                let (op, output_wire) = line.split(" -> ").collect_tuple().unwrap();
                let (in1, op, in2) = op.split(' ').collect_tuple().unwrap();
                let op = match op {
                    "AND" => GateType::And,
                    "OR" => GateType::Or,
                    "XOR" => GateType::Xor,
                    other @ _=> panic!("Unknown gate type {}", other)
                };
                gates.push(Gate {in1, in2, out: output_wire, op});
            }
        }

        loop {
            let mut change = false;
            for gate in gates.iter() {
                if wires.contains_key(gate.out) {
                    // gate already evaluated
                    continue;
                }
                if wires.contains_key(gate.in1) && wires.contains_key(gate.in2) {
                    match gate.op {
                        GateType::And => wires.insert(gate.out, wires[gate.in1].and(&wires[gate.in2])),
                        GateType::Or => wires.insert(gate.out, wires[gate.in1].or(&wires[gate.in2])),
                        GateType::Xor => wires.insert(gate.out, wires[gate.in1].xor(&wires[gate.in2])),
                    };
                    change = true;
                }
            }
            if change == false {
                break;
            }
        }

        let mut outputs= wires.iter().filter(|(k, _)| k.starts_with('z')).collect_vec();
        outputs.sort_by(|a, b| a.0.cmp(b.0));
        let res = outputs.iter().enumerate().fold(0, |acc, (i,(w, s))| acc + if **s == WireState::High { 2i64.pow(i as u32) } else { 0 });

        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut wires = HashMap::with_capacity(256);
        let mut gates = Vec::with_capacity(256);
        for line in input.lines() {
            if line.contains(':') {
                let (wire_name, state) = line.split(": ").collect_tuple().unwrap();
                let state = match state {
                    "0" => WireState::Low,
                    "1" => WireState::High,
                    other @ _=> panic!("Unknown state {}", other)
                };
                wires.insert(wire_name, state);
            } else if line.len() != 0 {
                let (op, output_wire) = line.split(" -> ").collect_tuple().unwrap();
                let (in1, op, in2) = op.split(' ').collect_tuple().unwrap();
                let op = match op {
                    "AND" => GateType::And,
                    "OR" => GateType::Or,
                    "XOR" => GateType::Xor,
                    other @ _=> panic!("Unknown gate type {}", other)
                };
                gates.push(Gate {in1, in2, out: output_wire, op});
            }
        }

        let x = parse_wires('x', &wires);
        let y = parse_wires('y', &wires);
        let obj = x+y;
        let mut n_loop = 0;
        loop {
            n_loop += 1;
            let mut change = false;
            for gate in gates.iter() {
                if wires.contains_key(gate.out) {
                    // gate already evaluated
                    continue;
                }
                if wires.contains_key(gate.in1) && wires.contains_key(gate.in2) {
                    match gate.op {
                        GateType::And => wires.insert(gate.out, wires[gate.in1].and(&wires[gate.in2])),
                        GateType::Or => wires.insert(gate.out, wires[gate.in1].or(&wires[gate.in2])),
                        GateType::Xor => wires.insert(gate.out, wires[gate.in1].xor(&wires[gate.in2])),
                    };
                    change = true;
                }
            }
            if change == false {
                break;
            }
        }
        println!(" x {:046b}\n y {:046b}\n z {:046b}\n o {:046b}", x, y, obj, parse_wires('z', &wires));
        
        format!("{} {} {}", parse_wires('z', &wires), n_loop, gates.len())
    }
}


fn parse_wires(prefix: char, wires: &HashMap<&str, WireState>) -> i64 {
    let mut outputs= wires.iter().filter(|(k, _)| k.starts_with(prefix)).collect_vec();
    outputs.sort_by(|a, b| a.0.cmp(b.0));
    outputs.iter().enumerate().fold(0, |acc, (i,(_, s))| acc + if **s == WireState::High { 2i64.pow(i as u32) } else { 0 })
}