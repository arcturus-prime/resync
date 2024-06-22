use crate::ir::Code;

pub struct State {
    pub stack: Vec<u128>,
    pub memory: Vec<u8>,
}

impl State {
    pub fn new() -> State {
        State {
            stack: Vec::new(),
            memory: Vec::new(),
        }
    }
}

fn step(code: &mut &[u8], state: &mut State) {
    let op = code[0];

    match Code::try_from(op) {
        Ok(Code::Nop) => {}
        Ok(Code::Data) => {
            let size = code[1] as usize;
            let mut num = 0 as u128;

            for i in 0..size {
                num += (code[2 + i] as u128) << i * 8;
            }

            state.stack.push(num);

            *code = &code[2 + size..]
        }
        Ok(Code::Load8) => {
            let offset = state.stack.pop().unwrap() as usize;

            state.stack.push(state.memory[offset] as u128);

            *code = &code[1..];
        }
        Ok(Code::Load16) => {
            let offset = state.stack.pop().unwrap() as usize;

            state.stack.push(u16::from_ne_bytes(
                state.memory[offset..offset + 2].try_into().unwrap(),
            ) as u128);

            *code = &code[1..];
        }
        Ok(Code::Load32) => {
            let offset = state.stack.pop().unwrap() as usize;

            state.stack.push(u32::from_ne_bytes(
                state.memory[offset..offset + 4].try_into().unwrap(),
            ) as u128);

            *code = &code[1..];
        }
        Ok(Code::Load64) => {
            let offset = state.stack.pop().unwrap() as usize;

            state.stack.push(u64::from_ne_bytes(
                state.memory[offset..offset + 8].try_into().unwrap(),
            ) as u128);

            *code = &code[1..];
        }
        Ok(Code::Save8) => {
            let data = state.stack.pop().unwrap();
            let offset = state.stack.pop().unwrap() as usize;

            state.memory.resize(offset + 1, 0);

            state.memory[offset] = data as u8;
            *code = &code[1..];
        }
        Ok(Code::Save16) => {
            let data = state.stack.pop().unwrap();
            let offset = state.stack.pop().unwrap() as usize;

            state.memory.resize(offset + 2, 0);

            state.memory[offset..offset + 2].copy_from_slice(&data.to_ne_bytes()[0..2]);
            *code = &code[1..];
        }
        Ok(Code::Save32) => {
            let data = state.stack.pop().unwrap();
            let offset = state.stack.pop().unwrap() as usize;

            state.memory.resize(offset + 4, 0);

            state.memory[offset..offset + 4].copy_from_slice(&data.to_ne_bytes()[0..4]);
            *code = &code[1..];
        }
        Ok(Code::Save64) => {
            let data = state.stack.pop().unwrap();
            let offset = state.stack.pop().unwrap() as usize;

            state.memory.resize(offset + 8, 0);

            state.memory[offset..offset + 8].copy_from_slice(&data.to_ne_bytes()[0..8]);
            *code = &code[1..];
        }
        Ok(Code::Xor) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push(a ^ b);
            *code = &code[1..];
        }
        Ok(Code::Or) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push(a | b);
            *code = &code[1..];
        }
        Ok(Code::And) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push(a & b);
            *code = &code[1..];
        }
        Ok(Code::Not) => {
            let a = state.stack.pop().unwrap();

            state.stack.push(!a);
            *code = &code[1..];
        }
        Ok(Code::Shift) => {
            let a = &state.stack.pop().unwrap();
            let b = i128::from_ne_bytes(state.stack.pop().unwrap().to_ne_bytes());

            if b > 0 {
                state.stack.push(a << (b as usize));
            } else {
                state.stack.push(a >> (b as usize));
            }

            *code = &code[1..];
        }
        Ok(Code::IShift) => {
            let a = i128::from_ne_bytes(state.stack.pop().unwrap().to_ne_bytes());
            let b = i128::from_ne_bytes(state.stack.pop().unwrap().to_ne_bytes());

            if b > 0 {
                state
                    .stack
                    .push(u128::from_ne_bytes((a << (b as usize)).to_ne_bytes()));
            } else {
                state
                    .stack
                    .push(u128::from_ne_bytes((a >> (b as usize)).to_ne_bytes()));
            }

            *code = &code[1..];
        }
        Ok(Code::Add) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push(a + b);
            *code = &code[1..];
        }
        Ok(Code::Sub) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push(a - b);
            *code = &code[1..];
        }
        Ok(Code::Mul) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push(a * b);
            *code = &code[1..];
        }
        Ok(Code::Div) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push(a / b);
            *code = &code[1..];
        }
        Ok(Code::IMul) => {
            let a = i128::from_ne_bytes(state.stack.pop().unwrap().to_ne_bytes());
            let b = i128::from_ne_bytes(state.stack.pop().unwrap().to_ne_bytes());

            state.stack.push(u128::from_ne_bytes((a * b).to_ne_bytes()));
            *code = &code[1..];
        }
        Ok(Code::IDiv) => {
            let a = i128::from_ne_bytes(state.stack.pop().unwrap().to_ne_bytes());
            let b = i128::from_ne_bytes(state.stack.pop().unwrap().to_ne_bytes());

            state.stack.push(u128::from_ne_bytes((a / b).to_ne_bytes()));
            *code = &code[1..];
        }
        Ok(Code::Mod) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push(a % b);
            *code = &code[1..];
        }
        Ok(Code::Neg) => {
            let a = i128::from_ne_bytes(state.stack.pop().unwrap().to_ne_bytes());

            state.stack.push(u128::from_ne_bytes((-a).to_ne_bytes()));
            *code = &code[1..];
        }
        Ok(Code::FAdd) => {
            let a = f32::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..4]
                    .try_into()
                    .unwrap(),
            );
            let b = f32::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..4]
                    .try_into()
                    .unwrap(),
            );

            state
                .stack
                .push(u32::from_ne_bytes((a + b).to_ne_bytes()) as u128);
            *code = &code[1..];
        }
        Ok(Code::FSub) => {
            let a = f32::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..4]
                    .try_into()
                    .unwrap(),
            );
            let b = f32::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..4]
                    .try_into()
                    .unwrap(),
            );

            state
                .stack
                .push(u32::from_ne_bytes((a - b).to_ne_bytes()) as u128);
            *code = &code[1..];
        }
        Ok(Code::FMul) => {
            let a = f32::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..4]
                    .try_into()
                    .unwrap(),
            );
            let b = f32::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..4]
                    .try_into()
                    .unwrap(),
            );

            state
                .stack
                .push(u32::from_ne_bytes((a * b).to_ne_bytes()) as u128);
            *code = &code[1..];
        }
        Ok(Code::FDiv) => {
            let a = f32::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..4]
                    .try_into()
                    .unwrap(),
            );
            let b = f32::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..4]
                    .try_into()
                    .unwrap(),
            );

            state
                .stack
                .push(u32::from_ne_bytes((a / b).to_ne_bytes()) as u128);
            *code = &code[1..];
        }
        Ok(Code::FNeg) => {
            let a = f32::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..4]
                    .try_into()
                    .unwrap(),
            );

            state
                .stack
                .push(u32::from_ne_bytes((-a).to_ne_bytes()) as u128);
            *code = &code[1..];
        }

        Ok(Code::DAdd) => {
            let a = f64::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..8]
                    .try_into()
                    .unwrap(),
            );
            let b = f64::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..8]
                    .try_into()
                    .unwrap(),
            );

            state
                .stack
                .push(u64::from_ne_bytes((a + b).to_ne_bytes()) as u128);
            *code = &code[1..];
        }
        Ok(Code::DSub) => {
            let a = f64::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..8]
                    .try_into()
                    .unwrap(),
            );
            let b = f64::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..8]
                    .try_into()
                    .unwrap(),
            );

            state
                .stack
                .push(u64::from_ne_bytes((a - b).to_ne_bytes()) as u128);
            *code = &code[1..];
        }
        Ok(Code::DMul) => {
            let a = f64::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..8]
                    .try_into()
                    .unwrap(),
            );
            let b = f64::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..8]
                    .try_into()
                    .unwrap(),
            );

            state
                .stack
                .push(u64::from_ne_bytes((a * b).to_ne_bytes()) as u128);
            *code = &code[1..];
        }
        Ok(Code::DDiv) => {
            let a = f64::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..8]
                    .try_into()
                    .unwrap(),
            );
            let b = f64::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..8]
                    .try_into()
                    .unwrap(),
            );

            state
                .stack
                .push(u64::from_ne_bytes((a / b).to_ne_bytes()) as u128);
            *code = &code[1..];
        }
        Ok(Code::DNeg) => {
            let a = f64::from_ne_bytes(
                state.stack.pop().unwrap().to_ne_bytes()[0..8]
                    .try_into()
                    .unwrap(),
            );

            state
                .stack
                .push(u64::from_ne_bytes((-a).to_ne_bytes()) as u128);
            *code = &code[1..];
        }

        Ok(Code::Lt) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push((a < b) as u128);
            *code = &code[1..];
        }
        Ok(Code::Lte) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push((a <= b) as u128);
            *code = &code[1..];
        }
        Ok(Code::Gt) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push((a > b) as u128);
            *code = &code[1..];
        }
        Ok(Code::Gte) => {
            let a = state.stack.pop().unwrap();
            let b = state.stack.pop().unwrap();

            state.stack.push((a >= b) as u128);
            *code = &code[1..];
        }

        Ok(Code::FLt) => {
            // let a = f32::from_ne_bytes(
            //     state.stack.pop().unwrap().to_ne_bytes()[0..8]
            //         .try_into()
            //         .unwrap(),
            // );
            // let b = f32::from_ne_bytes(
            //     state.stack.pop().unwrap().to_ne_bytes()[0..8]
            //         .try_into()
            //         .unwrap(),
            // );

            // state
            //     .stack
            //     .push(u64::from_ne_bytes((a / b).to_ne_bytes()) as u128);
            // *code = &code[1..];
        }
        Ok(Code::FLte) => todo!(),
        Ok(Code::FGt) => todo!(),
        Ok(Code::FGte) => todo!(),

        Ok(Code::DLt) => todo!(),
        Ok(Code::DLte) => todo!(),
        Ok(Code::DGt) => todo!(),
        Ok(Code::DGte) => todo!(),

        Ok(Code::Eql) => todo!(),

        Ok(Code::Return) => todo!(),

        Ok(Code::Call) => todo!(),
        Ok(Code::Jump) => todo!(),

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
