use std::collections::HashMap;

use crate::{
    buffer::{add, and, collect_isize, collect_usize, mul, neg, not, or, shift, sub, xor},
    ir::Code,
};

pub struct State {
    pub stack: Vec<Vec<u8>>,
    pub files: HashMap<u8, Vec<u8>>,
}

impl State {
    pub fn new() -> State {
        State {
            stack: Vec::new(),
            files: HashMap::new(),
        }
    }
}

fn step(code: &mut &[u8], state: &mut State) {
    let op = code[0];

    match Code::try_from(op) {
        Ok(Code::Data) => {
            state.stack.push(code[2..2 + code[1] as usize].to_vec());
            *code = &code[2 + code[1] as usize..];
        }
        Ok(Code::Load) => {
            let a = collect_usize(&state.stack.pop().unwrap());
            let b = collect_usize(&state.stack.pop().unwrap());

            if let Some(file) = &state.files.get(&code[1]) {
                state.stack.push(file[a..b].to_vec());
            } else {
                state.stack.push(Vec::new());
            }
            *code = &code[2..];
        }
        Ok(Code::Save) => {
            let a = collect_usize(&state.stack.pop().unwrap());
            let b = state.stack.pop().unwrap();

            if !state.files.contains_key(&code[1]) {
                state.files.insert(code[1], Vec::new());
                state
                    .files
                    .get_mut(&code[1])
                    .unwrap()
                    .resize(a + b.len(), 0);
            }

            state.files.get_mut(&code[1]).unwrap()[a..a + b.len()].copy_from_slice(&b[..]);
            *code = &code[2..];
        }
        Ok(Code::Xor) => {
            let a = state.stack.pop().unwrap();

            xor(state.stack.last_mut().unwrap(), &a);
            *code = &code[1..];
        }
        Ok(Code::Or) => {
            let a = state.stack.pop().unwrap();

            or(state.stack.last_mut().unwrap(), &a);
            *code = &code[1..];
        }
        Ok(Code::And) => {
            let a = state.stack.pop().unwrap();

            and(state.stack.last_mut().unwrap(), &a);
            *code = &code[1..];
        }
        Ok(Code::Not) => {
            not(state.stack.last_mut().unwrap());
            *code = &code[1..];
        }
        Ok(Code::Shift) => {
            let a = collect_isize(&state.stack.pop().unwrap());

            shift(state.stack.last_mut().unwrap(), a);
            *code = &code[1..];
        }
        Ok(Code::Add) => {
            let a = state.stack.pop().unwrap();

            add(state.stack.last_mut().unwrap(), &a);
            *code = &code[1..];
        }
        Ok(Code::Sub) => {
            let a = state.stack.pop().unwrap();

            sub(state.stack.last_mut().unwrap(), &a);
            *code = &code[1..];
        }
        Ok(Code::Mul) => {
            let mut a = state.stack.pop().unwrap();
            let mut b = state.stack.pop().unwrap();

            state.stack.push(mul(&mut b, &mut a));
            *code = &code[1..];
        }
        Ok(Code::Div) => {
            let a = state.stack.pop().unwrap();
            *code = &code[1..];
        }
        Ok(Code::Mod) => {
            let a = state.stack.pop().unwrap();
            *code = &code[1..];
        }
        Ok(Code::Neg) => {
            neg(state.stack.last_mut().unwrap());
            *code = &code[1..];
        }
        Ok(Code::Lt) => todo!(),
        Ok(Code::Lte) => todo!(),
        Ok(Code::Gt) => todo!(),
        Ok(Code::Gte) => todo!(),
        Ok(Code::Eql) => todo!(),
        Ok(Code::Return) => todo!(),
        Ok(Code::VCall) => todo!(),
        Ok(Code::VJump) => todo!(),
        Ok(Code::Call) => todo!(),
        Ok(Code::Jump) => todo!(),
        Ok(Code::Nop) => {}
        Err(_) => panic!("Not an opcode!"),
    }
}

pub fn emulate(mut code: &[u8]) -> State {
    let mut state = State::new();

    while code.len() > 0 {
        step(&mut code, &mut state);
    }

    state
}
