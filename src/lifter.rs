use crate::ir::Block;

pub trait Lifter<InputCode> {
	fn lift(code: InputCode) -> Block;
}