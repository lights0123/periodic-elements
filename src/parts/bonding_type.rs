use core::str::FromStr;
use alloc::fmt::{Display, Error, Formatter};
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BondingType {
	Diatomic,
	Atomic,
	Metallic,
	CovalentNetwork,
}

impl FromStr for BondingType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"diatomic" => Ok(BondingType::Diatomic),
			"atomic" => Ok(BondingType::Atomic),
			"metallic" => Ok(BondingType::Metallic),
			"covalent network" => Ok(BondingType::CovalentNetwork),
			_ => Err(()),
		}
	}
}

impl Display for BondingType {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		write!(f, "{}", match self {
			BondingType::Diatomic => "Diatomic",
			BondingType::Atomic => "Atomic",
			BondingType::Metallic => "Metallic",
			BondingType::CovalentNetwork => "Covalent network",
		})
	}
}
