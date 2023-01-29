use crate::{instruction::Instruction, register::Register};
use std::{
    io::{stdin, stdout, Read, Write},
    process::exit,
};

pub fn emulate(instructions: Vec<Instruction>) {
    let mut registers = [0u8; 7];
    let mut memory = [0u8; 256];
    let mut stack = [0u8; 256];

    loop {
        let instruction = &instructions[registers[Register::I.to_index()] as usize];
        registers[Register::I.to_index()] += 1;
        println!("{instruction}");

        match instruction {
            Instruction::IMM(register, value) => {
                registers[register.to_index()] = *value;
            }
            Instruction::ADD(register_a, register_b) => {
                registers[register_a.to_index()] =
                    registers[register_a.to_index()].wrapping_add(registers[register_b.to_index()]);
            }
            Instruction::SYS(syscall, arg) => match syscall {
                0x10 => {
                    // TODO: use registers to determine fd
                    let c = registers[Register::C.to_index()];
                    let mut buffer = vec![0u8; c as usize];
                    let bytes_read = stdin()
                        .read(&mut buffer)
                        .expect("failed to read from stdin");

                    let start = registers[Register::B.to_index()] as usize;
                    memory[start..start + bytes_read].copy_from_slice(&buffer[..bytes_read]);
                    registers[arg.to_index()] = bytes_read as u8;
                }
                0x1 => {
                    // TODO: use registers to determine fd
                    let b = registers[Register::B.to_index()];
                    let c = registers[Register::C.to_index()];

                    let bytes_written = stdout()
                        .write(&stack[b as usize - 1..b as usize + c as usize - 1])
                        .expect("failed to write to stdout");

                    registers[arg.to_index()] = bytes_written as u8;
                }
                0x8 => {
                    exit(*arg as i32);
                }
                _ => todo!("unimplemented syscall"),
            },
            Instruction::CMP(register_a, register_b) => {
                let a = registers[register_a.to_index()];
                let b = registers[register_b.to_index()];

                registers[Register::F.to_index()] = match a.cmp(&b) {
                    // TODO: verify that this is how the flags work
                    std::cmp::Ordering::Less => 0x11,
                    std::cmp::Ordering::Equal => 0x4,
                    std::cmp::Ordering::Greater => 0x18,
                };
            }
            Instruction::STM(register_a, register_b) => {
                memory[registers[register_a.to_index()] as usize] =
                    registers[register_b.to_index()];
            }
            Instruction::LDM(register_a, register_b) => {
                registers[register_a.to_index()] =
                    memory[registers[register_b.to_index()] as usize];
            }
            Instruction::JMP(condition, register) => {
                if registers[Register::F.to_index()] & condition != 0 {
                    registers[Register::I.to_index()] = registers[register.to_index()];
                }
            }
            Instruction::STK(push, pop) => {
                // TODO: handle stack {under,over}flow

                if *push != Register::None {
                    stack[registers[Register::S.to_index()] as usize] = registers[push.to_index()];
                    registers[Register::S.to_index()] += 1;
                }

                if *pop != Register::None {
                    registers[Register::S.to_index()] -= 1;
                    registers[pop.to_index()] = stack[registers[Register::S.to_index()] as usize];
                }
            }
        }
    }
}
