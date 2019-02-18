use core::str::FromStr;
use alloc::fmt::{Display, Error, Formatter};
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IonRadius {
	pub radius: f32,
	pub variation: i8,
}

impl FromStr for IonRadius {
	type Err = ();

	/// Takes an input such as `76 (+1)`
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let radius = s
			.find(' ')
			.and_then(|space_delimiter| s.get(..space_delimiter))
			.and_then(|radius| radius.parse().ok())
			.ok_or(())?;

		let variation = s
			.find('(')
			.and_then(|parenthesis| s.get(parenthesis + 1..s.len() - 1))
			.and_then(|variation| variation.parse().ok())
			.ok_or(())?;
		Ok(IonRadius { radius, variation })
	}
}

impl Display for IonRadius {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{} ({:+})", self.radius, self.variation)
	}
}
