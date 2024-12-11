use crate::common::Day;

pub struct Day09;

#[derive(Debug, Clone)]
struct File {
    id: u32,
    size: i8,
    place: usize
}

impl File {
    fn new(id: u32, size: i8, place: usize) -> File {
        File { id, size, place }
    }
}

#[allow(unused)]
impl Day for Day09 {
    fn solve_part1(&self, input: &str) -> String {
        let nbs: Vec<i8> = input.split("").filter(|c| *c != "\n" && c.len() != 0).map(|c| c.parse().unwrap()).collect();
        let mut expanded = Vec::with_capacity(nbs.len()*5);
        let mut counter: u32 = 0;
        let mut is_file = true;
        for nb in nbs {
            if is_file {
                // file
                for _ in 0..nb {
                    expanded.push(Some(counter));
                }
                counter += 1;
            } else {
                // empty space
                for _ in 0..nb {
                    expanded.push(None);
                }
            }
            is_file = !is_file;
        }
        // print(&expanded);
        let mut reduced =  expanded.clone();
        let mut counter = 0;
        for (i, nb) in expanded.iter().enumerate().rev() {
            if counter >= i {
                break;
            }
            if let Some(nb) = nb {
                while reduced[counter].is_some() {
                    counter += 1;
                }
                reduced[counter] = Some(*nb);
                reduced[i] = None;
                counter += 1;
            }
        }
        // print(&reduced);
        let res = reduced.iter().filter(|n| n.is_some()).enumerate().fold(0, |acc, (i, n)| acc + i * n.unwrap() as usize);
        // 1 960 677 604 too low (overflow error :'()
        format!("{}", res)
    }

    fn solve_part2(&self, input: &str) -> String {
        let nbs: Vec<i8> = input.split("").filter(|c| *c != "\n" && c.len() != 0).map(|c| c.parse().unwrap()).collect();
        let mut expanded = Vec::with_capacity(nbs.len()*5);
        let mut counter: u32 = 0;
        let mut is_file = true;
        let mut files = Vec::with_capacity(nbs.len()/2 + 1);
        let mut place = 0;
        for nb in nbs {
            if is_file {
                // file
                for _ in 0..nb {
                    expanded.push(Some(counter));
                }
                files.push(File::new(counter, nb, place));
                counter += 1;
            } else {
                // empty space
                for _ in 0..nb {
                    expanded.push(None);
                }
            }
            place += nb as usize;
            is_file = !is_file;
        }

        files.sort_by(|a, b| a.id.cmp(&b.id).reverse());

        let mut reduced = expanded.clone();
        // print(&expanded);
        let max_len = reduced.len();
        'outer : for f in files.iter() {
            // dbg!(&f);
            
            let mut counter = 0;
            while counter < f.place {
                while reduced[counter].is_some() {
                    counter += 1;
                }

                if counter > f.place {
                    continue;
                }
                
                let place_found = counter;
                let mut size_found: usize = 0;
                // dbg!(&place_found);
                while size_found <= f.size as usize && counter < max_len && reduced[counter].is_none() {
                    // println!("wat");
                    size_found += 1;
                    counter += 1;
                }

                // println!("{:?} {:?}", size_found, size_found <= f.size as usize);
                
                if size_found >= f.size as usize {
                    for i in 0..f.size as usize {
                        reduced[place_found + i] = Some(f.id);
                        reduced[f.place + i] = None;
                    }
                    // print(&reduced);
                    continue 'outer;
                }
            }
        }
        print_t(&expanded[0..100.min(expanded.len())]);
        print_t(&reduced[0..100.min(reduced.len())]);
        let res = reduced.iter().enumerate().fold(0, |acc, (i, n)| acc + i * if let Some(nb) = n { *nb as usize} else { 0 });
        println!("Part 2: 6409057127686 too high :'(");
        format!("{}", res)
    }
}

fn _print(v: &Vec<Option<u32>>) {
    for n in v {
        match n {
            None => print!("."),
            Some(n) => print!("{}", n)
        };
    }
    println!("")
}

fn print_t(v: &[Option<u32>]) {
    for n in v {
        match n {
            None => print!("'.'"),
            Some(n) => print!("'{}'", n)
        };
    }
    println!("")
}