pub type NodeRef = u8;
pub type ParamRef = u8;
pub type Size = u8;
pub type ConstRef = u16;

// BODY
// ------
// code

// FOOTER
// --------
// condition
// blocktrue
// blockfalse

pub enum Node {
    Nop,

    Const(Size, ConstRef),
    Param(Size, ParamRef),

    Copy(NodeRef),

    Load(NodeRef),
    Store(NodeRef, NodeRef),

    Add(NodeRef, NodeRef),
    Sub(NodeRef, NodeRef),
    Mul(NodeRef, NodeRef),
    Div(NodeRef, NodeRef),
    Mod(NodeRef, NodeRef),
    Lt(NodeRef, NodeRef),
    Lte(NodeRef, NodeRef),
    Gt(NodeRef, NodeRef),
    Gte(NodeRef, NodeRef),
    Neg(NodeRef),

    ILt(NodeRef, NodeRef),
    ILte(NodeRef, NodeRef),
    IGt(NodeRef, NodeRef),
    IGte(NodeRef, NodeRef),

    FAdd(NodeRef, NodeRef),
    FSub(NodeRef, NodeRef),
    FMul(NodeRef, NodeRef),
    FDiv(NodeRef, NodeRef),
    FLt(NodeRef, NodeRef),
    FLte(NodeRef, NodeRef),
    FGt(NodeRef, NodeRef),
    FGte(NodeRef, NodeRef),
    FNeg(NodeRef),

    Equ(NodeRef, NodeRef),

    And(NodeRef, NodeRef),
    Or(NodeRef, NodeRef),
    Xor(NodeRef, NodeRef),
    Not(NodeRef),

    Call(NodeRef),
}

pub struct Block {
    pub name: String,

    pub nodes: Vec<Node>,
    pub constants: Vec<[u8; 8]>,
}

pub struct Program {
    pub blocks: Vec<Block>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            name: String::new(),

            nodes: Vec::new(),
            constants: Vec::new(),
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }
}
