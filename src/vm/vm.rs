use crate::vm::opcode::Opcode;
use crate::{debug, error, info, warn};

#[derive(Debug)]
/// Registry-based virtual machine
pub struct VM {
	registers: [u32; 32],
	pc:        usize,
	program:   Vec<u8>
}

impl VM {
	/// Creates a new `VM` with all fields set to `0`
	pub fn new() -> VM {
		VM {
			registers: [0u32; 32],
			pc:        0,
			program:   vec![]
		}
	}

	/// Fetches the byte at `self.pc`, cast it into an `Opcode` and increments
	/// `self.pc` by `1`
	fn decode_opcode(&mut self) -> Opcode {
		let opcode = Opcode::from(self.program[self.pc]);
		self.pc += 1;
		opcode
	}

	/// Fetches the byte at `self.pc` and increments `self.pc` by `1`
	fn next_8_bits(&mut self) -> u8 {
		let result = self.program[self.pc];
		self.pc += 1;
		result
	}

	/// Fetches two bytes starting at `self.pc` and increments `self.pc` by `2`
	fn next_16_bits(&mut self) -> u16 {
		let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
		self.pc += 2;
		result
	}

	fn get_register(&mut self) -> Option<usize> {
		let register = self.next_8_bits() as usize;
		if register >= self.registers.len() {
			error!(format!("Registry OOB access, panic"));
			None
		} else {
			Some(register)
		}
	}

	/// Main function of the `VM`
	fn run(&mut self) -> Option<()> {
		loop {
			if self.pc >= self.program.len() {
				error!("Something went wrong, VM program counter > program length");

				break
			}

			match self.decode_opcode() {
				Opcode::HLT => {
					warn!(format!("HLT found at cycle {}, execution aborted", self.pc));

					break;
				},
				Opcode::LOAD => {
					info!(format!("LOAD found at cycle {}", self.pc));

					let register = self.get_register()?;
					let number = self.next_16_bits() as u32;

					self.registers[register] = number;
				},
				Opcode::ADD => {
					info!(format!("ADD found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let b = self.registers[self.get_register()?];
					let dest = self.next_8_bits() as usize;

					self.registers[dest] = a + b;
				},
				Opcode::MUL => {
					info!(format!("MUL found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let b = self.registers[self.get_register()?];
					let dest = self.next_8_bits() as usize;

					self.registers[dest] = a * b;
				},
				Opcode::SUB => {
					info!(format!("SUB found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let b = self.registers[self.get_register()?];
					let dest = self.next_8_bits() as usize;

					self.registers[dest] = a - b;
				},
				Opcode::DIV => {
					info!(format!("DIV found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let b = self.registers[self.get_register()?];
					let dest = self.next_8_bits() as usize;

					self.registers[dest] = a / b;
				},
				Opcode::REM => {
					info!(format!("REM found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let b = self.registers[self.get_register()?];
					let dest = self.next_8_bits() as usize;

					self.registers[dest] = a % b;
				},
				Opcode::NIL => {
					error!(format!("NIL found at cycle {}, panic", self.pc));
					break
				}
			}
		}

		Some(())
	}
}

#[cfg(test)]
mod vm_tests {
	use super::*;

	#[test]
	fn test_load_opcode() {
		let mut vm = VM::new();

		// should load in register 2, as it is 0-indexed, number 32, and then halt
		vm.program = vec![1, 2, 0, 32, 0];

		vm.run();
		assert_eq!(vm.registers[2], 32);
	}

	#[test]
	fn test_math_opcodes() {
		let mut vm = VM::new();

		// should to the following
		// - load 20 in registry 1
		// - load 10 in registry 2
		// - r3 = r1 + r2
		// - r4 = r1 * r2
		// - r5 = r1 - r2
		// - r6 = r1 / r2
		// - r7 = r1 % r2
		vm.program = vec![
			1, 1, 0, 20, 1, 2, 0, 10, 2, 1, 2, 3, 3, 1, 2, 4, 4, 1, 2, 5, 5, 1, 2, 6, 6, 1, 2, 7, 0,
		];

		vm.run();
		assert_eq!(vm.registers[0..8], vec![0, 20, 10, 30, 200, 10, 2, 0]);
	}
}
