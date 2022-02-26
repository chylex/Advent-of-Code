use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

use crate::utils::GenericError;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let instructions = utils::parse_input_lines::<Instruction>()?;
	let mut state = State::new();
	
	let mut visited_instructions = HashSet::new();
	
	loop {
		if !visited_instructions.insert(state.ip) {
			println!("State of accumulator before visiting instruction on line {} for the second time: {}", state.ip + 1, state.acc);
			break
		}
		
		if !state.execute(&instructions) {
			break
		}
	}
	
	Ok(())
}

enum Opcode {
	Nop,
	Acc,
	Jmp,
}

impl FromStr for Opcode {
	type Err = GenericError;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"nop" => Ok(Opcode::Nop),
			"acc" => Ok(Opcode::Acc),
			"jmp" => Ok(Opcode::Jmp),
			_ => Err(GenericError::new(format!("Unknown opcode: {}", s)))
		}
	}
}

struct Instruction {
	opcode: Opcode,
	offset: i32,
}

impl Instruction {
	fn apply(&self, state: &mut State) {
		match self.opcode {
			Opcode::Nop => {
				state.ip += 1
			}
			
			Opcode::Acc => {
				state.acc += self.offset;
				state.ip += 1
			}
			
			Opcode::Jmp => {
				state.ip = (state.ip as i32 + self.offset) as usize
			}
		}
	}
}

impl FromStr for Instruction {
	type Err = GenericError;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (opcode_str, offset_str) = s.split_once(' ').ok_or(GenericError::new(format!("Missing space in instruction: {}", s)))?;
		
		let opcode = opcode_str.parse::<Opcode>()?;
		let offset = offset_str.parse::<i32>().map_err(|_| GenericError::new(format!("Invalid offset: {}", offset_str)))?;
		
		Ok(Instruction { opcode, offset })
	}
}

struct State {
	ip: usize,
	acc: i32,
}

impl State {
	fn new() -> Self {
		State { acc: 0, ip: 0 }
	}
	
	fn execute(&mut self, instructions: &Vec<Instruction>) -> bool {
		if let Some(instruction) = instructions.get(self.ip) {
			instruction.apply(self);
			true
		} else {
			false
		}
	}
}
