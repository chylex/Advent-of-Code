use std::error::Error;

#[path = "../utils/mod.rs"]
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	let lines = utils::parse_input_lines::<u32>()?;
	
	part1(&lines);
	part2(&lines);
	
	Ok(())
}

fn part1(lines: &Vec<u32>) {
	'outer: for i in 0..lines.len() {
		let value1 = lines.get(i).unwrap();
		
		for j in (i + 1)..lines.len() {
			let value2 = lines.get(j).unwrap();
			
			if value1 + value2 == 2020 {
				println!("Result: {} x {} = {}", value1, value2, value1 * value2);
				break 'outer;
			}
		}
	}
}

fn part2(lines: &Vec<u32>) {
	'outer: for i in 0..lines.len() {
		let value1 = lines.get(i).unwrap();
		
		for j in (i + 1)..lines.len() {
			let value2 = lines.get(j).unwrap();
			
			for k in (j + 1)..lines.len() {
				let value3 = lines.get(k).unwrap();
				
				if value1 + value2 + value3 == 2020 {
					println!("Result: {} x {} x {} = {}", value1, value2, value3, value1 * value2 * value3);
					break 'outer;
				}
			}
		}
	}
}
