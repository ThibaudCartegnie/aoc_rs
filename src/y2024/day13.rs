use crate::common::Day;

pub struct Day13;

#[derive(Debug, Clone, Default)]
struct Machine {
    but_a: (isize, isize),
    but_b: (isize, isize),
    prize: (isize, isize)
}

fn divide_only_if_integer(a: isize, b: isize) -> Option<isize> {
    match a%b {
        0 => Some(a/b),
        _ => None
    }
}

impl Machine {

    fn resolve(&self) -> Option<(isize, isize)> {
        // dbg!(&self);
        let (xa, ya) = self.but_a;
        let (xb, yb) = self.but_b;
        let (xp, yp) = self.prize;
        if xa == 0 || yb*xa-xb*ya == 0 {
            // println!("Incompatible: {} {} : m={:?}", xa, yb*xa-xb*ya, &self);
            return None;
        }

        if let Some(b) = divide_only_if_integer(yp*xa - xp*ya, yb*xa-xb*ya) {
            if let Some(a) = divide_only_if_integer(xp-b*xb, xa) {
                return Some((a, b));
            }
        }
        None
    }

    fn _resolve_schlag(&self) -> Option<(isize, isize)> {
        let (xa, ya) = self.but_a;
        let (xb, yb) = self.but_b;
        let (xp, yp) = self.prize;
        for a in 0..100 {
            for b in 0..100 {
                if xa*a +xb*b == xp && ya*a+yb*b == yp {
                    return Some((a,b))
                }
            }
        }
        None
    }
}

fn parse_line(line: &str, prize: bool, offset: isize) -> (isize, isize) {
    let patterns = if prize {
        ("X=", "Y=")
    } else {
        ("X+", "Y+")
    };
    let x_idx = line.find(patterns.0).unwrap();
    let x_idx_end = line[x_idx..].find(' ').unwrap();
    let x: isize = line[x_idx+2..x_idx+x_idx_end-1].parse().unwrap();

    let y_idx = line.find(patterns.1).unwrap();
    let y: isize = line[y_idx+2..].parse().unwrap();

    (x+offset, y+offset)
}

#[allow(unused)]
impl Day for Day13 {
    fn solve_part1(&self, input: &str) -> String {
        let mut machines = Vec::new();

        machines.push(Machine::default());
        for line in input.lines() {
            if line.len() == 0 {
                machines.push(Machine::default());
            } else if line.starts_with("Button A") {
                machines.last_mut().unwrap().but_a = parse_line(line, false, 0);
            } else if line.starts_with("Button B") {
                machines.last_mut().unwrap().but_b = parse_line(line, false, 0);
            } else if line.starts_with("Prize") {
                machines.last_mut().unwrap().prize = parse_line(line, true, 0);
            }
        }
        // if the input ends with an empty line, there will be an empty machine
        if machines.last().unwrap().but_a == (0,0) {
            machines.pop();
        }

        let res: isize = machines.iter().filter_map(|m| m.resolve()).map(|(a, b)| 3*a+b).sum();

        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut machines = Vec::new();

        machines.push(Machine::default());
        for line in input.lines() {
            if line.len() == 0 {
                machines.push(Machine::default());
            } else if line.starts_with("Button A") {
                machines.last_mut().unwrap().but_a = parse_line(line, false, 0);
            } else if line.starts_with("Button B") {
                machines.last_mut().unwrap().but_b = parse_line(line, false, 0);
            } else if line.starts_with("Prize") {
                machines.last_mut().unwrap().prize = parse_line(line, true, 10000000000000);
            }
        }
        // if the input ends with an empty line, there will be an empty machine
        if machines.last().unwrap().but_a == (0,0) {
            machines.pop();
        }

        let res: isize = machines.iter().filter_map(|m| m.resolve()).map(|(a, b)| 3*a+b).sum();

        format!("{}", res)
    }
}
