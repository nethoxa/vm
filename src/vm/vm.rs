use bnum::types::U512;

use crate::vm::opcode::Opcode;
use crate::{error, info, warn};

#[derive(Debug, Clone)]
/// Registry-based virtual machine
pub struct VM {
	pub registers: [U512; 32],
	pub pc:        usize,
	pub program:   Vec<u8>,

	pub aborted: bool
}

impl VM {
	/// Creates a new `VM` with all fields set to `0`
	pub fn new() -> VM {
		VM {
			registers: [0u32; 32],
			pc:        0,
			program:   vec![],
			aborted:   false
		}
	}

	pub fn reset(&mut self) {
		self.registers = [0u32; 32];
		self.pc = 0;
		self.program = vec![];
		self.aborted = false;
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
	pub fn run(&mut self) -> Option<()> {
		loop {
			if self.pc >= self.program.len() {
				error!("Something went wrong, VM program counter > program length");
				self.aborted = true;

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
				Opcode::JMP => {
					info!(format!("JMP found at cycle {}", self.pc));

					let dest = self.next_16_bits() as usize;
					let padding = self.next_8_bits() as usize;

					// check padding is filled with 0s
					if padding != 0 {
						error!(format!("Invalid JMP padding, it is not zero"));
						self.aborted = true;

						break;
					}

					// check dest falls in the first instruction byte
					if dest % 4 != 0 {
						error!(format!("Invalid JMP addr, tried jumping to {}", dest));
						self.aborted = true;

						break;
					}
					// check we do not jump OOB of the program
					if dest >= self.program.len() {
						error!(format!(
							"Invalid JMP addr, length of program is {}, tried jumping to {}",
							self.program.len(),
							dest
						));
						self.aborted = true;

						break;
					}

					self.pc = dest;
				},
				Opcode::EQ => {
					info!(format!("EQ found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let b = self.registers[self.get_register()?];
					let dest = self.get_register()?;

					self.registers[dest] = (a == b) as u32;
				},
				Opcode::LT => {
					info!(format!("LT found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let b = self.registers[self.get_register()?];
					let dest = self.get_register()?;

					self.registers[dest] = (a < b) as u32;
				},
				Opcode::GT => {
					info!(format!("GT found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let b = self.registers[self.get_register()?];
					let dest = self.get_register()?;

					self.registers[dest] = (a > b) as u32;
				},
				Opcode::AND => {
					info!(format!("AND found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let b = self.registers[self.get_register()?];
					let dest = self.get_register()?;

					self.registers[dest] = (a & b) as u32;
				},
				Opcode::OR => {
					info!(format!("OR found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let b = self.registers[self.get_register()?];
					let dest = self.get_register()?;

					self.registers[dest] = (a | b) as u32;
				},
				Opcode::XOR => {
					info!(format!("XOR found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let b = self.registers[self.get_register()?];
					let dest = self.get_register()?;

					self.registers[dest] = (a ^ b) as u32;
				},
				Opcode::NOT => {
					info!(format!("NOT found at cycle {}", self.pc));

					let a = self.registers[self.get_register()?];
					let dest = self.get_register()?;
					let padding = self.next_8_bits() as usize;

					// check padding is filled with 0s
					if padding != 0 {
						error!(format!("Invalid NOT padding, it is not zero"));
						self.aborted = true;

						break;
					}
					self.registers[dest] = !a as u32;
				},
				Opcode::NOP => {
					info!(format!(
						"NOP found at cycle {}, waiting for more bytecode",
						self.pc
					));

					break;
				},
				Opcode::NIL => {
					error!(format!("NIL found at cycle {}, panic", self.pc));
					self.aborted = true;

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

		vm.program = vec![
			1, 1, 0, 20, // load 20 in registry 1
			1, 2, 0, 10, // load 10 in registry 2
			2, 1, 2, 3, // r3 = r1 + r2
			3, 1, 2, 4, // r4 = r1 * r2
			4, 1, 2, 5, // r5 = r1 - r2
			5, 1, 2, 6, // r6 = r1 / r2
			6, 1, 2, 7, // r7 = r1 % r2
			0, // HLT
		];

		vm.run();
		assert_eq!(vm.registers[0..8], vec![0, 20, 10, 30, 200, 10, 2, 0]);
	}

	#[test]
	fn test_jump_opcode() {
		let mut vm = VM::new();

		// happy program
		vm.program = vec![
			1, 1, 0, 1, // load 1 in registry 1
			1, 2, 0, 2, // load 2 in registry 2
			2, 3, 1, 3, // r3 = r3 + r1
			2, 3, 2, 3, // r3 = r3 + r2
			7, 0, 28, 0, // pc = 28
			0, 0, 0, 0, // padding to verify it jumps correctly
			0, 0, 0, 0, // padding to verify it jumps correctly
			2, 3, 2, 3, // r3 = r3 + r1
			0, // HLT
		];

		vm.run();
		assert_eq!(vm.registers[0..4], vec![0, 1, 2, 5]);

		vm.reset();

		// test invalid padding
		vm.program = vec![
			1, 1, 0, 1, // load 1 in registry 1
			1, 2, 0, 2, // load 2 in registry 2
			2, 3, 1, 3, // r3 = r3 + r1
			2, 3, 2, 3, // r3 = r3 + r2
			7, 0, 28, 123, // pc = 28
		];

		vm.run();
		assert_eq!(vm.aborted, true);

		vm.reset();

		// test OOB jump
		vm.program = vec![
			7, 0, 100, 0, // pc = 100
		];

		vm.run();
		assert_eq!(vm.aborted, true);

		vm.reset();

		// test invalid jump byte
		vm.program = vec![
			1, 1, 0, 1, // load 1 in registry 1
			1, 2, 0, 2, // load 2 in registry 2
			2, 3, 1, 3, // r3 = r3 + r1
			2, 3, 2, 3, // r3 = r3 + r2
			7, 0, 1, 0, // pc = 1
		];

		vm.run();
		assert_eq!(vm.aborted, true);
	}

	#[test]
	fn test_logic_opcodes() {
		let mut vm = VM::new();
		vm.program = vec![
			1, 1, 0, 1, // load 1 in registry 1
			1, 2, 0, 2, // load 2 in registry 2
			8, 1, 2, 3, // r3 = r1 == r2
			9, 1, 2, 4, // r4 = r1 < r2
			10, 1, 2, 5, // r5 = r1 > r2
			11, 1, 2, 6, // r6 = r1 & r2
			12, 1, 2, 7, // r7 = r1 | r2
			13, 1, 2, 8, // r8 = r1 ^ r2
			14, 1, 9, 0, // r9 = !r1
			0, // HLT
		];

		vm.run();
		assert_eq!(
			vm.registers[0..10],
			vec![0, 1, 2, 0, 1, 0, 0, 3, 3, u32::MAX - 1]
		);

		// about the last term, !1 is designed to flip all bits except the last
		// one, which is set to 0, ergo 2**32 - 2 = (2**32 - 1) - 1 = u32::MAX -
		// 1
	}
}
