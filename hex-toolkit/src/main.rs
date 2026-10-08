mod cli;
mod core;
mod output;

use clap::Parser;

use crate::cli::Cli;
use crate::core::parser::parse_number;
use crate::output::print_number;

fn main() {
    let cli = Cli::parse();

    match parse_number(&cli.number) {
        Ok(number) => {
            print_number(&number);
        }

        Err(error) => {
            eprintln!("Error: {error}");
        }
    }
}
