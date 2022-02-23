use std::collections::{HashMap, HashSet};
use std::error::Error;

use lazy_static::lazy_static;

use crate::utils::GenericError;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let lines = utils::read_input_lines()?;
	let passports = load_passports(&lines)?;
	
	let valid_passports = passports.iter().filter(|p| p.is_valid_or_from_north_pole()).count();
	println!("Valid passports: {}", valid_passports);
	
	Ok(())
}

fn load_passports(lines: &Vec<String>) -> Result<Vec<Passport>, GenericError> {
	let mut passports = Vec::new();
	let mut passport = Passport::new();
	
	for line in lines {
		if line.is_empty() {
			passports.push(passport);
			passport = Passport::new();
		} else {
			passport.load_fields_from_line(line.as_str())?;
		}
	}
	
	passports.push(passport);
	Ok(passports)
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum PassportField {
	BirthYear,
	IssueYear,
	ExpirationYear,
	Height,
	HairColor,
	EyeColor,
	PassportId,
	CountryId,
}

impl PassportField {
	fn from(s: &str) -> Option<PassportField> {
		match s {
			"byr" => Some(PassportField::BirthYear),
			"iyr" => Some(PassportField::IssueYear),
			"eyr" => Some(PassportField::ExpirationYear),
			"hgt" => Some(PassportField::Height),
			"hcl" => Some(PassportField::HairColor),
			"ecl" => Some(PassportField::EyeColor),
			"pid" => Some(PassportField::PassportId),
			"cid" => Some(PassportField::CountryId),
			_ => None
		}
	}
}

struct Passport {
	fields: HashMap<PassportField, String>,
}

lazy_static! {
	static ref REQUIRED_FIELDS: HashSet<PassportField> = HashSet::from([
		PassportField::BirthYear,
		PassportField::IssueYear,
		PassportField::ExpirationYear,
		PassportField::Height,
		PassportField::HairColor,
		PassportField::EyeColor,
		PassportField::PassportId
	]);
}

impl Passport {
	fn new() -> Passport {
		Passport { fields: HashMap::new() }
	}
	
	fn load_fields_from_line(&mut self, line: &str) -> Result<(), GenericError> {
		for field_entry in line.split(' ') {
			let (field_name, field_value) = field_entry.split_once(':').ok_or_else(|| GenericError::new("Passport entry is missing a colon."))?;
			let field = PassportField::from(field_name).ok_or_else(|| GenericError::new(format!("Passport field is invalid: {}", field_name)))?;
			self.fields.insert(field, field_value.to_string());
		}
		
		Ok(())
	}
	
	fn is_valid_or_from_north_pole(&self) -> bool {
		let fields = &self.fields.keys().map(|f| *f).collect::<HashSet<PassportField>>();
		return fields.is_superset(&REQUIRED_FIELDS);
	}
}
