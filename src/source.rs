pub type NodeRef = u16;
pub type ConstRef = u16;
pub type VarRef = u16;
pub type ParamRef = u8;

pub enum Node {
    Const(ConstRef),
    Var(VarRef),
    Param(ParamRef),

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

    IMul(NodeRef, NodeRef),
    IDiv(NodeRef, NodeRef),
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

    DAdd(NodeRef, NodeRef),
    DSub(NodeRef, NodeRef),
    DMul(NodeRef, NodeRef),
    DDiv(NodeRef, NodeRef),
    DLt(NodeRef, NodeRef),
    DLte(NodeRef, NodeRef),
    DGt(NodeRef, NodeRef),
    DGte(NodeRef, NodeRef),
    DNeg(NodeRef),

    And(NodeRef, NodeRef),
    Or(NodeRef, NodeRef),
    Xor(NodeRef, NodeRef),
    Not(NodeRef),

    Equ(NodeRef, NodeRef),
    NotEqu(NodeRef, NodeRef),

    BNeg(NodeRef),

    Assign(NodeRef, NodeRef),
    AddAssign(NodeRef, NodeRef),
    SubAssign(NodeRef, NodeRef),
    MulAssign(NodeRef, NodeRef),
    DivAssign(NodeRef, NodeRef),
    IMulAssign(NodeRef, NodeRef),
    IDivAssign(NodeRef, NodeRef),
    FAddAssign(NodeRef, NodeRef),
    FSubAssign(NodeRef, NodeRef),
    FMulAssign(NodeRef, NodeRef),
    FDivAssign(NodeRef, NodeRef),
    DAddAssign(NodeRef, NodeRef),
    DSubAssign(NodeRef, NodeRef),
    DMulAssign(NodeRef, NodeRef),
    DDivAssign(NodeRef, NodeRef),
    AndAssign(NodeRef, NodeRef),
    OrAssign(NodeRef, NodeRef),
    XorAssign(NodeRef, NodeRef),

    Return(NodeRef),

    Call {
        target: NodeRef,
        arguments: NodeRef,
    },
    Deref(NodeRef),
    Index(NodeRef, NodeRef),

    StatementGroup(NodeRef, NodeRef),
    ExpressionGroup(NodeRef, NodeRef),

    While {
        condition: NodeRef,
        block: NodeRef,
    },
    DoWhile {
        condition: NodeRef,
        block: NodeRef,
    },
    If {
        condition: NodeRef,
        block: NodeRef,
    },
    ElseIf {
        previous: NodeRef,
        condition: NodeRef,
        block: NodeRef,
    },
    Else {
        previous: NodeRef,
        block: NodeRef,
    },
}

pub enum Constant {
    Uint(usize),
    Int(isize),
    Float(f32),
    Double(f64),
}

pub struct Variable {
    pub name: String,
    pub size: usize,
}

pub struct Function {
    pub name: String,
    pub size: usize,

    pub nodes: Vec<Node>,
    pub constants: Vec<Constant>,
    pub variables: Vec<Variable>,
    pub parameters: Vec<Variable>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            size: 8,

