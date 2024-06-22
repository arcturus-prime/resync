use binal::analysis::emulate;
use binal::ir::Code;

pub fn main() -> () {
    let code = vec![
        Code::Data as u8,
        1,
        0x00,
        Code::Data as u8,
        8,
        0x10,
        0xF4,
        0xA1,
        0x72,
        0xE2,
        0xDE,
        0xE9,
        0x91,
        Code::Save64 as u8,
        Code::Data as u8,
        1,
        0x01,
        Code::Load8 as u8,
    ];

    let state = emulate(code.as_slice());

    println!("{}", state.stack.last().unwrap());
}
