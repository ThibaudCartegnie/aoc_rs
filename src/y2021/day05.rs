use crate::common::Day;

pub struct Day05;

#[derive(Clone, Copy, Debug, Default)]
struct Point{
    x: i32,
    y: i32
}

#[derive(Clone, Copy, Debug, Default)]
struct Segment{
    p1: Point,
    p2: Point
}

#[allow(unused)]
impl Segment {
    fn from_str(line: &str) -> Segment {
        let points: Vec<&str> = line.split(" -> ").collect();
        let mut s = Segment::default();

        let mut p1 = points[0].split(',');
        s.p1.x = i32::from_str_radix(p1.next().unwrap(), 10).unwrap();
        s.p1.y = i32::from_str_radix(p1.next().unwrap(), 10).unwrap();

        let mut p2 = points[1].split(',');
        s.p2.x = i32::from_str_radix(p2.next().unwrap(), 10).unwrap();
        s.p2.y = i32::from_str_radix(p2.next().unwrap(), 10).unwrap();

        s
    }

    fn straight(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }

    fn intersect(&self, other: &Segment) -> i32 {
        0
    }
}

#[allow(unused)]
impl Day for Day05 {
    fn solve_part1(&self, input: &str) -> String {
        let segments: Vec<Segment> = input.lines().map(Segment::from_str).collect();

        format!("")
    }

    fn solve_part2(&self, input: &str) -> String {
        format!("")
    }
}
