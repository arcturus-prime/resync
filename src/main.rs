use binal::analysis::emulate;
use binal::ir::Code;

pub fn main() -> () {
    let code = vec![
        Code::Data as u8,
        5,
        255,
        21,
        32,
        23,
        0,
        Code::Data as u8,
        1,
        0x0F,
        Code::Save as u8,
        0,
        Code::Data as u8,
        1,
        0x14,
        Code::Data as u8,
        1,
        0x01,
        Code::Load as u8,
        0,
        Code::Neg as u8,
    ];

    let state = emulate(code.as_slice());

    println!("{:?}", state.stack.last().unwrap().as_slice());
}
