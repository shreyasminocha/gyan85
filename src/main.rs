use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use yan85::{
    disasm::{disassemble, DisassembleError},
    emu::emulate,
};

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "constants.yml")]
    constants_file: PathBuf,

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

    let constants_file = args.constants_file;
    let yaml = fs::read_to_string(constants_file).expect("Unable to open constants file");
    let consts = serde_yaml::from_str(&yaml).expect("Unable to parse constants file");

    match args.command {
        Command::Assemble { .. } => todo!(),
        Command::Disassemble { path } => {
            let bytes = fs::read(path).expect("Unable to open file");
            let instructions = disassemble(consts, bytes)?;

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
            let disassembly = disassemble(consts, bytes)?;
            emulate(consts, disassembly, show_disassembly);

            Ok(())
        }
    }
}
