use clap::Args;

const INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Move {
    fn from(s: &str) -> Move {
        match s {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,

            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,

            _ => panic!("Invalid move"),
        }
    }
}

impl Move {
    fn play_against(&self, other: &Move) -> bool {
        match self {
            Move::Rock => *other == Move::Scissors,
            Move::Paper => *other == Move::Rock,
            Move::Scissors => *other == Move::Paper,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Move::Rock => 1u32,
            Move::Paper => 2u32,
            Move::Scissors => 3u32,
        }
    }

    fn defeated_by(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn defeats(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn ties(&self) -> Move {
        match self {
            Move::Rock => Move::Rock,
            Move::Paper => Move::Paper,
            Move::Scissors => Move::Scissors,
        }
    }
}

fn score(other: Move, me: Move) -> u32 {
    me.value()
        + if other.play_against(&me) {
            0u32
        } else if me.play_against(&other) {
            6u32
        } else {
            3u32
        }
}

#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub fn part2(&self) -> Box<dyn std::fmt::Display> {
        let mut sum = 0u32;
        for line in INPUT.lines() {
            let mut parts = line.split_whitespace();
            let other = Move::from(parts.next().unwrap());
            let me = match parts.next() {
                Some("X") => other.defeats(),
                Some("Y") => other.ties(),
                Some("Z") => other.defeated_by(),
                _ => panic!("Invalid move"),
            };
            sum += score(other, me);
        }
        return Box::from(sum);
    }

    pub fn part1(&self) -> Box<dyn std::fmt::Display> {
        let mut sum = 0u32;
        for line in INPUT.lines() {
            let mut parts = line.split_whitespace();
            let other = Move::from(parts.next().unwrap());
            let me = Move::from(parts.next().unwrap());
            sum += score(other, me);
        }
        return Box::from(sum);
    }
}
