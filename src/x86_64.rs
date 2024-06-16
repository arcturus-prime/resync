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
        block.push(Code::Extract, Data::default());
    }
}

fn lift_register_set(block: &mut Block, register: iced_x86::Register) -> () {
    if register != register.full_register() {
        block.push(
            Code::Load,
            Data {
                uint: register.full_register() as usize,
            },
        );

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

        block.push(Code::Insert, Data::default());
    }

    block.push(
        Code::Save,
        Data {
            uint: register.full_register() as usize,
        },
    );
}

fn lift_flag_set(block: &mut Block, position: usize, value: bool) -> () {
    block.push(Code::Data, Data { uint: 1 });
    block.push(
        Code::Load,
        Data {
            uint: size_of::<iced_x86::Register>() * 2 ^ 8,
        },
    );
    block.push(Code::Data, Data { uint: position });
    block.push(Code::Insert, Data::default());

    block.push(
        Code::Save,
        Data {
            uint: size_of::<iced_x86::Register>() * 2 ^ 8,
        },
    );
}

fn lift_flag_get(block: &mut Block, position: usize) -> () {
    block.push(
        Code::Load,
        Data {
            uint: size_of::<iced_x86::Register>() * 2 ^ 8,
        },
    );
    block.push(Code::Data, Data { uint: position });
    block.push(Code::Data, Data { uint: 1 });

    block.push(Code::Extract, Data::default());
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
            block.push(
                Code::Data,
                Data {
                    uint: instruction.memory_index_scale() as usize,
                },
            );
            block.push(Code::Mul, Data::default());
        }

        num += 1;
    }

    if instruction.memory_displacement64() != 0 {
        block.push(
            Code::Data,
            Data {
                int: instruction.memory_displacement64() as isize,
            },
        );

        num += 1;
    }

    for _ in 1..num {
        block.push(Code::Add, Data::default());
    }
}

fn lift_memory_get(block: &mut Block, instruction: IcedInstruction) -> () {
    use iced_x86::MemorySize::*;

    block.push(Code::Load, Data { uint: 0 });
    lift_memory_expression(block, instruction);
    block.push(Code::Data, Data { uint: 8 });
    block.push(Code::Mul, Data::default());
    block.push(
        Code::Data,
        Data {
            uint: match instruction.memory_size() {
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
            },
        },
    );
    block.push(Code::Extract, Data::default());
}

fn lift_memory_set(block: &mut Block, instruction: IcedInstruction) -> () {
    block.push(Code::Load, Data { uint: 0 });
    lift_memory_expression(block, instruction);
    block.push(Code::Data, Data { uint: 8 });
    block.push(Code::Mul, Data::default());
    block.push(Code::Insert, Data::default());
    block.push(Code::Save, Data { uint: 0 });
}

fn lift_immediate_get(
    block: &mut Block,
    instruction: iced_x86::Instruction,
    operand: u32,
    size: usize,
) -> () {
    block.push(
        Code::Data,
        Data {
            uint: instruction.immediate(operand) as usize,
        },
    );
    if size != size_of::<usize>() * 8 {
        block.push(Code::Data, Data { uint: 0 });
        block.push(Code::Data, Data { uint: size });
        block.push(Code::Extract, Data::default());
    }
}

fn lift_operand_get(block: &mut Block, instruction: IcedInstruction, operand: u32) -> () {
    match instruction.op_kind(operand) {
        iced_x86::OpKind::NearBranch64
        | iced_x86::OpKind::NearBranch32
        | iced_x86::OpKind::NearBranch16 => block.push(
            Code::Data,
            Data {
                uint: instruction.near_branch_target() as usize,
            },
        ),
        iced_x86::OpKind::FarBranch16 => block.push(
            Code::Data,
            Data {
                uint: instruction.far_branch16() as usize,
            },
        ),
        iced_x86::OpKind::FarBranch32 => block.push(
            Code::Data,
            Data {
                uint: instruction.far_branch32() as usize,
            },
        ),
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
        | iced_x86::OpKind::Immediate64 => lift_immediate_get(block, instruction, operand, 64),
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

fn lift_operand_set(block: &mut Block, instruction: IcedInstruction, operand: u32) -> () {
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
            iced_x86::Mnemonic::Mov => lift_mov(block, instruction),
            iced_x86::Mnemonic::Push => lift_push(block, instruction),
            iced_x86::Mnemonic::Add => liftt_add(block, instruction),
            _ => {
                println!("Unimplemented instruction!");
            }
        }
    }
}

fn lift_mov(block: &mut Block, instruction: IcedInstruction) -> () {
    lift_operand_get(block, instruction, 1);
    lift_operand_set(block, instruction, 0);
}

fn lift_push(block: &mut Block, instruction: iced_x86::Instruction) -> () {
    lift_operand_get(block, instruction, 0);
    block.push(Code::Load, Data { uint: 0 });
    block.push(
        Code::Load,
        Data {
            uint: iced_x86::Register::RIP as usize,
        },
    );
    block.push(Code::Data, Data { uint: 8 });
    block.push(Code::Mul, Data::default());
    block.push(Code::Insert, Data::default());
    block.push(Code::Save, Data { uint: 0 });

    block.push(
        Code::Load,
        Data {
            uint: iced_x86::Register::RIP as usize,
        },
    );
    block.push(Code::Data, Data { uint: 8 });
    block.push(Code::Sub, Data::default());
    block.push(
        Code::Save,
        Data {
            uint: iced_x86::Register::RIP as usize,
        },
    )
}

fn liftt_add(block: &mut Block, instruction: IcedInstruction) -> () {
    lift_operand_get(block, instruction, 1);
    lift_operand_get(block, instruction, 0);
    block.push(Code::Add, Data::default());
    lift_operand_set(block, instruction, 0);
}
