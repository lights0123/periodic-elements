use alloc::fmt::{Display, Error, Formatter};
use core::str::FromStr;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mass {
	Average(f32),
	MostStable(u16),
}

impl FromStr for Mass {
	type Err = ();
	/// Expects an input such as `4.001`, `5234.1(4)`, or `[18]`.
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.starts_with('[') {
			if let Ok(stable) = s[1..s.len() - 1].parse() {
				return Ok(Mass::MostStable(stable));
			}
		} else if let Ok(average) = s[0..s.find('(').unwrap_or_else(|| s.len())].parse() {
			return Ok(Mass::Average(average));
		}
		Err(())
	}
}

impl Display for Mass {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			Mass::Average(avg) => write!(f, "{}", avg),
			Mass::MostStable(stable) => write!(f, "({})", stable),
		}
	}
}
