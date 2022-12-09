use clap::Args;
use std::collections::BinaryHeap;
use core::cmp::Reverse;

#[derive(Args)]
pub struct Command {
}

const INPUT: &str = include_str!("input");

fn top_sum(n: usize) -> u32 {
    let mut top = BinaryHeap::new();
    let mut current = 032;
    for line in INPUT.lines() {
        let num = line.parse::<u32>();

        match num {
            Ok(num) => {
                current += num;
            }
            Err(_) => {
                top.push(Reverse(current));
                if top.len() > n {
                    top.pop();
                }                    
                current = 0;
            }
        }
    }
    let mut sum = 0u32;
    for Reverse(val) in top {
        sum += val;
    }
    return sum;
}

impl Command {
    pub fn part1(&self) {
        println!("{}", top_sum(1));
    }

    pub fn part2(&self) {
        println!("{}", top_sum(3));
    }
}
