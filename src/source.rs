pub type NodeRef = u8;
pub type ConstRef = u16;
pub type Size = u8;

pub enum Node {
    Nop,

    Const(ConstRef),

    Copy(NodeRef),

    Load(NodeRef, Size),
    Store(NodeRef, Size),

    Add(NodeRef, NodeRef, Size),
    Sub(NodeRef, NodeRef, Size),
    Mul(NodeRef, NodeRef, Size),
    Div(NodeRef, NodeRef, Size),
    Mod(NodeRef, NodeRef, Size),
    Lt(NodeRef, NodeRef, Size),  
    Lte(NodeRef, NodeRef, Size),
    Gt(NodeRef, NodeRef, Size),
    Gte(NodeRef, NodeRef, Size),
    Neg(NodeRef, Size),

    ILt(NodeRef, NodeRef, Size),  
    ILte(NodeRef, NodeRef, Size),
    IGt(NodeRef, NodeRef, Size),
    IGte(NodeRef, NodeRef, Size),

    FAdd(NodeRef, NodeRef, Size),
    FSub(NodeRef, NodeRef, Size),
    FMul(NodeRef, NodeRef, Size),
    FDiv(NodeRef, NodeRef, Size),
    FLt(NodeRef, NodeRef, Size),  
    FLte(NodeRef, NodeRef, Size),
    FGt(NodeRef, NodeRef, Size),
    FGte(NodeRef, NodeRef, Size),
    FNeg(NodeRef, Size),

    Equ(NodeRef, NodeRef, Size),

    And(NodeRef, NodeRef, Size),
    Or(NodeRef, NodeRef, Size),
    Xor(NodeRef, NodeRef, Size),
    Not(NodeRef),

    Call(NodeRef),
}

pub enum Constant {
    Uint(usize),
    Int(isize),
    Float(f32),
    Double(f64),
}

pub struct Function {
    pub name: String,

    pub nodes: Vec<Node>,
    pub constants: Vec<Constant>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            name: String::new(),

            nodes: Vec::new(),
            constants: Vec::new(),
        }
    }
}
