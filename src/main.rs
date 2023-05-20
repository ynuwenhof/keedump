use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, env = "KEEDUMP_INPUT", value_name = "FILE")]
    input: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;

    let mut file = File::open(cli.input)?;
    let mut buf = [0; 1024];

    loop {
        let len = file.read(&mut buf)?;
        if len == 0 {
            break;
        }

        // TODO: Handle chunks
    }

    Ok(())
}
