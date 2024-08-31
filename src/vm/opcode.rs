#[derive(Debug, PartialEq)]
/// Abstraction over the operations the `VM` supports
pub enum Opcode {
	/// Halt execution
	HLT,
	/// Load an `u16` number into the given register
	///
	/// - 0..8   is the opcode
	/// - 8..16  is the destination registry
	/// - 16..32 is the inmediate value
	LOAD,

	/// Add operation
	///
	/// - 0..8   is the opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operand
	/// - 24..32 is the destination registry
	ADD,

	/// Mul operation
	///
	/// - 0..8   is the opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operand
	/// - 24..32 is the destination registry
	MUL,

	/// Sub operation
	///
	/// - 0..8   is the opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operand
	/// - 24..32 is the destination registry
	SUB,

	/// Div operation
	///
	/// - 0..8   is the opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operand
	/// - 24..32 is the destination registry
	DIV,

	/// Rem operation
	///
	/// - 0..8   is the opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operand
	/// - 24..32 is the destination registry
	REM,

	/// Jmp operation
	///
	/// - 0..8   is the opcode
	/// - 8..24  is the destination PC
	/// - 24..32 is filled with 0s
	JMP,

	/// EQ operation
	/// - 0..8   opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operand
	/// - 24..32 is the destination registry
	EQ,

	/// LT operation
	/// - 0..8   opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operand
	/// - 24..32 is the destination registry
	LT,

	/// GT operation
	/// - 0..8   opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operand
	/// - 24..32 is the destination registry
	GT,

	/// AND operation
	/// - 0..8   opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operand
	/// - 24..32 is the destination registry
	AND,

	/// OR operation
	/// - 0..8   opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operand
	/// - 24..32 is the destination registry
	OR,

	/// XOR operation
	/// - 0..8   opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operand
	/// - 24..32 is the destination registry
	XOR,

	/// NOT operation
	/// - 0..8   opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the destination registry
	/// - 24..32 is filled with 0s
	NOT,

	/// NOP operation
	NOP,

	/// `Opcode` not supported
	NIL
}

impl From<u8> for Opcode {
	/// Util to cast an `u8` to an `Opcode`
	fn from(value: u8) -> Self {
		match value {
			0 => Opcode::HLT,
			1 => Opcode::LOAD,
			2 => Opcode::ADD,
			3 => Opcode::MUL,
			4 => Opcode::SUB,
			5 => Opcode::DIV,
			6 => Opcode::REM,
			7 => Opcode::JMP,
			8 => Opcode::EQ,
			9 => Opcode::LT,
			10 => Opcode::GT,
			11 => Opcode::AND,
			12 => Opcode::OR,
			13 => Opcode::XOR,
			14 => Opcode::NOT,
			15 => Opcode::NOP,

			_ => Opcode::NIL
		}
	}
}
