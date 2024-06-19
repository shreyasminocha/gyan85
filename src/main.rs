use std::{error::Error, fs, path::PathBuf};

use clap::{Parser, Subcommand};

use gyan85::{
    asm::{assemble, parse_asm_file},
    disasm::disassemble,
    emu::emulate,
    emulator::Emulator,
    yan85::memory::Memory,
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
    Assemble {
        input_path: PathBuf,
        output_path: PathBuf,
    },

    #[clap(alias = "disasm")]
    Disassemble { path: PathBuf },

    #[clap(alias = "emu", alias = "run")]
    Emulate {
        path: PathBuf,

        #[clap(short = 'd', long)]
        show_disassembly: bool,

        #[clap(short = 'm', long = "memory-image")]
        memory_image_path: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let constants_file = args.constants_file;
    let yaml = fs::read_to_string(constants_file).expect("Unable to open constants file");
    let consts = serde_yaml::from_str(&yaml).expect("Unable to parse constants file");

    match args.command {
        Command::Assemble {
            input_path,
            output_path,
        } => {
            let asm = fs::read_to_string(input_path).expect("Unable to open file");
            let instructions = parse_asm_file(asm).unwrap();

            let bytes = assemble(consts, &instructions);
            fs::write(output_path, bytes).expect("Unable to write file");

            Ok(())
        }
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
            memory_image_path,
        } => {
            let bytes = fs::read(path).expect("Unable to open file");
            let disassembly = disassemble(consts, bytes)?;

            let memory = match memory_image_path {
                Some(path) => {
                    let image: [u8; 256] = fs::read(path)
                        .expect("Unable to open file")
                        .try_into()
                        .expect("Memory image of wrong size");

                    Memory::from(image)
                }
                None => Memory::default(),
            };

            let mut emulator = Emulator::new(consts, disassembly, memory);
            emulate(&mut emulator, show_disassembly);

            Ok(())
        }
    }
}
