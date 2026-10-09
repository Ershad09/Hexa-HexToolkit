use clap::Parser;

use hex_toolkit::cli::Cli;
use hex_toolkit::core::integer::{IntegerWidth, analyze_integer};
use hex_toolkit::core::parser::parse_number;
use hex_toolkit::output::{print_integer_analysis, print_number};

fn main() {
    let cli = Cli::parse();

    match parse_number(&cli.number) {
        Ok(number) => {
            print_number(&number);
            let widths = [
                IntegerWidth::Bits8,
                // IntegerWidth::Bits16,
                // IntegerWidth::Bits32,
                // IntegerWidth::Bits64,
                // IntegerWidth::Bits128,
            ];

            for width in widths {
                let analysis = analyze_integer(number.value, width);

                print_integer_analysis(&analysis);
            }
        }

        Err(error) => {
            eprintln!("Error: {error}");
        }
    }
}
