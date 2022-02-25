use std::error::Error;

use crate::rules::Rule;
use crate::utils::GenericError;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let rules = utils::parse_input_lines::<Rule>()?;
	
	let count = rules::count_bags_which_contain(&rules, String::from("shiny gold"));
	println!("Amount of bags which contain a 'shiny gold' bag: {}", count);
	
	Ok(())
}

mod rules {
	use std::collections::{HashMap, HashSet};
	use std::str::FromStr;
	
	use crate::GenericError;
	
	pub struct Rule {
		bag: String,
		contains: HashMap<String, u16>,
	}
	
	pub fn count_bags_which_contain(rules: &Vec<Rule>, target_bag: String) -> usize {
		let mut containers = HashSet::new();
		let mut queue = HashSet::new();
		queue.insert(&target_bag);
		
		while !queue.is_empty() {
			for bag in queue.drain().collect::<Vec<&String>>() {
				for rule in rules {
					if rule.contains.contains_key(bag) && containers.insert(&rule.bag) {
						queue.insert(&rule.bag);
					}
				}
			}
		}
		
		return containers.len();
	}
	
	impl FromStr for Rule {
		type Err = GenericError;
		
		fn from_str(s: &str) -> Result<Self, GenericError> {
			fn validate_bag(bag: &str) -> Result<String, GenericError> {
				return if bag.chars().filter(|c| *c == ' ').count() != 1 {
					Err(GenericError::new(format!("Invalid bag name: {}", bag)))
				} else {
					Ok(bag.to_string())
				};
			}
			
			let (bag, contains_str) = s.split_once(" bags contain ").ok_or(GenericError::new("Missing ' bags contain ' delimiter."))?;
			
			let mut rule = Rule {
				bag: validate_bag(bag)?,
				contains: HashMap::new(),
			};
			
			if contains_str == "no other bags." {
				return Ok(rule);
			}
			
			fn strip<'a>(s: &'a str, suffix: &str) -> &'a str {
				s.strip_suffix(suffix).unwrap_or(s)
			}
			
			for entry_str in contains_str.split(", ") {
				let (count_str, contained_bag_str) = entry_str.split_once(' ').ok_or_else(|| GenericError::new(format!("Missing count in entry: {}", entry_str)))?;
				let count = count_str.parse::<u16>().map_err(|_| GenericError::new(format!("Cannot parse count: {}", count_str)))?;
				let contained_bag_str = strip(contained_bag_str, ".");
				let contained_bag_str = strip(contained_bag_str, " bags");
				let contained_bag_str = strip(contained_bag_str, " bag");
				rule.contains.insert(validate_bag(contained_bag_str)?, count);
			}
			
			return Ok(rule);
		}
	}
}
