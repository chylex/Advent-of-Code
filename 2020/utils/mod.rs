#![allow(dead_code)]

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn read_input_lines() -> Result<Vec<String>, io::Error> {
	let file = File::open("input.txt")?;
	return BufReader::new(file).lines().collect();
}

pub fn parse_input_lines<T : FromStr>() -> Result<Vec<T>, Box<dyn Error>> where <T as FromStr>::Err : Into<Box<dyn Error>> {
	return read_input_lines()?.iter().map(|line| line.parse::<T>()).collect::<Result<Vec<T>, T::Err>>().map_err(Into::into);
}

pub struct GenericError {
	pub message: String
}

impl GenericError {
	pub fn new(message: impl Into<String>) -> GenericError {
		GenericError { message: message.into() }
	}
}

impl Display for GenericError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Parse error: {}", self.message)
	}
}

impl Debug for GenericError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Display::fmt(self, f)
	}
}

impl Error for GenericError {}
