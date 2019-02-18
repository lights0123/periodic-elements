use core::str::FromStr;
use alloc::fmt::{Display, Error, Formatter};
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Orbital {
	S = 2,
	P = 6,
	D = 10,
	F = 14,
}

impl Display for Orbital {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(
			f,
			"{}",
			match self {
				Orbital::S => "s",
				Orbital::P => "p",
				Orbital::D => "d",
				Orbital::F => "f",
			}
		)
	}
}

impl FromStr for Orbital {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lower = s.trim().to_lowercase();
		let lower_str: &str = &lower;
		match lower_str {
			"s" => Ok(Orbital::S),
			"p" => Ok(Orbital::P),
			"d" => Ok(Orbital::D),
			"f" => Ok(Orbital::F),
			_ => Err(()),
		}
	}
}
