use std::collections::{HashMap, HashSet};
use std::error::Error;

use lazy_static::lazy_static;

use crate::utils::GenericError;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let lines = utils::read_input_lines()?;
	let passports = load_passports(&lines)?;
	
	let passports = passports.into_iter().filter(|p| p.is_valid_or_from_north_pole()).collect::<Vec<Passport>>();
	println!("Valid passports with no field validation: {}", passports.len());
	println!("Valid passports with field validation: {}", passports.iter().filter(|p| p.are_field_values_valid()).count());
	
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
	
	fn is_value_valid(&self, value: &str) -> bool {
		fn as_u32(value: &str) -> Option<u32> {
			value.parse().ok()
		}
		
		fn as_u32_with_unit(value: &str, unit: &str) -> Option<u32> {
			value.strip_suffix(unit).and_then(as_u32)
		}
		
		match self {
			PassportField::BirthYear => as_u32(value).filter(|year| *year >= 1920 && *year <= 2002).is_some(),
			PassportField::IssueYear => as_u32(value).filter(|year| *year >= 2010 && *year <= 2020).is_some(),
			PassportField::ExpirationYear => as_u32(value).filter(|year| *year >= 2020 && *year <= 2030).is_some(),
			PassportField::Height => {
				if let Some(height) = as_u32_with_unit(value, "cm") {
					height >= 150 && height <= 193
				} else if let Some(height) = as_u32_with_unit(value, "in") {
					height >= 59 && height <= 76
				} else {
					false
				}
			}
			PassportField::HairColor => value.strip_prefix('#').filter(|hex| hex.chars().all(|c| c.is_digit(16))).is_some(),
			PassportField::EyeColor => VALID_EYE_COLORS.contains(value),
			PassportField::PassportId => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
			PassportField::CountryId => true
		}
	}
}

struct Passport {
	fields: HashMap<PassportField, String>,
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
	
	fn are_field_values_valid(&self) -> bool {
		return self.fields.iter().all(|(field, value)| field.is_value_valid(value.as_str()));
	}
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
	
	static ref VALID_EYE_COLORS: HashSet<&'static str> = HashSet::from([
		"amb", "blu", "brn", "gry", "grn", "hzl", "oth"
	]);
}
