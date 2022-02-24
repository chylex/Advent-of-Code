use std::collections::HashSet;
use std::error::Error;

use crate::utils::GenericError;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let lines = utils::read_input_lines()?;
	let groups = load_groups(&lines)?;
	
	let total_answers = groups.iter().map(|group| group.combined_answers.len()).sum::<usize>();
	println!("Total 'yes' answers among groups: {}", total_answers);
	
	Ok(())
}

fn load_groups(lines: &Vec<String>) -> Result<Vec<Group>, GenericError> {
	let mut groups = Vec::new();
	let mut group = Group::new();
	
	for line in lines {
		if line.is_empty() {
			groups.push(group);
			group = Group::new();
		} else {
			group.add_individual(&line)?;
		}
	}
	
	groups.push(group);
	Ok(groups)
}

struct Group {
	individual_answers: Vec<HashSet<char>>,
	combined_answers: HashSet<char>,
}

impl Group {
	fn new() -> Group {
		Group {
			individual_answers: Vec::new(),
			combined_answers: HashSet::new(),
		}
	}
	
	fn add_individual(&mut self, answer_str: &str) -> Result<(), GenericError> {
		let answer_set = answer_str.chars().collect::<HashSet<char>>();
		
		for answer in &answer_set {
			if answer.is_ascii_lowercase() {
				self.combined_answers.insert(answer.clone());
			} else {
				return Err(GenericError::new(format!("Invalid answer: {}", answer)));
			}
		}
		
		self.individual_answers.push(answer_set);
		Ok(())
	}
}
