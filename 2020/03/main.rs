use std::error::Error;

use grid::{Grid, GridLine};

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let lines = utils::parse_input_lines::<GridLine>()?;
	let grid = Grid::from(lines)?;
	
	println!("Total trees on slope going 3 right 1 down: {}", grid.count_trees_on_slope(3, 1));
	
	let mut product = 1u32;
	let slopes = vec![
		(1, 1),
		(3, 1),
		(5, 1),
		(7, 1),
		(1, 2)
	];
	
	for (down, right) in slopes {
		product *= grid.count_trees_on_slope(down, right);
	}
	
	println!("Product of all attempted slopes: {}", product);
	
	Ok(())
}

mod grid {
	use std::ops::Rem;
	use std::str::FromStr;
	
	use crate::utils::GenericError;
	
	pub struct Grid {
		data: Vec<GridLine>,
	}
	
	impl Grid {
		pub fn from(data: Vec<GridLine>) -> Result<Self, GenericError> {
			let expected_length = data.first().ok_or(GenericError::new("There are no lines."))?.data.len();
			
			if data.iter().any(|line| line.data.len() != expected_length) {
				return Err(GenericError::new("Every line must be the same length."));
			}
			
			Ok(Grid { data })
		}
		
		pub fn get_at(&self, x: i32, y: i32) -> Option<Cell> {
			let line = self.data.get(y as usize)?;
			Some(line.get_at(x))
		}
		
		pub fn count_trees_on_slope(&self, step_x: i32, step_y: i32) -> u32 {
			let mut x = 0;
			let mut y = 0;
			let mut trees = 0u32;
			
			loop {
				match self.get_at(x, y) {
					None => break,
					Some(Cell::Open) => {}
					Some(Cell::Tree) => {
						trees += 1;
					}
				}
				
				x += step_x;
				y += step_y;
			}
			
			return trees;
		}
	}
	
	pub struct GridLine {
		data: Vec<Cell>,
	}
	
	impl GridLine {
		fn new(data: Vec<Cell>) -> Self {
			GridLine { data }
		}
		
		fn get_at(&self, x: i32) -> Cell {
			let len = self.data.len() as i32;
			let x = x.rem(len);
			let x = if x < 0 { len + x } else { x };
			return *self.data.get(x as usize).unwrap();
		}
	}
	
	#[derive(Copy, Clone)]
	pub enum Cell {
		Open,
		Tree,
	}
	
	impl Cell {
		fn from(c: char) -> Result<Self, GenericError> {
			match c {
				'.' => Ok(Cell::Open),
				'#' => Ok(Cell::Tree),
				_ => Err(GenericError::new(format!("Invalid character: {}", c).as_str()))
			}
		}
	}
	
	impl FromStr for GridLine {
		type Err = GenericError;
		
		fn from_str(s: &str) -> Result<Self, Self::Err> {
			s.chars().map(Cell::from).collect::<Result<Vec<Cell>, GenericError>>().map(GridLine::new)
		}
	}
}
