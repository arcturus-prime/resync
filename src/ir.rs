use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, Copy, Clone, TryFromPrimitive)]
pub enum Code {
    //
    // size, data[size]
    Data = 0,
    // offset, size
    // file
    Load,
    // data, offset
    // file
    Save,

    Xor,
    Or,
    And,
    Not,
    Shift,

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,

    Lt,
    Lte,
    Gt,
    Gte,
    Eql,

    Return,

    // bool, dest
    VCall,
    // bool, dest
    VJump,

    // bool
    Call,
    // bool
    Jump,
    Nop,
}

pub const STACK_DEPTH_INFO: [isize; 28] = [
    (0),
    (1),
    (1),
    (1),
    (1),
    (0),
    (-1),
    (-2),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-1),
    (-2),
    (-2),
    (-1),
    (-1),
];
