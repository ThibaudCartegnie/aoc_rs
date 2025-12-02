use crate::common::Day;

pub struct Day01;

#[allow(unused)]
impl Day for Day01 {
    fn solve_part1(&self, input: &str) -> String {
        let mut counter = 50;
        let mut nb_zeros = 0;
        for line in input.lines() {
            let val = i32::from_str_radix(&line[1..], 10).unwrap();
            let dir = line.chars().nth(0).unwrap();
            if dir == 'L' {
                counter = (counter + 100 - val)%100;
            } else {
                counter = (counter + val)%100;
            }
            // println!("{}, c: {}", line, counter);
            if counter == 0 {
                nb_zeros += 1;
            }
        }

        format!("{}", nb_zeros)
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut counter = 50;
        let mut counter2 = 50;
        let mut nb_zeros = 0;
        let mut nb_zeros2 = 0;
        for line in input.lines() {
            let mut val = i32::from_str_radix(&line[1..], 10).unwrap();
            let dir = line.chars().nth(0).unwrap();

            if dir == 'L' {
                for i in 0..val {
                    counter2 -= 1;
                    if counter2 == 0 {
                        nb_zeros2 += 1;
                    } else if counter2 < 0 {
                        counter2 = 99;
                    } else if counter2 > 99 {
                        counter2 = 0;
                        nb_zeros2+=1;
                    }
                }
            } else {
                for i in 0..val {
                    counter2 += 1;
                    if counter2 == 0 {
                        nb_zeros2 += 1;
                    } else if counter2 < 0 {
                        counter2 = 99;
                    } else if counter2 > 99 {
                        counter2 = 0;
                        nb_zeros2+=1;
                    }
                }
            }

            if dir == 'L' {
                counter -= val;
            } else {
                counter += val;
            }
            println!("{}, nz {}", counter, nb_zeros);

            match counter {
                0 => {
                    nb_zeros += 1;
                },
                ..0 => {
                    let n = 1-counter/100;
                    println!("n {}", n);
                    nb_zeros += n;
                    counter += 100*n;
                    if counter == 100{
                        counter = 0;
                    }
                    if counter + val == 100 {
                        nb_zeros -= 1;
                    }
                },
                1.. => {
                    let n = counter/100;
                    nb_zeros += n;
                    counter -= 100*n;
                }
            }

            println!("{}, c {}, c2 {}, nz {}, nz2 {}", line, counter, counter2, nb_zeros, nb_zeros2);
            if counter != counter2 || nb_zeros != nb_zeros2 {
                return String::new();
            }
            // println!("{}, c: {}, nz: {}", line, counter, nb_zeros);

        }

        // 4787 too low
        // 6201 too high
        format!("{} == 5831", nb_zeros)
    }
}
