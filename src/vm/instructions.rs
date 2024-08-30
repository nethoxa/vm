use crate::vm::opcode::Opcode;

#[derive(Debug, PartialEq)]
/// The computing unit of the `VM`
pub struct Instruction {
	opcode: Opcode
}

impl Instruction {
	/// Creates a new `Instruction` with the given `Opcode`
	fn new(opcode: Opcode) -> Instruction {
		Instruction {
			opcode
		}
	}
}
