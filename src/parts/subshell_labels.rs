use core::str::FromStr;

use alloc::fmt::{Display, Error, Formatter};

use crate::Orbital;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct SubshellLabels {
	pub shell: u8,
	pub orbital: Orbital,
	pub electron_count: u8,
}

impl Display for SubshellLabels {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}{}{}", self.shell, self.orbital, self.electron_count)
	}
}

impl FromStr for SubshellLabels {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self {
			shell: s.get(0..1).ok_or(())?.parse().map_err(|_| ())?,
			orbital: s.get(1..2).ok_or(())?.parse().map_err(|_| ())?,
			electron_count: s.get(2..).ok_or(())?.parse().map_err(|_| ())?,
		})
	}
}
