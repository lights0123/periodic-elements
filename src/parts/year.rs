use core::str::FromStr;

use alloc::fmt::{Display, Error, Formatter};
use core::num::ParseIntError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Year {
	Ancient,
	Known(u16),
}

impl FromStr for Year {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Ancient" => Ok(Year::Ancient),
			x => {
				// Parse it as a a u32, and if that works, put it in a Year::Known. Else, carry
				// the error.
				x.parse().map(Year::Known)
			}
		}
	}
}

impl Display for Year {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			Year::Ancient => write!(f, "Ancient"),
			Year::Known(year) => write!(f, "{}", year),
		}
	}
}
