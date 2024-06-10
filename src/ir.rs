pub enum Code {
    Data,
    // data, size
    Resize,

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
    Mod,
    Neg,

    IMul,
    IDiv,

    FAdd,
    FSub,
    FMul,
    FDiv,
    FNeg,

    Lt,
    Lte,
    Gt,
    Gte,
    Eq,

    // file, offset, size
    Load,
    // data, file, offset
    Store,

    Return,

    // bool, blocktrue, blockfalse
    Call,
    // bool, blocktrue, blockfalse
    Jump,
}

pub union Data {
    pub int: isize,
    pub uint: usize,
    pub double: f64,
    pub float: f32,
    pub boolean: bool,
}

pub struct Block {
    code: Vec<Code>,
    data: Vec<Data>,
}

impl Block {
    pub fn push_code(&mut self, code: Code) -> () {
        self.code.push(code);
    }

    pub fn push_data(&mut self, data: Data) -> () {
        self.code.push(Code::Data);
        self.data.push(data);
    }
}
