use clap::Parser;
use std::collections::HashMap;
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
    let mut recovery_len = 0;
    let mut recovered = HashMap::new();

    loop {
        let len = file.read(&mut buf)?;
        if len == 0 {
            break;
        }

        let mut i = 0;
        while i < len - 1 {
            let first = buf[i];
            let second = buf[i + 1];

            if first == 0xCF && second == 0x25 {
                i += 1;
                recovery_len += 1;
            } else if recovery_len != 0 {
                recovery_len += 1;

                let num = u32::from_le_bytes([first, second, 0, 0]);
                match char::from_u32(num) {
                    None => i += 1,
                    Some(c) => {
                        if matches!(c, ' '..='~') {
                            recovered
                                .entry(recovery_len)
                                .and_modify(|v: &mut Vec<char>| v.push(c))
                                .or_insert(vec![c]);
                        }

                        recovery_len = 0;
                    }
                }
            }

            i += 1;
        }
    }

    for val in recovered.values_mut() {
        val.sort();
        val.dedup();
    }

    Ok(())
}
