pub mod common;
pub mod vm;

use std::io::{self, Write};
use std::process::Command;

use vm::vm::VM;

#[macro_use]
pub mod utils;

fn main() {
	Command::new("clear")
		.status()
		.expect("Failed to clean terminal");

	let mut vm = VM::new();
	println!("Usage of this shitty vm:");
	println!("    .program   => Outputs the current program as a vec array");
	println!("    .registers => Outputs the current registers as a vec array");
	println!("    .quit      => Self explanatory");
	println!(
		"    _          => You write A B C D and that is the instruction you are adding and \
		 executing"
	);
	println!("");
	println!(
		"The instruction format is made of A being the opcode and the rest being opcode-specific"
	);

	loop {
		print!("===> ");
		io::stdout().flush().unwrap();

		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read input");

		match input.trim() {
			".program" => {
				debug!(format!("The current program is {:?}", vm.program));
			},
			".registers" => {
				debug!(format!(
					"The registers at PC {} are {:?}",
					vm.pc, vm.registers
				));
			},
			".quit" => {
				debug!("Quitting the vm...");
				break;
			},
			"" => {},
			_ => {
				let mut instruction: Vec<u8> = input
					.split_whitespace()
					.map(|s| s.parse().expect("Failed to parse number"))
					.collect();
				let n = instruction.len() / 4;
				vm.add_instructions(&mut instruction).unwrap_or_else(|err| {
					error!(format!(
						"Virtual machine panicked when adding instruction {:?} with error {:?}",
						instruction, err
					))
				});
				vm.run_n_steps(n).unwrap_or_else(|err| {
					error!(format!(
						"Virtual machine panicked when running instruction {:?} with error {:?}, \
						 resetting the VM...",
						instruction, err
					));
					vm.reset();
				});
			}
		}
	}
}
