use core::str::FromStr;
use alloc::fmt::{Display, Error, Formatter};
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum State {
	Solid,
	Liquid,
	Gas,
}

impl FromStr for State {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"solid" => Ok(State::Solid),
			"liquid" => Ok(State::Liquid),
			"gas" => Ok(State::Gas),
			_ => Err(()),
		}
	}
}

impl Display for State {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", match self {
			State::Solid => "Solid",
			State::Liquid => "Liquid",
			State::Gas => "Gas",
		})
	}
}
