use iced_x86::{Decoder, DecoderOptions, Instruction as IcedInstruction};
use std::mem::size_of;

use crate::ir::{Block, Code, Data};

fn lift_register_get(block: &mut Block, register: iced_x86::Register) -> () {
    block.push(
        Code::Load,
        Data {
            uint: register.full_register() as usize,
        },
    );

    if register != register.full_register() {
        match register {
            iced_x86::Register::DH
            | iced_x86::Register::CH
            | iced_x86::Register::BH
            | iced_x86::Register::AH => {
                block.push(Code::Data, Data { uint: 8 });
            }
            _ => {
                block.push(Code::Data, Data { uint: 0 });
            }
        };
        block.push(
            Code::Data,
            Data {
                uint: register.size(),
            },
        );
        block.push_code(Code::Extract);
    }
}

fn lift_register_set(block: &mut Block, register: iced_x86::Register) -> () {
    block.push(
        Code::Load,
        Data {
            uint: register.full_register() as usize,
        },
    );
    if register != register.full_register() {
        match register {
            iced_x86::Register::DH
            | iced_x86::Register::CH
            | iced_x86::Register::BH
            | iced_x86::Register::AH => {
                block.push(Code::Data, Data { uint: 8 });
            }
            _ => {
                block.push(Code::Data, Data { uint: 0 });
            }
        };

        block.push_code(Code::Insert);
    }

    block.push(
        Code::Save,
        Data {
            uint: register.full_register() as usize,
        },
    );
}

fn lift_flag_set(block: &mut Block, position: usize, value: bool) -> () {
    block.push_data(Data::Uint(1 << position));
    block.push_data(Data::Uint(0));

    // Choose a slot beyond all the other registers because iced_x86 doesn't have a EFLAGS register enum
    block.push_data(Data::Uint(size_of::<iced_x86::Register>() * 2 ^ 8 * 64));

    block.push_code(Code::Store);
}

fn lift_flag_get(block: &mut Block, position: usize) -> () {
    block.push_data(Data::Uint(0));

    //See comment in lift_flag_set
    block.push_data(Data::Uint(size_of::<iced_x86::Register>() * 2 ^ 8 * 64));
    block.push_data(Data::Uint(1));

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
            block.push_data(Data::Uint(instruction.memory_index_scale() as usize));
            block.push(Code::Mul, Data { none: () });
        }

        num += 1;
    }

    if instruction.memory_displacement64() != 0 {
        block.push_data(Data::Int(instruction.memory_displacement64() as isize));

        num += 1;
    }

    for _ in 0..num {
        block.push_code(Code::Add);
    }
}

fn lift_memory_get(block: &mut Block, instruction: iced_x86::IcedInstruction) -> () {
    use iced_x86::MemorySize::*;

    block.push_data(Data::Uint(1));
    lift_memory_expression(block, instruction);
    block.push_data(Data::Uint(match instruction.memory_size() {
        UInt8 | Int8 => 1,
        UInt16 | Int16 | WordOffset => 2,
        UInt32 | Int32 | DwordOffset | SegPtr16 => 4,
        UInt52 => 7,
        UInt64 | Int64 | QwordOffset => 8,
        UInt128 | Int128 => 16,
        UInt256 | Int256 => 32,
        UInt512 | Int512 => 64,
        SegPtr32 => 6,
        SegPtr64 => 10,
        _ => todo!(),
    }));

    block.push_code(Code::Load);
}

fn lift_memory_set(block: &mut Block, instruction: iced_x86::IcedInstruction) -> () {
    block.push_data(Data::Uint(1));
    lift_memory_expression(block, instruction);

    block.push_code(Code::Store);
}

fn lift_immediate_get(
    block: &mut Block,
    instruction: iced_x86::Instruction,
    operand: u32,
    size: usize,
) -> () {
    block.push_data(Data::Uint(instruction.immediate(operand) as usize));
    block.push_data(Data::Uint(size));
    block.push_code(Code::Resize);
}

fn lift_operand_get(block: &mut Block, instruction: iced_x86::IcedInstruction, operand: u32) -> () {
    match instruction.op_kind(operand) {
        iced_x86::OpKind::NearBranch64
        | iced_x86::OpKind::NearBranch32
        | iced_x86::OpKind::NearBranch16 => {
            block.push_data(Data::Uint(instruction.near_branch_target() as usize))
        }
        iced_x86::OpKind::FarBranch16 => {
            block.push_data(Data::Uint(instruction.far_branch16() as usize))
        }
        iced_x86::OpKind::FarBranch32 => {
            block.push_data(Data::Uint(instruction.far_branch32() as usize))
        }
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
            block.push_data(Data::Uint(instruction.immediate(operand) as usize));
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

fn lift_operand_set(block: &mut Block, instruction: iced_x86::IcedInstruction, operand: u32) -> () {
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

pub fn lift(block: &mut Block, code: &[u8]) -> () {
    let mut decoder = Decoder::with_ip(64, code, 0, DecoderOptions::NONE);
    let mut instruction = IcedInstruction::default();

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);

        match instruction.mnemonic() {
            iced_x86::Mnemonic::Mov => {
                lift_operand_get(block, instruction, 1);
                lift_operand_set(block, instruction, 0);
            }
            iced_x86::Mnemonic::Add => {
                lift_operand_get(block, instruction, 1);
                lift_operand_get(block, instruction, 0);
                block.push_code(Code::Add);
                lift_operand_set(block, instruction, 0);
            }
            _ => {}
        }
    }
}
