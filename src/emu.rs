use anyhow::Result;

use crate::emulator::Emulator;

/// Emulates the Yan85 program loaded in `emulator`.
pub fn emulate(emulator: &mut Emulator, show_disassembly: bool) -> Result<()> {
    loop {
        let instruction = emulator.step()?;

        if show_disassembly {
            println!("{instruction}");
        }
    }
}
