use std::mem::size_of;

use iced_x86;

use crate::ir::{Block, Code, Data};

fn lift_register_offset(block: &mut Block, register: iced_x86::Register) -> () {
    let offset = match register {
        iced_x86::Register::AH => iced_x86::Register::RAX as usize * 512 + 8,
        iced_x86::Register::BH => iced_x86::Register::RBX as usize * 512 + 8,
        iced_x86::Register::CH => iced_x86::Register::RCX as usize * 512 + 8,
        iced_x86::Register::DH => iced_x86::Register::RDX as usize * 512 + 8,
        _ => register.full_register() as usize,
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

fn lift_memory_expression(block: &mut Block, operand: iced_x86::MemoryOperand) -> () {
    let mut num = 0;

    if operand.base != iced_x86::Register::None {
        lift_register_get(block, operand.base);
        num += 1;
    }

    if operand.index != iced_x86::Register::None {
        lift_register_get(block, operand.index);

        if operand.scale != 1 {
            block.push_data(Data {
                uint: operand.scale as usize,
            });
            block.push_code(Code::Mul);
        }

        num += 1;
    }

    if operand.displacement != 0 {
        block.push_data(Data {
            int: operand.displacement as isize,
        });

        num += 1;
    }

    for i in 0..num {
        block.push_code(Code::Add);
    }
}

fn lift_memory_get(block: &mut Block, operand: iced_x86::Operand) -> () {
    block.push_data(Data { uint: 1 });
    lift_memory_expression(block, operand);

    block.push_data(Data { uint:  });
}
