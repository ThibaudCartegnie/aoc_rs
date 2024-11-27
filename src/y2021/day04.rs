use crate::common::Day;

pub struct Day04;

#[derive(Clone, Debug, Default)]
struct Cell {
    number: i32,
    marked: bool
}

#[derive(Clone, Debug, Default)]
struct Bingo {
    rows: [[Cell; 5]; 5],
    bingo: bool
}

impl Bingo {
    fn mark(&mut self, number: i32) {
        for row in & mut self.rows {
            for col in row {
                if col.number == number {
                    col.marked = true;
                }
            }
        }
    }

    fn bingo(&mut self) -> bool {
        let rows =  self.rows.iter().any(|r| r.iter().all(|c|c.marked));
        let mut cols = false;
        for i in 0..5 {
            let mut col = true;
            for r in &self.rows {
                col &= r[i].marked;
            }
            cols |= col;
        }
        self.bingo = rows || cols;
        self.bingo
    }

    fn get_score(&self) -> i32 {
        self.rows.iter().fold(0, |acc, row|row.iter().fold(acc, |s, c| s + if c.marked { 0 } else { c.number }))
    }
}

#[allow(unused)]
impl Day for Day04 {
    fn solve_part1(&self, input: &str) -> String {
        let mut lines = input.lines().filter(|l| l.len() != 0);
        let first_line = lines.next().unwrap();
        let numbers: Vec<i32> = first_line.split(",").map(|n| i32::from_str_radix(n, 10).unwrap()).collect();
        let mut cards: Vec<Bingo> = Vec::new();
        let mut card_line = 0;
        for (i, line) in lines.enumerate() {
            if i%5 == 0 {
                cards.push(Bingo::default());
            }
            let numbers: Vec<i32> = line.split(" ").filter(|n| n.len() != 0).map(|n| i32::from_str_radix(n, 10).unwrap()).collect();
            let last = cards.len()-1;
            let card = cards.get_mut(last).unwrap();
            for (idx, nb) in numbers.iter().enumerate() {
                card.rows[i%5][idx].number = *nb;
            }
        }

        for nb in numbers {
            for card in & mut cards {
                card.mark(nb);
                if card.bingo() {
                    return format!("BINGO! {}", card.get_score()*nb)
                }
            }
        }
        format!("No bingo :'(")
    }

    fn solve_part2(&self, input: &str) -> String {
        let mut lines = input.lines().filter(|l| l.len() != 0);
        let first_line = lines.next().unwrap();
        let numbers: Vec<i32> = first_line.split(",").map(|n| i32::from_str_radix(n, 10).unwrap()).collect();
        let mut cards: Vec<Bingo> = Vec::new();
        let mut card_line = 0;
        for (i, line) in lines.enumerate() {
            if i%5 == 0 {
                cards.push(Bingo::default());
            }
            let numbers: Vec<i32> = line.split(" ").filter(|n| n.len() != 0).map(|n| i32::from_str_radix(n, 10).unwrap()).collect();
            let last = cards.len()-1;
            let card = cards.get_mut(last).unwrap();
            for (idx, nb) in numbers.iter().enumerate() {
                card.rows[i%5][idx].number = *nb;
            }
        }

        let mut n_bingo = 0;
        let n_cards = cards.len();
        for nb in numbers {
            for (i, card) in cards.iter_mut().enumerate() {
                card.mark(nb);
                if ! card.bingo && card.bingo() {
                    n_bingo += 1;
                    // println!("Bingo! Card n°{}, {}/{}",i, n_bingo, n_cards);
                    if n_bingo == n_cards {
                        return format!("Last bingo :( : {}", card.get_score()*nb);
                    }
                }
            }
        }
        format!("No bingo :'(")
    }
}
