mod day1;
mod day2;
mod day3;
mod day4;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day1(day1::Command),
    Day2(day2::Command),
    Day3(day3::Command),
    Day4(day4::Command),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Day1(cmd) => {
            cmd.part1();
            cmd.part2();
        }
        Commands::Day2(cmd) => {
            println!("Part 1\t{}", cmd.part1());
            println!("Part 2\t{}", cmd.part2());
        }
        Commands::Day3(cmd) => {
            println!("Part 1\t{}", cmd.part1());
            println!("Part 2\t{}", cmd.part2());
        }
        Commands::Day4(cmd) => {
            println!("Part 1\t{}", cmd.part1());
            println!("Part 2\t{}", cmd.part2());
        }
    }
}
