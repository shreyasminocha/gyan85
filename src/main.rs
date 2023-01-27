use std::{env, fs, process::exit};

use yan85::disasm::disassemble;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <path>", args[0]);
        exit(1);
    }

    let path = &args[1];
    let bytes = fs::read(path).expect("Unable to read file");

    println!("{:?}", disassemble(bytes));
}
