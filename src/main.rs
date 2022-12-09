pub mod day1;

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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Day1(cmd) => {
            cmd.part1();
            cmd.part2();
        }
    }
}
