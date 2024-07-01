use crate::ir::{Block, Node, NodeRef};
use crate::lifter::Lifter;

use iced_x86::{Decoder, Instruction};

pub struct X86_64Lifter {
    block: Block,
    reg_state: [Option<u8>; 255],
}

impl X86_64Lifter {
    fn lift_operand(&mut self, inst: Instruction, operand: u32) -> NodeRef {
        match inst.op_kind(operand) {
            iced_x86::OpKind::Register => self.reg_state[inst.op_register(operand) as usize]
                .expect("Register used with no previous load or calling order defined!"),
            iced_x86::OpKind::NearBranch16 => todo!(),
            iced_x86::OpKind::NearBranch32 => todo!(),
            iced_x86::OpKind::NearBranch64 => todo!(),
            iced_x86::OpKind::FarBranch16 => todo!(),
            iced_x86::OpKind::FarBranch32 => todo!(),
            iced_x86::OpKind::Immediate8 => todo!(),
            iced_x86::OpKind::Immediate8_2nd => todo!(),
            iced_x86::OpKind::Immediate16 => todo!(),
            iced_x86::OpKind::Immediate32 => todo!(),
            iced_x86::OpKind::Immediate64 => todo!(),
            iced_x86::OpKind::Immediate8to16 => todo!(),
            iced_x86::OpKind::Immediate8to32 => todo!(),
            iced_x86::OpKind::Immediate8to64 => todo!(),
            iced_x86::OpKind::Immediate32to64 => todo!(),
            iced_x86::OpKind::MemorySegSI => todo!(),
            iced_x86::OpKind::MemorySegESI => todo!(),
            iced_x86::OpKind::MemorySegRSI => todo!(),
            iced_x86::OpKind::MemorySegDI => todo!(),
            iced_x86::OpKind::MemorySegEDI => todo!(),
            iced_x86::OpKind::MemorySegRDI => todo!(),
            iced_x86::OpKind::MemoryESDI => todo!(),
            iced_x86::OpKind::MemoryESEDI => todo!(),
            iced_x86::OpKind::MemoryESRDI => todo!(),
            iced_x86::OpKind::Memory => todo!(),
        }
    }

    fn lift_mov(&mut self, inst: Instruction) {}
}


impl Lifter<&[u8]> for X86_64Lifter {
    fn lift(code: &[u8]) -> Block {
        todo!()
    }
}