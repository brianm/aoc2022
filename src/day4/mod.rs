use clap::Args;

const INPUT: &str = include_str!("input");

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl From<&str> for Range {
    fn from(s: &str) -> Range {
        let mut parts_first = s.split('-');
        let first_start = parts_first.next().unwrap().parse().unwrap();
        let first_end = parts_first.next().unwrap().parse().unwrap();
        return Range {
            start: first_start,
            end: first_end,
        };
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.start
            || other.start <= self.start && other.end >= self.start
    }
}

#[derive(Debug)]
struct Pair {
    first: Range,
    second: Range,
}

impl From<&str> for Pair {
    // 1-41,1-36
    fn from(s: &str) -> Pair {
        let mut parts = s.split(',');
        let first_raw = parts.next().unwrap();
        let second_raw = parts.next().unwrap();
        Pair {
            first: Range::from(first_raw),
            second: Range::from(second_raw),
        }
    }
}

#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub fn part1(&self) -> Box<dyn std::fmt::Display> {
        let mut value = 0u32;
        for line in INPUT.lines() {
            let p = Pair::from(line);
            if p.first.contains(&p.second) | p.second.contains(&p.first) {
                value += 1;
            }
        }
        return Box::from(value);
    }

    pub fn part2(&self) -> Box<dyn std::fmt::Display> {
        let mut value = 0u32;
        for line in INPUT.lines() {
            let p = Pair::from(line);
            if p.first.overlaps(&p.second) {
                value += 1;
            }
        }
        return Box::from(value);
    }
}
