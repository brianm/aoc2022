use clap::Args;
use std::collections::BinaryHeap;
use core::cmp::Reverse;

#[derive(Args)]
pub struct Command {
    name: Option<String>,
}

impl Command {
    pub fn part2(&self) {
        let input = include_str!("input");
        let mut top = BinaryHeap::new();
        let mut current = 0;
        for line in input.lines() {
            let num = line.parse::<i32>();

            match num {
                Ok(num) => {
                    current += num;
                }
                Err(_) => {
                    top.push(Reverse(current));
                    if top.len() > 3 {
                        top.pop();
                    }                    
                    current = 0;
                }
            }
        }
        let mut sum = 0;
        for Reverse(val) in top {
            sum += val;
        }
        println!("{}", sum);
    }

    pub fn part1(&self) {
        let input = include_str!("input");
        let mut max = -1;
        let mut current = 0;
        for line in input.lines() {
            let num = line.parse::<i32>();

            match num {
                Ok(num) => {
                    current += num;
                }
                Err(_) => {
                    if current > max {
                        max = current;
                    }
                    current = 0;
                }
            }
        }
        println!("{}", max);
    }
}
