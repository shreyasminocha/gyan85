//! Command-line interface to the assembler, disassembler, and emulator.

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};

use gyan85::{
    asm::{assemble, parse_asm_file},
    disasm::disassemble,
    emu::emulate,
    emulator::Emulator,
    yan85::memory::Memory,
};

/// Supported arguments.
#[derive(Parser, Debug)]
struct Args {
    /// YAML file specifying level-specific encoding constants.
    #[clap(short, long, default_value = "constants.yml")]
    constants_file: PathBuf,

    /// Subcommand.
    #[clap(subcommand)]
    command: Command,
}

/// Supported subcommands.
#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// Assemble Yan85 assembly to machine code.
    #[clap(alias = "asm")]
    Assemble {
        /// Path of the assembly file to convert.
        input_path: PathBuf,
        /// Path to output file.
        output_path: PathBuf,
    },

    /// Disassemble Yan85 machine code to assembly.
    #[clap(alias = "disasm")]
    Disassemble {
        /// Path of the machine code file to convert.
        path: PathBuf,
    },

    /// Emulate the supplied Yan85 machine code.
    #[clap(alias = "emu", alias = "run")]
    Emulate {
        /// Path of the machine code file to emulate.
        path: PathBuf,

        /// Whether to output the disassembly of each instruction emulated.
        #[clap(short = 'd', long)]
        show_disassembly: bool,

        /// Path to an initial Yan85 memory image.
        #[clap(short = 'm', long = "memory-image")]
        memory_image_path: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    let constants_file = args.constants_file;
    let yaml = fs::read_to_string(constants_file)?;
    let consts = serde_yaml::from_str(&yaml).context("Unable to parse constants file")?;

    match args.command {
        Command::Assemble {
            input_path,
            output_path,
        } => {
            let asm = fs::read_to_string(input_path)?;
            let instructions = parse_asm_file(asm)?;

            let bytes = assemble(&instructions, consts);
            fs::write(output_path, bytes)?;

            Ok(())
        }
        Command::Disassemble { path } => {
            let bytes = fs::read(path)?;
            let instructions = disassemble(bytes, consts)?;

            for instruction in instructions {
                println!("{instruction}");
            }

            Ok(())
        }
        Command::Emulate {
            path,
            show_disassembly,
            memory_image_path,
        } => {
            let bytes = fs::read(path)?;
            let disassembly = disassemble(bytes, consts)?;

            let memory = match memory_image_path {
                Some(path) => {
                    let image: [u8; 256] = fs::read(path)?
                        .try_into()
                        .map_err(|_| anyhow!("Memory image of wrong size"))?;

                    Memory::from(image)
                }
                None => Memory::default(),
            };

            let mut emulator = Emulator::new(consts, disassembly, memory);
            emulate(&mut emulator, show_disassembly)?;

            Ok(())
        }
    }
}
