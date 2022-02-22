use std::error::Error;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let lines = utils::parse_input_lines::<u32>()?;
	
	'outer: for i in 0..lines.len() {
		let value1 = lines.get(i).unwrap();
		
		for j in (i + 1)..lines.len() {
			let value2 = lines.get(j).unwrap();
			
			if value1 + value2 == 2020 {
				println!("Result: {} x {} = {}", value1, value2, value1 * value2);
				break 'outer
			}
		}
	}
	
	Ok(())
}
