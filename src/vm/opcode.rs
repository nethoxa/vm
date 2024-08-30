#[derive(Debug, PartialEq)]
/// Abstraction over the operations the `VM` supports
pub enum Opcode {
    /// Halt execution
	HLT,
    /// Load an `u16` number into the given register
    LOAD,

    /// `Opcode` not supported
	NIL
}

impl From<u8> for Opcode {
    /// Util to cast an `u8` to an `Opcode`
	fn from(value: u8) -> Self {
		match value {
			0 => Opcode::HLT,
            1 => Opcode::LOAD,

			_ => Opcode::NIL
		}
	}
}
