use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "hexa",
    version,
    about = "Analyze and convert numbers between different representations"
)]
pub struct Cli {
    #[arg(allow_negative_numbers = true)]
    pub number: String,
}
