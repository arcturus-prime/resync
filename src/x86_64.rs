use crate::source::{Block, Node, NodeRef};
use iced_x86::{Decoder, Instruction};

pub struct Lifter<'a> {
    block: &'a mut Block,
    reg_state: [Option<u8>; 255],
    reg_order: [Option<u8>; 255],
}

impl<'a> Lifter<'a> {
    pub fn new(block: &'a mut Block) -> Self {
        Lifter { block, reg_state: [None; 255], reg_order: [None; 255] }
    }

    fn lift_operand(&mut self, inst: Instruction, operand: u32) -> NodeRef {
        match inst.op_kind(operand) {
            iced_x86::OpKind::Register => {
                let state = self.reg_state[inst.op_register(operand) as usize];
                let order = self.reg_order[inst.op_register(operand) as usize];

                if state.is_none() && order.is_some() {
                    return order.unwrap()
                }

                if state.is_some() {
                    return state.unwrap()
                }

                panic!("Register used in block without a previous load or calling order defined!");
            },
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

    fn lift_mov(&mut self, inst: Instruction) {
    }
}
