
use bitvec::prelude::*;
use valuable::{Fields, StructDef, Structable, Valuable, Value};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BitString(BitVec<u8, Msb0>);

impl std::ops::Deref for BitString {
	type Target = BitVec<u8, Msb0>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for BitString {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl From<BitVec<u8, Msb0>> for BitString {
	fn from(bitvec: BitVec<u8, Msb0>) -> Self {
		Self(bitvec)
	}
}

impl From<BitString> for BitVec<u8, Msb0> {
	fn from(bitstring: BitString) -> Self {
		bitstring.0
	}
}

impl Structable for BitString {
	fn definition(&self) -> StructDef<'_> {
		StructDef::new_static("BitString", Fields::Unnamed(1))
	}
}

impl Valuable for BitString {
	fn as_value(&self) -> Value<'_> {
		Value::Structable(self)
	}
	fn visit(
		&self,
		visitor: &mut dyn ::valuable::Visit,
	) {
        let vec = self.0.clone().into_vec();
		let value = format!("0x{}", faster_hex::hex_string(&vec));
		visitor.visit_unnamed_fields(&[Valuable::as_value(&value)]);
	}
}

impl BitString {
	pub fn new() -> Self {
		Self(BitVec::new())
	}

	pub fn from_inner(bitvec: BitVec<u8, Msb0>) -> Self {
		Self(bitvec)
	}

	pub fn into_inner(self) -> BitVec<u8, Msb0> {
		self.0
	}

	pub fn inner(&self) -> &BitVec<u8, Msb0> {
		&self.0
	}

	pub fn inner_mut(&mut self) -> &mut BitVec<u8, Msb0> {
		&mut self.0
	}
}