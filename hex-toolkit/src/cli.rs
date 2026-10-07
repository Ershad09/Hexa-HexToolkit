use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "hexa",
    version,
    about = "Analyze and convert numbers between different representations"
)]
pub struct Cli {
    pub number: String,
}
