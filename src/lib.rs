#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

//! # periodic-elements
//! This crate provides a list of all 118 discovered elements, as well as additional information
//! about each. You'll probably want to use the constant [`ELEMENTS`], as it provides a `'static`
//! slice of elements. Additionally, this crate uses [std::borrow::Cow] extensively, as it allows
//! for mutating element information very efficiently, while not requiring heap allocations by
//! default. Additionally, every field within [`Element`] supports [`std::fmt::Display`]
//! (with the exception of [`cpk`](Element::cpk)) and [`std::fmt::Debug`], making it easy to display.
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;


use rgb::RGB8;

use alloc::borrow::Cow;
pub use parts::exports::*;
pub use raw::*;

mod raw;
mod parts;

#[derive(Debug, Clone)]
pub struct Element<'a> {
	pub number: u16,
	pub symbol: Cow<'a, str>,
	pub name: Cow<'a, str>,
	pub mass: Mass,
	pub cpk: Option<RGB8>,
	pub electron_configuration: ElectronConfiguration<'a>,
	pub electronegativity: Option<f32>,
	pub atomic_radius: Option<u16>,
	pub ion_radius: Option<IonRadius>,
	pub van_del_walls_radius: Option<u16>,
	pub ionization_energy: Option<u16>,
	pub electron_affinity: Option<i16>,
	pub oxidation_states: Cow<'a, [i8]>,
	pub standard_state: Option<State>,
	pub bonding_type: Option<BondingType>,
	pub melting_point: Option<i16>,
	pub boiling_point: Option<i16>,
	pub density: Option<f64>,
	pub group_block: Block,
	pub discovered: Year,
}
