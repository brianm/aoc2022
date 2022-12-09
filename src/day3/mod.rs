use std::collections::HashSet;
use std::fmt::Display;

use clap::Args;
use itertools::{Chunks, IntoChunks, Itertools};

const INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Item(char);

impl From<&str> for Item {
    fn from(s: &str) -> Item {
        Item(s.chars().next().unwrap())
    }
}

impl Item {
    fn priority(&self) -> u32 {
        match self.0 {
            'a'..='z' => (self.0 as u32) - 96,
            'A'..='Z' => (self.0 as u32) - 38,
            _ => panic!("Invalid Item"),
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    compartment1: Vec<Item>,
    compartment2: Vec<Item>,
}

impl Rucksack {
    fn create(items: Vec<Item>) -> Rucksack {
        let mut rucksack = Rucksack {
            compartment1: Vec::new(),
            compartment2: Vec::new(),
        };

        let mut mid = items.len() / 2;
        for item in items {
            if mid > 0 {
                mid = mid - 1;
                rucksack.compartment1.push(item);
            } else {
                rucksack.compartment2.push(item);
            }
        }
        rucksack
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Rucksack {
        Rucksack::create(s.chars().map(|c| Item(c)).collect())
    }
}

impl Display for Rucksack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c1 = self.compartment1.iter().map(|i| i.0).collect::<String>();
        let c2 = self.compartment2.iter().map(|i| i.0).collect::<String>();
        write!(f, "Rucksack {{ {} {} }}", c1, c2)
    }
}

#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub fn part1(&self) -> Box<dyn std::fmt::Display> {
        let mut value = 0u32;
        for line in INPUT.lines() {
            let r = Rucksack::from(line);
            let s1: HashSet<Item> = HashSet::from_iter(r.compartment1.iter().cloned());
            let s2: HashSet<Item> = HashSet::from_iter(r.compartment2.iter().cloned());

            let is = s1.intersection(&s2);
            for in_both in is {
                value += in_both.priority();
            }
        }
        return Box::from(value);
    }

    pub fn part2(&self) -> Box<dyn std::fmt::Display> {
        let mut value = 0u32;
        INPUT
            .lines()
            .map(|s| Rucksack::from(s))
            .tuples()
            .for_each(|(r1, r2, r3)| {
                let s1: HashSet<Item> = HashSet::from_iter(
                    r1.compartment1
                        .iter()
                        .cloned()
                        .chain(r1.compartment2.iter().cloned()),
                );

                let s2: HashSet<Item> = HashSet::from_iter(
                    r2.compartment1
                        .iter()
                        .cloned()
                        .chain(r2.compartment2.iter().cloned()),
                );

                let s3: HashSet<Item> = HashSet::from_iter(
                    r3.compartment1
                        .iter()
                        .cloned()
                        .chain(r3.compartment2.iter().cloned()),
                );

                let i1:HashSet<&Item> = HashSet::from_iter(s1.intersection(&s2).into_iter());
                let i2:HashSet<&Item> = HashSet::from_iter(s2.intersection(&s3).into_iter());
                
                let i3:HashSet<&&Item> = HashSet::from_iter(i1.intersection(&i2).into_iter());

                for in_all in i3 {
                    value += in_all.priority();
                }
            });

        return Box::from(value);
    }
}
