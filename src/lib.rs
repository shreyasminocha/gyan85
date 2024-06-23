#![warn(missing_docs)]

//! Yan85 assembler, disassembler, and emulator.

/// Yan85 assembler. Converts Yan85 assembly to machine code.
pub mod asm;
/// Yan85 disassembler. Converts Yan85 machine code to assembly.
pub mod disasm;
/// Yan85 emulator wrapper that steps through instructions indefinitely.
pub mod emu;
/// Yan85 emulator.
pub mod emulator;
/// Yan85 architecture representation structures.
pub mod yan85;
