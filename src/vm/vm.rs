use crate::vm::opcode::Opcode;

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

	/// Fetches the byte at `self.pc`, cast it into an `Opcode` and increments `self.pc` by `1`
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

	/// Main function of the `VM`
	fn run(&mut self) {
		loop {
			if self.pc >= self.program.len() {
				println!("[\x1b[93mERROR\x1b[0m] Something went wrong, VM program counter > program length");
				break
			}

			match self.decode_opcode() {
				Opcode::HLT => {
					println!("[\x1b[94mINFO\x1b[0m] HLT found at cycle {}, execution aborted", self.pc);
					break;
				},
				Opcode::LOAD => {
					println!("[\x1b[92mINFO\x1b[0m] LOAD found at cycle {}", self.pc);
					let register = self.next_8_bits() as usize;
					let number = self.next_16_bits() as u32;

					self.registers[register] = number;
				},
				Opcode::NIL => {
					println!("[\x1b[91mERROR\x1b[0m] Opcode not supported at cycle {}, execution aborted", self.pc);
					break
				}
			}
		}
	}
}

#[cfg(test)]
mod vm_tests {
    use super::*;

	#[test]
	fn test_load_opcode() {
		
		let mut vm = VM::new();
		vm.program = vec![1, 2, 0, 0x20, 0]; // should load in register 2, as it is 0-indexed, number 32, and then halt

		vm.run();
		assert_eq!(vm.registers[2], 32);
	}
}