use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use yan85::{
    disasm::{disassemble, DisassembleError},
    emu::emulate,
};

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

fn main() -> Result<(), DisassembleError> {
    let args = Args::parse();

    match args.command {
        Command::Assemble { .. } => todo!(),
        Command::Disassemble { path } => {
            let bytes = fs::read(path).expect("Unable to open file");
            let instructions = disassemble(bytes)?;

            for instruction in instructions {
                println!("{instruction}");
            }

            Ok(())
        }
        Command::Emulate {
            path,
            show_disassembly,
        } => {
            let bytes = fs::read(path).expect("Unable to open file");
            let disassembly = disassemble(bytes)?;
            emulate(disassembly, show_disassembly);

            Ok(())
        }
    }
}
