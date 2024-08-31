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
	/// - 16..24 is the second operando
	/// - 24..32 is the destination registry
	ADD,

	/// Mul operation
	///
	/// - 0..8   is the opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operando
	/// - 24..32 is the destination registry
	MUL,

	/// Sub operation
	///
	/// - 0..8   is the opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operando
	/// - 24..32 is the destination registry
	SUB,

	/// Div operation
	///
	/// - 0..8   is the opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operando
	/// - 24..32 is the destination registry
	DIV,

	/// Rem operation
	///
	/// - 0..8   is the opcode
	/// - 8..16  is the first operand
	/// - 16..24 is the second operando
	/// - 24..32 is the destination registry
	REM,

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

			_ => Opcode::NIL
		}
	}
}
