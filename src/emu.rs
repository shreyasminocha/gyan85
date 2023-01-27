use crate::{instruction::Instruction, register::Register};

pub fn emulate(instructions: Vec<Instruction>) {
    let mut registers = [0u8; 7];
    let mut memory = [0u8; 256];
    let mut stack = [0u8; 256];

    loop {
        registers[Register::I.to_index()] += 1;
        let instruction = &instructions[registers[Register::I.to_index()] as usize];

        match instruction {
            Instruction::IMM(register, value) => {
                registers[register.to_index()] = *value;
            }
            Instruction::ADD(register_a, register_b) => {
                registers[register_a.to_index()] += registers[register_b.to_index()];
            }
            Instruction::SYS(syscall, register) => {
                todo!()
            }
            Instruction::CMP(register_a, register_b) => {
                todo!()
            }
            Instruction::STM(register_a, register_b) => {
                memory[registers[register_a.to_index()] as usize] =
                    registers[register_b.to_index()];
            }
            Instruction::LDM(register_a, register_b) => {
                memory[registers[register_a.to_index()] as usize] =
                    registers[register_b.to_index()];
            }
            Instruction::JMP(condition, register) => {
                todo!()
            }
            Instruction::STK(push, pop) => {
                // TODO: handle stack {under,over}flow

                if *push != Register::None {
                    stack[registers[Register::S.to_index()] as usize] = registers[push.to_index()];
                    registers[Register::S.to_index()] += 1;
                }

                if *pop != Register::None {
                    registers[Register::S.to_index()] -= 1;
                    stack[registers[pop.to_index()] as usize] =
                        stack[registers[Register::S.to_index()] as usize];
                }
            }
        }
    }
}
