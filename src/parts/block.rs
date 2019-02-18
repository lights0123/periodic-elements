use core::str::FromStr;
use alloc::fmt::{Display, Error, Formatter};
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Block {
	Nonmetal,
	NobleGas,
	AlkaliMetal,
	AlkalineEarthMetal,
	Metalloid,
	Halogen,
	Metal,
	Lanthanoid,
	Actinoid,
	TransitionMetal,
	PostTransitionMetal,
}

impl FromStr for Block {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"nonmetal" => Ok(Block::Nonmetal),
			"noble gas" => Ok(Block::NobleGas),
			"alkali metal" => Ok(Block::AlkaliMetal),
			"alkaline earth metal" => Ok(Block::AlkalineEarthMetal),
			"metalloid" => Ok(Block::Metalloid),
			"halogen" => Ok(Block::Halogen),
			"metal" => Ok(Block::Metal),
			"lanthanoid" => Ok(Block::Lanthanoid),
			"actinoid" => Ok(Block::Actinoid),
			"transition metal" => Ok(Block::TransitionMetal),
			"post-transition metal" => Ok(Block::PostTransitionMetal),
			_ => Err(()),
		}
	}
}

impl Display for Block {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", match self {
			Block::Nonmetal => "Nonmetal",
			Block::NobleGas => "Noble gas",
			Block::AlkaliMetal => "Alkali metal",
			Block::AlkalineEarthMetal => "Alkaline earth metal",
			Block::Metalloid => "Metalloid",
			Block::Halogen => "Halogen",
			Block::Metal => "Metal",
			Block::Lanthanoid => "Lanthanoid",
			Block::Actinoid => "Actinoid",
			Block::TransitionMetal => "Transition metal",
			Block::PostTransitionMetal => "Post-transition metal",
		})
	}
}
