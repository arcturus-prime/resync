use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, Copy, Clone, TryFromPrimitive)]
pub enum Code {
    Nop = 0,
    Data, // size, data

    // offset,
    Load8,
    Load16,
    Load32,
    Load64,
    // data, offset
    Save8,
    Save16,
    Save32,
    Save64,

    Xor,
    Or,
    And,
    Not,
    Shift,
    IShift,

    Add,
    Sub,
    Mul,
    Div,
    IMul,
    IDiv,
    Mod,
    Neg,

    FAdd,
    FSub,
    FMul,
    FDiv,
    FNeg,

    DAdd,
    DSub,
    DMul,
    DDiv,
    DNeg,

    Lt,
    Lte,
    Gt,
    Gte,

    FLt,
    FLte,
    FGt,
    FGte,

    DLt,
    DLte,
    DGt,
    DGte,

    Eql,

    Return,

    // num, ...
    Call,
    // num, ...
    Jump,
}

pub const STACK_DEPTH_INFO: [isize; 28] = [
    (0),
    (0),
    (-2),
    (0),
    (0),
    (0),
    (-1),
    (-1),
    (-1),
    (0),
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
