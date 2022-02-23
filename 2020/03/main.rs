use std::error::Error;

use grid::{Grid, GridLine};

use crate::grid::Cell;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let lines = utils::parse_input_lines::<GridLine>()?;
	let grid = Grid::from(lines)?;
	
	let mut x = 0;
	let mut y = 0;
	let mut trees = 0;
	
	loop {
		match grid.get_at(x, y) {
			None => break,
			Some(Cell::Open) => {}
			Some(Cell::Tree) => {
				trees += 1;
			}
		}
		
		x += 3;
		y += 1;
	}
	
	println!("Total trees: {}", trees);
	
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
