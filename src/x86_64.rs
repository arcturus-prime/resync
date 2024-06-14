use iced_x86;
use std::mem::size_of;

use crate::ir::{Block, Code, Data};

fn lift_register_offset(block: &mut Block, register: iced_x86::Register) -> () {
    let offset = match register {
        iced_x86::Register::AH => iced_x86::Register::RAX as usize * 512 + 8,
        iced_x86::Register::BH => iced_x86::Register::RBX as usize * 512 + 8,
        iced_x86::Register::CH => iced_x86::Register::RCX as usize * 512 + 8,
        iced_x86::Register::DH => iced_x86::Register::RDX as usize * 512 + 8,
        _ => register.full_register() as usize * 512,
    };

    block.push_data(Data { uint: offset });
}

fn lift_register_get(block: &mut Block, register: iced_x86::Register) -> () {
    block.push_data(Data { uint: 0 });
    lift_register_offset(block, register);
    block.push_data(Data {
        uint: register.size(),
    });
    block.push_code(Code::Load);
}

fn lift_register_set(block: &mut Block, register: iced_x86::Register) -> () {
    block.push_data(Data { uint: 0 });
    lift_register_offset(block, register);
    block.push_code(Code::Store);
}

fn lift_flag_set(block: &mut Block, position: usize, value: bool) -> () {
    block.push_data(Data { boolean: value });
    block.push_data(Data { uint: 0 });
    block.push_data(Data {
        uint: size_of::<iced_x86::Register>() * 2 ^ 8 * 512 + position,
    });

    block.push_code(Code::Store);
}

fn lift_flag_get(block: &mut Block, position: usize) -> () {
    block.push_data(Data { uint: 0 });
    block.push_data(Data {
        uint: size_of::<iced_x86::Register>() * 2 ^ 8 * 512 + position,
    });
    block.push_data(Data { uint: 1 });

    block.push_code(Code::Load);
}

fn lift_memory_expression(block: &mut Block, instruction: iced_x86::Instruction) -> () {
    let mut num = 0;

    if instruction.memory_base() != iced_x86::Register::None {
        lift_register_get(block, instruction.memory_base());
        num += 1;
    }

    if instruction.memory_index() != iced_x86::Register::None {
        lift_register_get(block, instruction.memory_index());

        if instruction.memory_index_scale() != 1 {
            block.push_data(Data {
                uint: instruction.memory_index_scale() as usize,
            });
            block.push_code(Code::Mul);
        }

        num += 1;
    }

    if instruction.memory_displacement64() != 0 {
        block.push_data(Data {
            int: instruction.memory_displacement64() as isize,
        });

        num += 1;
    }

    for _ in 0..num {
        block.push_code(Code::Add);
    }
}

fn lift_memory_get(block: &mut Block, instruction: iced_x86::Instruction) -> () {
    block.push_data(Data { uint: 1 });
    lift_memory_expression(block, instruction);

    block.push_data(Data {
        uint: instruction.memory_displ_size() as usize * 8,
    });
    block.push_code(Code::Load);
}

fn lift_memory_set(block: &mut Block, instruction: iced_x86::Instruction) -> () {
    block.push_data(Data { uint: 1 });
    lift_memory_expression(block, instruction);

    block.push_code(Code::Store);
}

fn lift_immediate_get(
    block: &mut Block,
    instruction: iced_x86::Instruction,
    operand: u32,
    size: usize,
) -> () {
    block.push_data(Data {
        uint: instruction.immediate(operand) as usize,
    });
    block.push_data(Data { uint: size });
    block.push_code(Code::Resize);
}

fn lift_operand_get(block: &mut Block, instruction: iced_x86::Instruction, operand: u32) -> () {
    match instruction.op_kind(operand) {
        iced_x86::OpKind::NearBranch64
        | iced_x86::OpKind::NearBranch32
        | iced_x86::OpKind::NearBranch16 => block.push_data(Data {
            uint: instruction.near_branch_target() as usize,
        }),
        iced_x86::OpKind::FarBranch16 => block.push_data(Data {
            uint: instruction.far_branch16() as usize,
        }),
        iced_x86::OpKind::FarBranch32 => block.push_data(Data {
            uint: instruction.far_branch32() as usize,
        }),
        iced_x86::OpKind::Immediate8_2nd | iced_x86::OpKind::Immediate8 => {
            lift_immediate_get(block, instruction, operand, 8)
        }
        iced_x86::OpKind::Immediate8to16 | iced_x86::OpKind::Immediate16 => {
            lift_immediate_get(block, instruction, operand, 16)
        }
        iced_x86::OpKind::Immediate8to32 | iced_x86::OpKind::Immediate32 => {
            lift_immediate_get(block, instruction, operand, 32)
        }
        iced_x86::OpKind::Immediate8to64
        | iced_x86::OpKind::Immediate32to64
        | iced_x86::OpKind::Immediate64 => {
            block.push_data(Data {
                uint: instruction.immediate(operand) as usize,
            });
        }
        iced_x86::OpKind::Register => lift_register_get(block, instruction.op_register(operand)),
        iced_x86::OpKind::MemorySegSI => todo!(),
        iced_x86::OpKind::MemorySegESI => todo!(),
        iced_x86::OpKind::MemorySegRSI => todo!(),
        iced_x86::OpKind::MemorySegDI => todo!(),
        iced_x86::OpKind::MemorySegEDI => todo!(),
        iced_x86::OpKind::MemorySegRDI => todo!(),
        iced_x86::OpKind::MemoryESDI => todo!(),
        iced_x86::OpKind::MemoryESEDI => todo!(),
        iced_x86::OpKind::MemoryESRDI => todo!(),
        iced_x86::OpKind::Memory => lift_memory_get(block, instruction),
    }
}

fn lift_operand_set(block: &mut Block, instruction: iced_x86::Instruction, operand: u32) -> () {
    match instruction.op_kind(operand) {
        iced_x86::OpKind::NearBranch16
        | iced_x86::OpKind::NearBranch32
        | iced_x86::OpKind::NearBranch64
        | iced_x86::OpKind::FarBranch16
        | iced_x86::OpKind::FarBranch32
        | iced_x86::OpKind::Immediate8
        | iced_x86::OpKind::Immediate8_2nd
        | iced_x86::OpKind::Immediate32
        | iced_x86::OpKind::Immediate64
        | iced_x86::OpKind::Immediate8to16
        | iced_x86::OpKind::Immediate8to32
        | iced_x86::OpKind::Immediate8to64
        | iced_x86::OpKind::Immediate32to64
        | iced_x86::OpKind::Immediate16 => panic!("Cannot call lift_operand_set with a rvalue!"),
        iced_x86::OpKind::Register => lift_register_set(block, instruction.op_register(operand)),
        iced_x86::OpKind::MemorySegSI => todo!(),
        iced_x86::OpKind::MemorySegESI => todo!(),
        iced_x86::OpKind::MemorySegRSI => todo!(),
        iced_x86::OpKind::MemorySegDI => todo!(),
        iced_x86::OpKind::MemorySegEDI => todo!(),
        iced_x86::OpKind::MemorySegRDI => todo!(),
        iced_x86::OpKind::MemoryESDI => todo!(),
        iced_x86::OpKind::MemoryESEDI => todo!(),
        iced_x86::OpKind::MemoryESRDI => todo!(),
        iced_x86::OpKind::Memory => lift_memory_set(block, instruction),
    }
}

fn lift(block: &mut Block, code: &[u8]) -> () {}