            nodes: Vec::new(),
            constants: Vec::new(),
            variables: Vec::new(),
            parameters: Vec::new(),
        }
    }

    pub fn print(&self) -> String {
        let mut output = String::new();

        output.push_str(self.size.to_string().as_str());
        output.push_str(" ");
        output.push_str(&self.name);
        output.push_str("(");

        for index in 0..self.parameters.len() {
            output.push_str(&self.parameters[index].name);
            output.push_str(": ");
            output.push_str(self.parameters[index].size.to_string().as_str());

            if index < self.parameters.len() - 1 {
                output.push_str(", ");
            }
        }

        output.push_str(") {\n");
        self.print_node(&mut output, 1, 0);
        output.push_str("}");

        output
    }

    fn print_node(&self, output: &mut String, depth: usize, node_ref: NodeRef) {
        match self.nodes[node_ref as usize] {
            Node::Add(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" + ");
                self.print_node(output, depth, node2);
            }
            Node::Sub(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" - ");
                self.print_node(output, depth, node2);
            }
            Node::Mul(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" * ");
                self.print_node(output, depth, node2);
            }
            Node::Div(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" / ");
                self.print_node(output, depth, node2);
            }
            Node::Mod(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" % ");
                self.print_node(output, depth, node2);
            }
            Node::Neg(node) => {
                output.push_str("-");
                self.print_node(output, depth, node);
            }
            Node::Lt(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" < ");
                self.print_node(output, depth, node2);
            }
            Node::Lte(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" <= ");
                self.print_node(output, depth, node2);
            }
            Node::Gt(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" > ");
                self.print_node(output, depth, node2);
            }
            Node::Gte(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" >= ");
                self.print_node(output, depth, node2);
            }
            Node::IMul(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" i* ");
                self.print_node(output, depth, node2);
            }
            Node::IDiv(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" i/ ");
                self.print_node(output, depth, node2);
            }
            Node::ILt(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" i< ");
                self.print_node(output, depth, node2);
            }
            Node::ILte(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" i<= ");
                self.print_node(output, depth, node2);
            }
            Node::IGt(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" i> ");
                self.print_node(output, depth, node2);
            }
            Node::IGte(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" i>= ");
                self.print_node(output, depth, node2);
            }
            Node::FAdd(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f+ ");
                self.print_node(output, depth, node2);
            }
            Node::FSub(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f- ");
                self.print_node(output, depth, node2);
            }
            Node::FMul(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f* ");
                self.print_node(output, depth, node2);
            }
            Node::FDiv(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f/ ");
                self.print_node(output, depth, node2);
            }
            Node::FNeg(node) => {
                output.push_str("f-");
                self.print_node(output, depth, node);
            }
            Node::FLt(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f< ");
                self.print_node(output, depth, node2);
            }
            Node::FLte(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f<= ");
                self.print_node(output, depth, node2);
            }
            Node::FGt(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f> ");
                self.print_node(output, depth, node2);
            }
            Node::FGte(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f>= ");
                self.print_node(output, depth, node2);
            }
            Node::DAdd(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d+ ");
                self.print_node(output, depth, node2);
            }
            Node::DSub(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d- ");
                self.print_node(output, depth, node2);
            }
            Node::DMul(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d* ");
                self.print_node(output, depth, node2);
            }
            Node::DDiv(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d/ ");
                self.print_node(output, depth, node2);
            }
            Node::DNeg(node) => {
                output.push_str("d-");
                self.print_node(output, depth, node);
            }
            Node::DLt(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d< ");
                self.print_node(output, depth, node2);
            }
            Node::DLte(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d<= ");
                self.print_node(output, depth, node2);
            }
            Node::DGt(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d> ");
                self.print_node(output, depth, node2);
            }
            Node::DGte(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d>= ");
                self.print_node(output, depth, node2);
            }
            Node::And(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" & ");
                self.print_node(output, depth, node2);
            }
            Node::Or(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" | ");
                self.print_node(output, depth, node2);
            }
            Node::Xor(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" ^ ");
                self.print_node(output, depth, node2);
            }
            Node::Not(node) => {
                output.push_str("~");
                self.print_node(output, depth, node);
            }
            Node::Equ(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" == ");
                self.print_node(output, depth, node2);
            }
            Node::NotEqu(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" != ");
                self.print_node(output, depth, node2);
            }
            Node::BNeg(node) => {
                output.push_str("!");
                self.print_node(output, depth, node);
            }
            Node::Assign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" = ");
                self.print_node(output, depth, node2);
            }
            Node::AddAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" += ");
                self.print_node(output, depth, node2);
            }
            Node::SubAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" -= ");
                self.print_node(output, depth, node2);
            }
            Node::MulAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" *= ");
                self.print_node(output, depth, node2);
            }
            Node::DivAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" /= ");
                self.print_node(output, depth, node2);
            }
            Node::IMulAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" i*= ");
                self.print_node(output, depth, node2);
            }
            Node::IDivAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" i/= ");
                self.print_node(output, depth, node2);
            }
            Node::FAddAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f+= ");
                self.print_node(output, depth, node2);
            }
            Node::FSubAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f-= ");
                self.print_node(output, depth, node2);
            }
            Node::FMulAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f*= ");
                self.print_node(output, depth, node2);
            }
            Node::FDivAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" f/= ");
                self.print_node(output, depth, node2);
            }
            Node::DAddAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d+= ");
                self.print_node(output, depth, node2);
            }
            Node::DSubAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d-= ");
                self.print_node(output, depth, node2);
            }
            Node::DMulAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d*= ");
                self.print_node(output, depth, node2);
            }
            Node::DDivAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" d/= ");
                self.print_node(output, depth, node2);
            }
            Node::AndAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" &= ");
                self.print_node(output, depth, node2);
            }
            Node::OrAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" |= ");
                self.print_node(output, depth, node2);
            }
            Node::XorAssign(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str(" ^= ");
                self.print_node(output, depth, node2);
            }
            Node::Call { target, arguments } => {
                self.print_node(output, depth, target);
                output.push_str("(");
                self.print_node(output, depth, arguments);
                output.push_str(")");
            }
            Node::Deref(node) => {
                output.push_str("@");
                self.print_node(output, depth, node);
            }
            Node::Index(node1, node2) => {
                self.print_node(output, depth, node1);
                output.push_str("[");
                self.print_node(output, depth, node2);
                output.push_str("]");
            }
            Node::StatementGroup(node1, node2) => {
                for index in node1..node2 {
                    output.push_str(&"\t".repeat(depth));
                    self.print_node(output, depth, index);
                    output.push_str("\n");
                }
            }
            Node::ExpressionGroup(node1, node2) => {
                for index in node1..node2 {
                    self.print_node(output, depth, node1);

                    if index < node2 - 1 {
                        output.push_str(", ");
                    }
                }
            }
            Node::While { condition, block } => {
                output.push_str(&"\t".repeat(depth));
                output.push_str("while (");
                self.print_node(output, depth, condition);
                output.push_str(") {\n");
                self.print_node(output, depth + 1, block);
                output.push_str(&"\t".repeat(depth));
                output.push_str("}\n");
            }
            Node::DoWhile { condition, block } => {
                output.push_str(&"\t".repeat(depth));
                output.push_str("do {\n");
                self.print_node(output, depth + 1, block);
                output.push_str("} while(");
                self.print_node(output, depth, condition);
                output.push_str(")\n");
            }
            Node::If { condition, block } => {
                output.push_str(&"\t".repeat(depth));
                output.push_str("if (");
                self.print_node(output, depth, condition);
                output.push_str(") {\n");
                self.print_node(output, depth + 1, block);
                output.push_str(&"\t".repeat(depth));
                output.push_str("}\n");
            }
            Node::ElseIf {
                previous,
                condition,
                block,
            } => {
                self.print_node(output, depth, previous);
                output.push_str(" else if (");
                self.print_node(output, depth, condition);
                output.push_str(") {\n");
                self.print_node(output, depth + 1, block);
                output.push_str(&"\t".repeat(depth));
                output.push_str("}\n");
            }
            Node::Else { block, previous } => {
                self.print_node(output, depth, previous);
                output.push_str(" else {");
                self.print_node(output, depth + 1, block);
                output.push_str(&"\t".repeat(depth));
                output.push_str("}\n");
            }
            Node::Const(index) => {
                let constant = &self.constants[index as usize];

                match constant {
                    Constant::Uint(value) => output.push_str(value.to_string().as_str()),
                    Constant::Int(value) => output.push_str(value.to_string().as_str()),
                    Constant::Float(value) => output.push_str(value.to_string().as_str()),
                    Constant::Double(value) => output.push_str(value.to_string().as_str()),
                }
            }
            Node::Var(index) => {
                let variable = &self.variables[index as usize];

                output.push_str(variable.name.as_str());
            }
            Node::Param(index) => {
                let parameter = &self.parameters[index as usize];

                output.push_str(parameter.name.as_str());
            }
            Node::Return(node) => {
                output.push_str("return ");
                self.print_node(output, depth, node);
            }
        }
    }
}
