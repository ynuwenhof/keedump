use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, env = "KEEDUMP_INPUT", value_name = "FILE")]
    input: PathBuf,
}

fn main() {
    let _cli = Cli::parse();
}
