use crate::common::Day;

pub struct Day25;

#[derive(Debug, Clone, PartialEq, Default)]
struct KeyLock([u8; 5]);

impl KeyLock {
    fn remove_line(&mut self) {
        for n in self.0.iter_mut() {
            *n -= 1;
        }
    }
    fn fits(&self, other: &Self) -> bool {
        let r = self.0.iter().zip(other.0.iter()).all(|(a,b)| a + b <= 5);
        // println!("Key {:?}, Lock {:?}, fits: {}", self, other, r);
        r
    }
}

#[derive(Debug, PartialEq)]
enum ParserState {
    New,
    Key,
    Lock
}

fn parse_line(line: &str, keylock: &mut KeyLock) {
    assert!(line.len() == 5);
    for (i, c) in line.char_indices() {
        if c == '#' {
            keylock.0[i] += 1;
        }
    }
}

#[allow(unused)]
impl Day for Day25 {
    fn solve_part1(&self, input: &str) -> String {
        let mut keys: Vec<KeyLock> = Vec::with_capacity(256);
        let mut locks: Vec<KeyLock> = Vec::with_capacity(256);
        let mut idxs = (0, 0);
        let mut state = ParserState::New;
        for line in input.lines() {
            if line.len() == 0 {
                match state {
                    ParserState::Key => {
                        keys.get_mut(idxs.0).unwrap().remove_line();
                        idxs.0 += 1
                    },
                    ParserState::Lock => idxs.1 += 1,
                    ParserState::New => (),
                }
                state = ParserState::New;
                continue;
            }
            match state {
                ParserState::New => {
                    if ! line.contains('.') {
                        state = ParserState::Lock;
                        locks.push(KeyLock::default());
                    } else if ! line.contains('#') {
                        state = ParserState::Key;
                        keys.push(KeyLock::default());
                    }
                },
                ParserState::Key => {
                    parse_line(line, keys.get_mut(idxs.0).unwrap());
                },
                ParserState::Lock => {
                    parse_line(line, locks.get_mut(idxs.1).unwrap());
                },
            }
        }
        if state == ParserState::Key {
            keys.get_mut(idxs.0).unwrap().remove_line();
        }
        let mut res = 0;
        for lock in locks.iter() {
            for key in keys.iter() {
                if key.fits(lock) {
                    res += 1;
                }
            }
        }
        // println!("Locks {:?}", &locks);
        // println!("Keys {:?}", &keys);
        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        format!("")
    }
}
