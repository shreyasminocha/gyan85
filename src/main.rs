use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use yan85::{disasm::disassemble, emu::emulate};

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    #[clap(alias = "asm")]
    Assemble { path: PathBuf },

    #[clap(alias = "disasm")]
    Disassemble { path: PathBuf },

    #[clap(alias = "emu", alias = "run")]
    Emulate {
        path: PathBuf,
        #[clap(short = 'd', long)]
        show_disassembly: bool,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Assemble { .. } => todo!(),
        Command::Disassemble { path } => {
            let bytes = fs::read(path).expect("Unable to read file");
            let instructions = disassemble(bytes).expect("Failed to disassemble");

            for instruction in instructions {
                println!("{instruction}");
            }
        }
        Command::Emulate {
            path,
            show_disassembly,
        } => {
            let bytes = fs::read(path).expect("Unable to read file");
            emulate(
                disassemble(bytes).expect("Failed to disassemble"),
                show_disassembly,
            );
        }
    }
}
