use crate::yan85::{
    constants::Constants, instruction::Instruction, memory::Memory, register::Register,
    registers::Registers, stack::Stack,
};
use std::{
    cmp,
    io::{stdin, stdout, Read, Write},
    process::exit,
};

pub fn emulate(
    constants: Constants,
    instructions: &[Instruction],
    show_disassembly: bool,
    memory: &mut Memory,
) {
    let Constants {
        flag: f,
        syscall: s,
        ..
    } = constants;

    let mut registers = Registers::default();
    let mut stack = Stack::default();

    loop {
        let instruction = &instructions[registers[Register::I] as usize];
        registers[Register::I] += 1;

        if show_disassembly {
            println!("{instruction}");
        }

        match *instruction {
            Instruction::IMM(register, value) => {
                registers[register] = value;
            }
            Instruction::ADD(register_a, register_b) => {
                registers[register_a] = registers[register_a].wrapping_add(registers[register_b]);
            }
            Instruction::STK(push, pop) => {
                // TODO: handle stack {under,over}flow

                if push != Register::None {
                    stack[registers[Register::S]] = registers[push];
                    registers[Register::S] += 1;
                }

                if pop != Register::None {
                    registers[Register::S] -= 1;
                    registers[pop] = stack[registers[Register::S]];
                }
            }
            Instruction::STM(register_a, register_b) => {
                memory[registers[register_a]] = registers[register_b];
            }
            Instruction::LDM(register_a, register_b) => {
                registers[register_a] = memory[registers[register_b]];
            }
            Instruction::CMP(register_a, register_b) => {
                let a = registers[register_a];
                let b = registers[register_b];

                let mut flags = 0;

                match a.cmp(&b) {
                    cmp::Ordering::Less => flags |= f.L | f.N,
                    cmp::Ordering::Greater => flags |= f.G | f.N,
                    cmp::Ordering::Equal => flags |= f.E,
                }

                if (a == 0) && (b == 0) {
                    flags |= f.Z;
                }

                registers[Register::F] = flags;
            }
            Instruction::JMP(condition, register) => {
                if registers[Register::F] & condition != 0 {
                    registers[Register::I] = registers[register];
                }
            }
            Instruction::SYS(syscall, arg) => match syscall {
                _ if syscall == s.READ_MEMORY => {
                    // TODO: use registers to determine fd
                    let c = registers[Register::C];
                    let mut buffer = vec![0u8; c as usize];
                    let bytes_read = stdin()
                        .read(&mut buffer)
                        .expect("failed to read from stdin");
                    let bytes_read = u8::try_from(bytes_read).expect("the buffer size is a u8");

                    let start = registers[Register::B];
                    memory[start..start + bytes_read]
                        .copy_from_slice(&buffer[..bytes_read as usize]);
                    registers[arg] = bytes_read;
                }
                _ if syscall == s.WRITE => {
                    // TODO: use registers to determine fd
                    let b = registers[Register::B];
                    let c = registers[Register::C];

                    let bytes_written = stdout()
                        .write(&memory[b..b + c])
                        .expect("failed to write to stdout");
                    let bytes_written =
                        u8::try_from(bytes_written).expect("the range size is at most 255");

                    registers[arg] = bytes_written;
                }
                _ if syscall == s.EXIT => {
                    exit(registers[Register::A] as i32);
                }
                _ => todo!("unimplemented syscall {syscall:#02x}"),
            },
        }
    }
}
