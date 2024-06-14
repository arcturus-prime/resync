use std::mem::size_of;

use iced_x86;

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

fn lift_flag_get(block: &mut Block, position: usize, value: bool) -> () {
    block.push_data(Data { boolean: value });
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
