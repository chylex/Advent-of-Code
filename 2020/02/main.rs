use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

use utils::GenericError;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let lines = utils::parse_input_lines::<PasswordRule>()?;
	
	println!("Valid passwords according to part 1 rules: {}", lines.iter().filter(|x| x.is_valid_part1()).count());
	println!("Valid passwords according to part 2 rules: {}", lines.iter().filter(|x| x.is_valid_part2()).count());
	
	Ok(())
}

struct PasswordRule {
	left_value: usize,
	right_value: usize,
	required_char: String,
	actual_password: String
}

impl PasswordRule {
	fn is_valid_part1(&self) -> bool {
		let count = self.actual_password.matches(self.required_char.as_str()).count();
		return count >= self.left_value && count <= self.right_value;
	}
	
	fn is_valid_part2(&self) -> bool {
		let positions = self.actual_password.match_indices(self.required_char.as_str()).map(|(index, _)| index + 1).collect::<HashSet<usize>>();
		let is_in_first_position = positions.contains(&self.left_value);
		let is_in_second_position = positions.contains(&self.right_value);
		return (is_in_first_position && !is_in_second_position) || (!is_in_first_position && is_in_second_position);
	}
}

impl FromStr for PasswordRule {
	type Err = GenericError<'static>;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (s_range, s_rest) = s.split_once(' ').ok_or(GenericError::new("Missing space."))?;
		let (s_min, s_max) = s_range.split_once('-').ok_or(GenericError::new("Missing dash in the left part of the input."))?;
		let (s_char, s_password) = s_rest.split_once(": ").ok_or(GenericError::new("Missing colon followed by space in the right part of the input."))?;
		
		return Ok(PasswordRule {
			left_value: s_min.parse::<usize>().map_err(|_| GenericError::new("Cannot parse first number."))?,
			right_value: s_max.parse::<usize>().map_err(|_| GenericError::new("Cannot parse second number."))?,
			required_char: s_char.to_string(),
			actual_password: s_password.to_string()
		})
	}
}
