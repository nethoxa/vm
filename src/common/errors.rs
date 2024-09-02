#[derive(Debug)]
pub enum MathError {
	Overflow,
	Underflow,
	DivisionByZero
}

#[derive(Debug, Clone)]
pub enum VirtualMachineError {
	NOP,
	ProgramCounterOverflow,
	RegistryOutOfBoundsAccess,
	MalformedInstruction,
	VirtualMachinePanicked,
	InvalidPadding,
	InvalidJump
}
