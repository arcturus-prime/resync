use std::iter::Rev;

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

pub struct BlockIter<'a> {
    block: &'a Block,
    index: usize,
}

impl Block {
    pub fn push_code(&mut self, code: Code) -> () {
        self.code.push(code);
        self.data.push(Data { uint: 0 });
    }

    pub fn push_data(&mut self, data: Data) -> () {
        self.code.push(Code::Data);
        self.data.push(data);
    }

    pub fn iter(&self) -> BlockIter {
        BlockIter {
            block: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = (&'a Code, &'a Data);

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        if self.index == self.block.code.len() {
            return None;
        }

        Some((&self.block.code[self.index], &self.block.data[self.index]))
    }
}

impl<'a> DoubleEndedIterator for BlockIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            return None;
        }

        self.index -= 1;

        Some((&self.block.code[self.index], &self.block.data[self.index]))
    }
}
