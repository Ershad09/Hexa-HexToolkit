mod cli;
mod core;

use clap::Parser;

use crate::cli::Cli;
use crate::core::parser::parse_number;

fn main() {
    let cli = Cli::parse();

    match parse_number(&cli.number) {
        Ok(number) => {
            println!("Input: {}", number.original);
            println!("Base: {}", number.base);
            println!();

            println!("Decimal: {}", number.value);
            println!("Binary: {:b}", number.value);
            println!("Hexadecimal: {:X}", number.value);
        }

        Err(error) => {
            eprintln!("Error: {error}");
        }
    }
}
