mod day1;
mod day2;

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
    /// Adds files to myapp
    Day1(day1::Command),
    Day2(day2::Command),
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
    }
}
