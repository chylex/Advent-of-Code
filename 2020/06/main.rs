use std::error::Error;

use crate::group::Group;
use crate::utils::GenericError;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let lines = utils::read_input_lines()?;
	let groups = load_groups(&lines)?;
	
	let total_answers_any_members = groups.iter().map(Group::count_answers_by_any_members).sum::<usize>();
	let total_answers_all_members = groups.iter().map(Group::count_answers_by_all_members).sum::<usize>();
	println!("Total 'yes' answers, counting answers by any group member: {}", total_answers_any_members);
	println!("Total 'yes' answers, counting answers by all group members: {}", total_answers_all_members);
	
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

mod group {
	use std::collections::HashSet;
	
	use crate::GenericError;
	
	pub struct Group {
		individual_answers: Vec<HashSet<char>>,
		combined_answers: HashSet<char>,
	}
	
	impl Group {
		pub fn new() -> Group {
			Group {
				individual_answers: Vec::new(),
				combined_answers: HashSet::new(),
			}
		}
		
		pub fn add_individual(&mut self, answer_str: &str) -> Result<(), GenericError> {
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
		
		pub fn count_answers_by_any_members(&self) -> usize {
			return self.combined_answers.len();
		}
		
		pub fn count_answers_by_all_members(&self) -> usize {
			return self.combined_answers.iter().filter(|answer| self.individual_answers.iter().all(|answers| answers.contains(&answer))).count();
		}
	}
}
