

pub trait Day {
    fn solve_part1(&self, input: &str) -> String;
    fn solve_part2(&self, input: &str) -> String;

    fn run(&self, input: &str) {
        println!("Part 1: {}", self.solve_part1(input));
        println!("Part 2: {}", self.solve_part2(input));
    }
}

pub trait Year {
    fn get_day(&self, day: usize) -> Option<impl Day>;
    fn days(&self) -> Vec<usize>;
}