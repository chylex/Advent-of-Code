use std::collections::HashSet;
use std::error::Error;

use crate::pass::BoardingPass;
use crate::utils::GenericError;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let passes = utils::parse_input_lines::<BoardingPass>()?;
	let seat_ids = passes.iter().map(BoardingPass::get_seat_id).collect::<HashSet<u32>>();
	
	let max_seat_id = *seat_ids.iter().max().unwrap_or(&0);
	println!("Max seat ID: {}", max_seat_id);
	
	let missing_seat_ids = (0..=max_seat_id).filter(|id| !seat_ids.contains(id)).collect::<HashSet<u32>>();
	let your_seat_id_candidates = missing_seat_ids.iter()
		.map(|id| *id)
		.filter(|id| *id > 0 && *id < max_seat_id && !missing_seat_ids.contains(&(*id - 1)) && !missing_seat_ids.contains(&(*id + 1)))
		.collect::<Vec<u32>>();
	
	if your_seat_id_candidates.len() != 1 {
		Err(GenericError::new(format!("Cannot find your seat ID, too many candidates: {:?}", your_seat_id_candidates)))?;
	}
	else {
		println!("Your seat ID: {}", your_seat_id_candidates.first().unwrap());
	}
	
	Ok(())
}

mod pass {
	use std::str::FromStr;
	
	use crate::utils::GenericError;
	
	pub struct BoardingPass {
		row_directions: [bool; 7],
		col_directions: [bool; 3],
	}
	
	impl BoardingPass {
		pub fn find_seat(&self) -> (u32, u32) {
			fn combine_bits(items: &[bool]) -> u32 {
				items.iter().enumerate().filter(|(_, half)| **half).map(|(i, _)| 1u32 << (items.len() - i - 1)).reduce(|a, b| a | b).unwrap_or(0)
			}
			
			return (combine_bits(&self.row_directions), combine_bits(&self.col_directions));
		}
		
		pub fn get_seat_id(&self) -> u32 {
			let (row, col) = self.find_seat();
			return (row * (self.row_directions.len() as u32 + 1)) + col;
		}
	}
	
	impl FromStr for BoardingPass {
		type Err = GenericError;
		
		fn from_str(s: &str) -> Result<Self, Self::Err> {
			fn get_mut_at<'a>(arr: &'a mut [bool], index: usize, error_prefix: &'static str) -> Result<&'a mut bool, GenericError> {
				arr.get_mut(index).ok_or_else(|| GenericError::new(format!("{} out of bounds: {}", error_prefix, index)))
			}
			
			let mut row_directions = [false; 7];
			let mut col_directions = [false; 3];
			
			for (i, c) in s.chars().take(row_directions.len()).enumerate() {
				*get_mut_at(&mut row_directions, i, "Row direction")? = match c {
					'B' => Ok(true),
					'F' => Ok(false),
					_ => Err(GenericError::new(format!("Unknown row direction: {}", c)))
				}?;
			}
			
			for (i, c) in s.chars().skip(row_directions.len()).enumerate() {
				*get_mut_at(&mut col_directions, i, "Column direction")? = match c {
					'R' => Ok(true),
					'L' => Ok(false),
					_ => Err(GenericError::new(format!("Unknown col direction: {}", c)))
				}?;
			}
			
			Ok(BoardingPass { row_directions, col_directions })
		}
	}
}
