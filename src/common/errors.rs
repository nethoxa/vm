#[derive(Debug)]
pub enum CommonError {
	OutOfBoundsAccess
}

#[derive(Debug)]
pub enum MathError {
	Overflow,
	Underflow,
	DivisionByZero
}

#[derive(Debug, Clone)]
pub enum VirtualMachineError {
	NOP,
	ProgramCounterOverflow
}
