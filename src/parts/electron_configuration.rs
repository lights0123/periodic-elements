use core::str::FromStr;

use alloc::borrow::Cow;
use alloc::fmt::{Display, Error, Formatter};
use alloc::string::ToString;
use alloc::vec;
use Cow::{Borrowed, Owned};

use crate::Element;
use crate::SubshellLabels;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct ElectronConfiguration<'a> {
	pub shorthand: Option<Cow<'a, str>>,
	pub electrons: Cow<'a, [SubshellLabels]>,
}

impl<'a> ElectronConfiguration<'a> {
	/// Expand noble gas shorthand
	pub fn expand(&mut self, elements: &[Element]) -> Result<&mut Self, ()> {
		if let Some(shorthand) = &self.shorthand {
			let element = elements
				.iter()
				.find(|ele| &ele.symbol == shorthand)
				.ok_or(())?;
			self.electrons.to_mut().splice(
				0..0,
				element.electron_configuration.electrons.iter().cloned(),
			);
			self.shorthand = match &element.electron_configuration.shorthand {
				None => None,
				Some(shorthand) => match shorthand {
					Borrowed(borrowed) => Some(Owned(borrowed.to_string())),
					Cow::Owned(owned) => Some(Owned(owned.clone())),
				},
			};
			if self.shorthand.is_some() {
				self.expand(elements)?;
			}
		}
		Ok(self)
	}
}

impl<'a> FromStr for ElectronConfiguration<'a> {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut shorthand = None;
		let mut electrons = vec![];
		for part in s.split_whitespace() {
			// If it is in the form [Ar]
			if part.starts_with('[') {
				// Remove the brackets
				shorthand = match part.get(1..part.len() - 1) {
					Some(str) => Some(Owned(str.to_string())),
					_ => None,
				};
				continue;
			}
			electrons.push(part.parse().map_err(|_| ())?);
		}
		Ok(Self {
			shorthand,
			electrons: Owned(electrons),
		})
	}
}

impl<'a> Display for ElectronConfiguration<'a> {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match &self.shorthand {
			Some(shorthand) => write!(f, "[{}] ", shorthand)?,
			None => {}
		}
		let mut iter = self.electrons.iter();
		if let Some(arg) = iter.next() {
			write!(f, "{}", arg)?;
			for arg in iter {
				write!(f, " {}", arg)?;
			}
		}
		Ok(())
	}
}
