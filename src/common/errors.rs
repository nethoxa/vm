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

#[derive(Debug)]
pub enum VirtualMachineError {}

#[derive(Debug)]
pub enum BlockchainError {}
