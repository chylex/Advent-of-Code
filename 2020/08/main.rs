use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

use crate::utils::GenericError;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let instructions = utils::parse_input_lines::<Instruction>()?;
	
	find_infinite_loop(&instructions);
	fix_infinite_loop(&instructions);
	
	Ok(())
}

fn find_infinite_loop(instructions: &Vec<Instruction>) {
	let mut state = State::new();
	
	if state.execute_program(&instructions) == ProgramExecutionResult::InfiniteLoop {
		println!("State of accumulator before visiting instruction on line {} for the second time: {}", state.ip + 1, state.acc);
	}
}

fn fix_infinite_loop(instructions: &Vec<Instruction>) {
	let mut instructions = instructions.clone();
	
	for i in 0..instructions.len() {
		let instruction = instructions.get(i).unwrap();
		let replacement_opcode = match instruction.opcode {
			Opcode::Nop => Some(Opcode::Jmp),
			Opcode::Acc => None,
			Opcode::Jmp => Some(Opcode::Nop)
		};
		
		if let Some(replacement_opcode) = replacement_opcode {
			let mut state = State::new();
			let instruction = instruction.clone();
			
			instructions[i] = Instruction {
				opcode: replacement_opcode,
				offset: instruction.offset
			};
			
			if state.execute_program(&instructions) == ProgramExecutionResult::SuccessfulTermination {
				println!("State of accumulator after a successful termination with patched instruction on on line {}: {}", i + 1, state.acc);
				return;
			}
			
			instructions[i] = instruction;
		}
	}
	
	println!("No version of the program terminated successfully.");
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
struct Instruction {
	opcode: Opcode,
	offset: i32,
}

impl Instruction {
	fn apply(&self, state: &mut State) {
		match self.opcode {
			Opcode::Nop => {
				state.ip += 1;
			}
			
			Opcode::Acc => {
				state.acc += self.offset;
				state.ip += 1;
			}
			
			Opcode::Jmp => {
				state.ip += self.offset;
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

#[derive(Eq, PartialEq)]
enum ProgramExecutionResult {
	InvalidInstructionPointer,
	SuccessfulTermination,
	InfiniteLoop,
}

struct State {
	ip: i32,
	acc: i32,
	visited_ips: HashSet<i32>,
}

impl State {
	fn new() -> Self {
		State {
			acc: 0,
			ip: 0,
			visited_ips: HashSet::new(),
		}
	}
	
	fn execute_instruction(&mut self, instructions: &Vec<Instruction>) -> Option<ProgramExecutionResult> {
		return if !self.visited_ips.insert(self.ip) {
			Some(ProgramExecutionResult::InfiniteLoop)
		} else if self.ip < 0 {
			Some(ProgramExecutionResult::InvalidInstructionPointer)
		} else if let Some(instruction) = instructions.get(self.ip as usize) {
			instruction.apply(self);
			None
		} else {
			Some(ProgramExecutionResult::SuccessfulTermination)
		};
	}
	
	fn execute_program(&mut self, instructions: &Vec<Instruction>) -> ProgramExecutionResult {
		loop {
			if let Some(result) = self.execute_instruction(&instructions) {
				return result;
			}
		}
	}
}
