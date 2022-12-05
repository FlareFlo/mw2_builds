use std::fmt::{Display, Formatter};
use crate::build::WeaponClass;

pub struct Weapon {
	name: String,
	class: WeaponClass,
}

// Placeholder for proper weapon rendering
impl Display for Weapon {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}