use std::error::Error;
use std::str::FromStr;

use utils::GenericError;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let lines = utils::parse_input_lines::<PasswordRule>()?;
	
	let valid_passwords = lines.iter().filter(|x| x.is_valid()).count();
	println!("Valid passwords: {}", valid_passwords);
	
	Ok(())
}

struct PasswordRule {
	left_value: usize,
	right_value: usize,
	required_char: String,
	actual_password: String
}

impl PasswordRule {
	fn is_valid(&self) -> bool {
		let count = self.actual_password.matches(self.required_char.as_str()).count();
		return count >= self.left_value && count <= self.right_value;
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
