/// Assembles instructions into machine code bytes.
mod assemble;
/// Parses strings of assembly instructions into our representations of those instructions.
mod parse;

pub use assemble::assemble;
pub use parse::parse_asm_file;
