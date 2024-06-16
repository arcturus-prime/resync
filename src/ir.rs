use std::mem;

#[derive(Debug)]
pub enum Code {
    Data,
    Load,
    Save,

    // data, offset (bits), size (bits)
    Extract,
    // patch, data, offset (bits)
    Insert,

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

    Return,

    // bool, blocktrue, blockfalse
    Call,
    // bool, blocktrue, blockfalse
    Jump,
}

pub union Data {
    pub none: (),
    pub int: isize,
    pub uint: usize,
    pub double: f64,
    pub float: f32,
}

impl Default for Data {
    fn default() -> Self {
        Self { none: () }
    }
}

pub struct Block {
    code: Vec<Code>,
    data: Vec<Data>,
}

pub struct BlockIterMut<'a> {
    code: &'a mut [Code],
    data: &'a mut [Data],
}

pub struct BlockIter<'a> {
    code: &'a [Code],
    data: &'a [Data],
}

impl Block {
    pub fn new() -> Block {
        Block {
            code: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, code: Code, data: Data) -> () {
        self.code.push(code);
        self.data.push(data);
    }

    pub fn iter(&self) -> BlockIter {
        BlockIter {
            code: self.code.as_slice(),
            data: self.data.as_slice(),
        }
    }

    pub fn iter_mut(&mut self) -> BlockIterMut {
        BlockIterMut {
            code: self.code.as_mut_slice(),
            data: self.data.as_mut_slice(),
        }
    }
}

impl<'a> Iterator for BlockIterMut<'a> {
    type Item = (&'a mut Code, &'a mut Data);

    fn next(&mut self) -> Option<Self::Item> {
        let code = mem::take(&mut self.code);
        let data = mem::take(&mut self.data);

        if code.is_empty() || data.is_empty() {
            return None;
        }

        let (code_l, code_r) = code.split_at_mut(1);
        let (data_l, data_r) = data.split_at_mut(1);

        self.code = code_r;
        self.data = data_r;

        Some((code_l.get_mut(0).unwrap(), data_l.get_mut(0).unwrap()))
    }
}

impl<'a> DoubleEndedIterator for BlockIterMut<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let code = mem::take(&mut self.code);
        let data = mem::take(&mut self.data);

        if code.is_empty() || data.is_empty() {
            return None;
        }

        let (code_l, code_r) = code.split_at_mut(code.len() - 1);
        let (data_l, data_r) = data.split_at_mut(data.len() - 1);

        self.code = code_l;
        self.data = data_l;

        Some((code_r.get_mut(0).unwrap(), data_r.get_mut(0).unwrap()))
    }
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = (&'a Code, &'a Data);

    fn next(&mut self) -> Option<Self::Item> {
        if self.code.is_empty() || self.data.is_empty() {
            return None;
        }

        let (code_l, code_r) = self.code.split_at(1);
        let (data_l, data_r) = self.data.split_at(1);

        self.code = code_r;
        self.data = data_r;

        Some((&code_l[0], &data_l[0]))
    }
}

impl<'a> DoubleEndedIterator for BlockIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.code.is_empty() || self.data.is_empty() {
            return None;
        }

        let (code_l, code_r) = self.code.split_at(self.code.len() - 1);
        let (data_l, data_r) = self.data.split_at(self.data.len() - 1);

        self.code = code_l;
        self.data = data_l;

        Some((&code_r[0], &data_r[0]))
    }
}
